<template>
  <main class="app">
    <header class="header">
      <div class="title-group">
        <h1>{{ t("app.title") }}</h1>
        <div class="mindmap-controls">
          <label>
            <span>{{ t("mindmap.current") }}</span>
            <select :value="editorState.mindmap.id" @change="onSwitchMindmap">
              <option v-for="mindmap in mindmaps" :key="mindmap.id" :value="mindmap.id">{{ mindmap.name }}</option>
            </select>
          </label>
          <label class="name-field">
            <span>{{ t("mindmap.name") }}</span>
            <input :value="editorState.mindmap.name" @input="onMindmapNameChange" />
          </label>
          <button type="button" @click="createNewMindmap">{{ t("mindmap.new") }}</button>
        </div>
      </div>
      <div class="actions">
        <label class="locale-switcher">
          <span>{{ t("app.language") }}</span>
          <select :value="localeState.current" @change="onLocaleChange">
            <option value="zh-TW">繁體中文</option>
            <option value="en">English</option>
          </select>
        </label>
        <button type="button" @click="handleExportMindmap">{{ t("app.exportMindmap") }}</button>
        <button type="button" @click="handleImportMindmap">{{ t("app.importMindmap") }}</button>
      </div>
    </header>

    <section class="workspace">
      <aside class="panel left">
        <TreeView
          :nodes="editorState.nodes"
          :selected-node-id="editorState.selectedNodeId"
          @add-node="handleAddNode"
          @select-node="selectNode"
        />
      </aside>

      <section class="canvas" @contextmenu.prevent="handleCanvasAdd" @dragover.prevent @drop.prevent="onDropNode">
        <GraphView
          :nodes="editorState.nodes"
          :edges="editorState.edges"
          :layouts="editorState.layouts"
          @add-edge="upsertEdge"
          @node-moved="({ id, x, y }) => updateGraphPosition(id, x, y)"
        />
      </section>

      <aside class="panel right">
        <section class="help-card">
          <h2>{{ t("app.help.title") }}</h2>
          <ol>
            <li v-for="step in helpSteps" :key="step">{{ step }}</li>
          </ol>
          <p>{{ t("app.help.tip") }}</p>
        </section>

        <TemplateLibrary
          :visible-templates="visibleTemplates"
          :categories="templateCategories"
          :platform-filter="templatePlatformFilter"
          :category-filter="templateCategoryFilter"
          :selected-node-id="editorState.selectedNodeId"
          :selected-template-id="selectedNode?.templateId ?? null"
          @update-platform-filter="onTemplatePlatformFilterUpdate"
          @update-category-filter="onTemplateCategoryFilterUpdate"
          @select-template="onTemplateSelect"
          @apply-template="applyTemplateToSelectedNode"
          @create-new-template="resetSelectedTemplate"
          @import-template-bundle="handleImportTemplateBundle"
          @export-template-bundle="handleExportTemplateBundle"
        />

        <TemplateEditor
          :template="selectedLibraryTemplate"
          @save-template="handleSaveTemplate"
          @clone-template="handleCloneTemplate"
          @delete-template="handleDeleteTemplate"
          @create-new="resetSelectedTemplate"
          @cancel-edit="resetSelectedTemplate"
        />

        <InspectorPanel
          :node="selectedNode"
          :active-path-id="editorState.mindmap.activePathId"
          :template="selectedNodeTemplate"
          @update-node="onUpdateNode"
          @set-active-path="setActivePath"
          @delete-node="handleDeleteNode"
        />

        <OutputPreview
          :content="preview.content"
          :diagnostics="combinedDiagnostics"
          :target="buildTarget"
          :output-mode="outputMode"
          :recent-results="recentBuildResults"
          @update-target="onBuildTargetUpdate"
          @update-output-mode="onOutputModeUpdate"
          @generate="handleGenerate"
        />
      </aside>
    </section>

    <footer class="footer">
      <SaveStatus :status="editorState.saveStatus" />
    </footer>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import GraphView from "./components/GraphView.vue";
import InspectorPanel from "./components/InspectorPanel.vue";
import OutputPreview from "./components/OutputPreview.vue";
import SaveStatus from "./components/SaveStatus.vue";
import TemplateEditor from "./components/TemplateEditor.vue";
import TemplateLibrary from "./components/TemplateLibrary.vue";
import TreeView from "./components/TreeView.vue";
import {
  appInitialize,
  cloneBuiltinTemplate,
  createTemplate,
  deleteUserTemplate,
  ensureSqlPluginConnection,
  exportJsonToFile,
  getMindmapDetail,
  importJsonFromFile,
  listMindmaps,
  listRecentBuildResults,
  listTemplates,
  saveBuildResult,
  saveMindmapSnapshot,
  updateTemplate,
} from "./api/runtime";
import type {
  CreateTemplateRequest,
  ListRecentBuildResultsRequest,
  MindmapSummary,
  SaveMindmapSnapshotRequest,
  UpdateTemplateRequest,
} from "./domain/contracts";
import type {
  BuildResult,
  CommandNode,
  MindmapDetail,
  OutputMode,
  PlatformKind,
  TemplateDefinition,
  ValidatorDiagnostic,
} from "./domain/models";
import { useI18n, type AppLocale } from "./i18n";
import {
  addNode,
  editorState,
  loadMindmap,
  markDirty,
  removeNode,
  selectNode,
  setActivePath,
  setRuntimeReady,
  updateGraphPosition,
  updateNode,
  upsertEdge,
} from "./stores/editor";
import { buildOutputPreview } from "./utils/outputPreview";
import {
  createMindmapTransferPackage,
  createTemplateLibraryTransferPackage,
  parseMindmapTransferPackage,
  parseTemplateLibraryTransferPackage,
  validateMindmapImportConflicts,
  validateTemplateBundleConflicts,
} from "./utils/transferFormats";
import { resolveActivePathNodes } from "./utils/validators";

const { localeState, setLocale, t } = useI18n();

const mindmaps = ref<MindmapSummary[]>([]);
const templates = ref<TemplateDefinition[]>([]);
const selectedLibraryTemplate = ref<TemplateDefinition | null>(null);
const templatePlatformFilter = ref<PlatformKind | "all">("all");
const templateCategoryFilter = ref("all");
const buildTarget = ref<PlatformKind>("linux-shell");
const outputMode = ref<OutputMode>("command");
const recentBuildResults = ref<BuildResult[]>([]);

const selectedNode = computed(
  () => editorState.nodes.find((node) => node.id === editorState.selectedNodeId) ?? null
);
const selectedNodeTemplate = computed(
  () => templates.value.find((template) => template.id === selectedNode.value?.templateId) ?? null
);
const helpSteps = computed(() => t("app.help.steps") as string[]);
const templateCategories = computed(() =>
  [...new Set(templates.value.map((template) => template.category).filter((value): value is string => Boolean(value)))].sort()
);
const visibleTemplates = computed(() =>
  templates.value.filter((template) => {
    if (templatePlatformFilter.value !== "all" && template.platformKind !== templatePlatformFilter.value) return false;
    if (templateCategoryFilter.value !== "all" && template.category !== templateCategoryFilter.value) return false;
    return true;
  })
);
const preview = computed(() =>
  buildOutputPreview({
    mindmap: editorState.mindmap,
    nodes: editorState.nodes,
    edges: editorState.edges,
    templates: templates.value,
    target: buildTarget.value,
    outputMode: outputMode.value,
  })
);
const combinedDiagnostics = computed<ValidatorDiagnostic[]>(() => {
  const diagnostics = [...editorState.diagnostics, ...preview.value.diagnostics];
  const seen = new Set<string>();
  return diagnostics.filter((diagnostic) => {
    const key = [
      diagnostic.code,
      diagnostic.level,
      diagnostic.message,
      diagnostic.nodeId ?? "",
      diagnostic.edgeId ?? "",
      diagnostic.templateId ?? "",
    ].join("::");
    if (seen.has(key)) {
      return false;
    }
    seen.add(key);
    return true;
  });
});

const createEmptyMindmap = (name = String(t("mindmap.untitled"))): MindmapDetail => ({
  mindmap: {
    id: `mm_${crypto.randomUUID()}`,
    name,
    description: "",
    rootNodeId: null,
    activePathId: null,
    currentVersion: 0,
    lastBuildResultId: null,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
  },
  nodes: [],
  edges: [],
  layouts: { tree: {}, graph: {} },
});

const currentMindmapDetail = (): MindmapDetail => ({
  mindmap: { ...editorState.mindmap },
  nodes: [...editorState.nodes],
  edges: [...editorState.edges],
  layouts: structuredClone(editorState.layouts),
});

const inferPreferredTarget = (detail: MindmapDetail): PlatformKind | null => {
  const pathNodes = resolveActivePathNodes(detail.mindmap, detail.nodes, detail.edges);
  const templateMap = new Map(templates.value.map((template) => [template.id, template]));
  const pathPlatforms = pathNodes
    .map((node) => (node.templateId ? templateMap.get(node.templateId)?.platformKind ?? null : null))
    .filter((value): value is PlatformKind => value !== null);

  if (pathPlatforms.length === 0) {
    return null;
  }

  const uniquePlatforms = [...new Set(pathPlatforms)];
  return uniquePlatforms.length === 1 ? uniquePlatforms[0] : pathPlatforms[0];
};

const syncBuildTargetForMindmap = (detail: MindmapDetail): void => {
  const preferredTarget = inferPreferredTarget(detail);
  if (preferredTarget) {
    buildTarget.value = preferredTarget;
  }
};

const toSnapshotRequest = (): SaveMindmapSnapshotRequest => ({
  mindmap: {
    id: editorState.mindmap.id,
    name: editorState.mindmap.name,
    description: editorState.mindmap.description,
    rootNodeId: editorState.mindmap.rootNodeId,
    activePathId: editorState.mindmap.activePathId,
    currentVersion: editorState.mindmap.currentVersion,
    lastBuildResultId: editorState.mindmap.lastBuildResultId,
  },
  nodes: editorState.nodes,
  edges: editorState.edges,
  layouts: editorState.layouts,
  metadata: { updatedAt: new Date().toISOString() },
});

const refreshMindmaps = async (): Promise<void> => {
  const result = await listMindmaps();
  mindmaps.value = result.items;
};

const refreshTemplates = async (): Promise<void> => {
  const result = await listTemplates({
    platformKind: null,
    category: null,
    includeBuiltIn: true,
    includeUser: true,
  });
  templates.value = result.items;
  if (selectedLibraryTemplate.value) {
    selectedLibraryTemplate.value = result.items.find((template) => template.id === selectedLibraryTemplate.value?.id) ?? null;
  }
};

const refreshRecentBuildResults = async (): Promise<void> => {
  if (!editorState.mindmap.id) {
    recentBuildResults.value = [];
    return;
  }
  const result = await listRecentBuildResults({ mindmapId: editorState.mindmap.id, limit: 5 } satisfies ListRecentBuildResultsRequest);
  recentBuildResults.value = result.items;
};

const pickInitialMindmapId = (): string | null => {
  if (mindmaps.value.length === 0) {
    return null;
  }

  const sampleMindmap = mindmaps.value.find((mindmap) => mindmap.id.startsWith("mm_sample_"));
  if (sampleMindmap) {
    return sampleMindmap.id;
  }

  const firstNonEmptyMindmap = mindmaps.value.find((mindmap) => mindmap.lastBuildResultId || mindmap.currentVersion > 1);
  return firstNonEmptyMindmap?.id ?? mindmaps.value[0].id;
};

const bootstrapMindmap = async (): Promise<void> => {
  await refreshMindmaps();
  if (mindmaps.value.length === 0) {
    loadMindmap(createEmptyMindmap());
    handleAddNode();
    markDirty();
    return;
  }
  const initialMindmapId = pickInitialMindmapId();
  if (!initialMindmapId) {
    return;
  }
  const first = await getMindmapDetail(initialMindmapId);
  loadMindmap(first);
  editorState.diagnostics = [];
  syncBuildTargetForMindmap(first);
  await refreshRecentBuildResults();
};

const handleAddNode = (): void => {
  addNode(120, 80, String(t("node.defaultTitle")));
};

const handleCanvasAdd = (): void => {
  addNode(220, 140, String(t("node.defaultTitle")));
};

const onDropNode = (event: DragEvent): void => {
  if (event.dataTransfer?.getData("application/x-node-template") !== "command-node") return;
  addNode(event.offsetX, event.offsetY, String(t("node.defaultTitle")));
};

const onUpdateNode = (payload: { id: string; patch: Partial<CommandNode> }): void => {
  updateNode(payload.id, payload.patch);
};

const handleDeleteNode = (nodeId: string): void => {
  removeNode(nodeId);
};

const onTemplatePlatformFilterUpdate = (value: PlatformKind | "all"): void => {
  templatePlatformFilter.value = value;
};

const onTemplateCategoryFilterUpdate = (value: string): void => {
  templateCategoryFilter.value = value;
};

const onTemplateSelect = (template: TemplateDefinition): void => {
  selectedLibraryTemplate.value = template;
};

const resetSelectedTemplate = (): void => {
  selectedLibraryTemplate.value = null;
};

const onBuildTargetUpdate = (target: PlatformKind): void => {
  buildTarget.value = target;
};

const onOutputModeUpdate = (mode: OutputMode): void => {
  outputMode.value = mode;
};

const applyTemplateToSelectedNode = (template: TemplateDefinition): void => {
  if (!selectedNode.value) return;
  buildTarget.value = template.platformKind;
  const existingValues = new Map(selectedNode.value.params.map((param) => [param.paramKey, param.value]));
  updateNode(selectedNode.value.id, {
    templateId: template.id,
    title: selectedNode.value.title === String(t("node.defaultTitle")) ? template.name : selectedNode.value.title,
    params: template.params.map((param) => ({
      id: param.id,
      paramKey: param.paramKey,
      paramType: param.type,
      value: existingValues.get(param.paramKey) ?? param.defaultValue ?? "",
    })),
  });
};

const handleSaveTemplate = async (
  payload: CreateTemplateRequest & { templateId?: string }
): Promise<void> => {
  if (payload.templateId) {
    await updateTemplate(payload as UpdateTemplateRequest);
  } else {
    await createTemplate(payload);
  }
  await refreshTemplates();
  selectedLibraryTemplate.value = null;
};

const handleCloneTemplate = async (payload: { templateId: string; newName: string }): Promise<void> => {
  await cloneBuiltinTemplate(payload);
  await refreshTemplates();
};

const handleDeleteTemplate = async (templateId: string): Promise<void> => {
  await deleteUserTemplate({ templateId });
  for (const node of editorState.nodes.filter((item) => item.templateId === templateId)) {
    updateNode(node.id, { templateId: null, params: [] });
  }
  await refreshTemplates();
  selectedLibraryTemplate.value = null;
};

const createImportedTemplate = async (template: TemplateDefinition): Promise<string> => {
  const response = await createTemplate({
    name: template.name,
    description: template.description,
    platformKind: template.platformKind,
    category: template.category,
    commandPattern: template.commandPattern,
    params: template.params,
  });
  return response.templateId;
};

const handleExportMindmap = async (): Promise<void> => {
  const target = await save({
    filters: [{ name: "Mindmap JSON", extensions: ["json"] }],
    defaultPath: `${editorState.mindmap.name || "mindmap"}.mindmap.json`,
  });
  if (!target) return;
  const transferPackage = createMindmapTransferPackage(currentMindmapDetail(), templates.value);
  await exportJsonToFile(target, transferPackage);
};

const handleImportMindmap = async (): Promise<void> => {
  const filePath = await open({ multiple: false, filters: [{ name: "Mindmap JSON", extensions: ["json"] }] });
  if (!filePath || Array.isArray(filePath)) return;
  const result = await importJsonFromFile(filePath);
  const transferPackage = parseMindmapTransferPackage(result.payload);
  if (!transferPackage) {
    editorState.diagnostics = [{ code: "invalid-mindmap-format", level: "error", message: "The selected file is not a command mindmap package." }];
    return;
  }

  const conflicts = validateMindmapImportConflicts(transferPackage, mindmaps.value, templates.value);
  if (conflicts.length > 0) {
    editorState.diagnostics = conflicts;
    return;
  }

  const templateIdMap = new Map<string, string>();
  for (const template of transferPackage.referencedTemplates) {
    const newId = await createImportedTemplate(template);
    templateIdMap.set(template.id, newId);
  }

  const imported = structuredClone(transferPackage.payload);
  imported.mindmap.id = `mm_${crypto.randomUUID()}`;
  imported.mindmap.name = `${imported.mindmap.name} Imported`;
  imported.mindmap.lastBuildResultId = null;
  imported.nodes = imported.nodes.map((node) => ({
    ...node,
    templateId: node.templateId ? templateIdMap.get(node.templateId) ?? node.templateId : null,
  }));
  loadMindmap(imported);
  syncBuildTargetForMindmap(imported);
  editorState.diagnostics = [];
  await refreshTemplates();
  markDirty();
};

const handleExportTemplateBundle = async (): Promise<void> => {
  const target = await save({
    filters: [{ name: "Template Bundle JSON", extensions: ["json"] }],
    defaultPath: "command-templates.bundle.json",
  });
  if (!target) return;
  const transferPackage = createTemplateLibraryTransferPackage(templates.value.filter((template) => !template.builtIn));
  await exportJsonToFile(target, transferPackage);
};

const handleImportTemplateBundle = async (): Promise<void> => {
  const filePath = await open({ multiple: false, filters: [{ name: "Template Bundle JSON", extensions: ["json"] }] });
  if (!filePath || Array.isArray(filePath)) return;
  const result = await importJsonFromFile(filePath);
  const transferPackage = parseTemplateLibraryTransferPackage(result.payload);
  if (!transferPackage) {
    editorState.diagnostics = [{ code: "invalid-template-format", level: "error", message: "The selected file is not a template library bundle." }];
    return;
  }

  const conflicts = validateTemplateBundleConflicts(transferPackage, templates.value);
  if (conflicts.length > 0) {
    editorState.diagnostics = conflicts;
    return;
  }

  for (const template of transferPackage.templates) {
    await createImportedTemplate(template);
  }
  editorState.diagnostics = [];
  await refreshTemplates();
};

const handleGenerate = async (): Promise<void> => {
  const blockingDiagnostics = preview.value.diagnostics.filter((diagnostic) => diagnostic.level === "error");
  if (blockingDiagnostics.length > 0) return;
  const result = await saveBuildResult({
    mindmapId: editorState.mindmap.id,
    target: buildTarget.value,
    outputMode: outputMode.value,
    content: preview.value.content,
  });
  editorState.mindmap.lastBuildResultId = result.buildResultId;
  markDirty();
  await refreshRecentBuildResults();
};

const createNewMindmap = (): void => {
  loadMindmap(createEmptyMindmap());
  buildTarget.value = "linux-shell";
  selectedLibraryTemplate.value = null;
  recentBuildResults.value = [];
  editorState.diagnostics = [];
  handleAddNode();
  markDirty();
};

const onSwitchMindmap = async (event: Event): Promise<void> => {
  const nextId = (event.target as HTMLSelectElement).value;
  const detail = await getMindmapDetail(nextId);
  loadMindmap(detail);
  syncBuildTargetForMindmap(detail);
  editorState.diagnostics = [];
  await refreshRecentBuildResults();
};

const onMindmapNameChange = (event: Event): void => {
  editorState.mindmap.name = (event.target as HTMLInputElement).value;
  markDirty();
};

const onLocaleChange = (event: Event): void => {
  setLocale((event.target as HTMLSelectElement).value as AppLocale);
};

let saveTimer: number | undefined;
watch(
  () => [editorState.mindmap, editorState.nodes, editorState.edges, editorState.layouts],
  () => {
    if (editorState.saveStatus !== "dirty") return;
    if (saveTimer) window.clearTimeout(saveTimer);
    saveTimer = window.setTimeout(async () => {
      editorState.saveStatus = "saving";
      try {
        const result = await saveMindmapSnapshot(toSnapshotRequest());
        editorState.mindmap.currentVersion = result.currentVersion;
        editorState.mindmap.updatedAt = result.updatedAt;
        editorState.saveStatus = "saved";
        await refreshMindmaps();
      } catch {
        editorState.saveStatus = "error";
      }
    }, 800);
  },
  { deep: true }
);

onMounted(async () => {
  await appInitialize();
  await ensureSqlPluginConnection();
  setRuntimeReady(true);
  await refreshTemplates();
  await bootstrapMindmap();
});
</script>

<style scoped>
.app {
  height: 100vh;
  display: grid;
  grid-template-rows: auto 1fr auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  border-bottom: 1px solid #1d2a3b;
  padding: 12px 16px;
}

.title-group {
  display: flex;
  flex-direction: column;
  gap: 10px;
  min-width: 0;
}

.title-group h1 {
  margin: 0;
}

.mindmap-controls,
.actions {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  flex-wrap: wrap;
}

.mindmap-controls label,
.locale-switcher {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 13px;
  color: #a5bed8;
}

.name-field {
  min-width: 220px;
}

input,
select,
button {
  border: 1px solid #2e435f;
  border-radius: 6px;
  background: #14263c;
  color: #d3e8ff;
  padding: 8px 12px;
}

.workspace {
  min-height: 0;
  display: grid;
  grid-template-columns: 240px 1fr 420px;
}

.panel {
  border-right: 1px solid #1d2a3b;
  padding: 12px;
  overflow: auto;
}

.panel.right {
  border-right: none;
  border-left: 1px solid #1d2a3b;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.canvas {
  min-height: 0;
}

.help-card {
  border: 1px solid #2e435f;
  border-radius: 12px;
  background: rgba(20, 38, 60, 0.6);
  padding: 12px;
}

.help-card h2 {
  margin: 0 0 8px;
  font-size: 15px;
}

.help-card ol {
  margin: 0;
  padding-left: 18px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  color: #d3e8ff;
  font-size: 13px;
}

.help-card p {
  margin: 10px 0 0;
  color: #a5bed8;
  font-size: 12px;
}

.footer {
  border-top: 1px solid #1d2a3b;
  padding: 10px 16px;
}
</style>

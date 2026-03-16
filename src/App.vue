<template>
  <main class="app">
    <header class="header">
      <h1>Command Mindmap Editor</h1>
      <div class="actions">
        <button type="button" @click="handleExport">Export JSON</button>
        <button type="button" @click="handleImport">Import JSON</button>
      </div>
    </header>

    <section class="workspace">
      <aside class="panel left">
        <TreeView
          :nodes="editorState.nodes"
          :selected-node-id="editorState.selectedNodeId"
          @add-node="addNode()"
          @select-node="selectNode"
        />
      </aside>

      <section
        class="canvas"
        @contextmenu.prevent="addNode(220, 140)"
        @dragover.prevent
        @drop.prevent="onDropNode"
      >
        <GraphView
          :nodes="editorState.nodes"
          :edges="editorState.edges"
          :layouts="editorState.layouts"
          @add-edge="upsertEdge"
          @node-moved="({ id, x, y }) => updateGraphPosition(id, x, y)"
        />
      </section>

      <aside class="panel right">
        <InspectorPanel
          :node="selectedNode"
          :active-path-id="editorState.mindmap.activePathId"
          @update-node="onUpdateNode"
          @set-active-path="setActivePath"
        />
      </aside>
    </section>

    <footer class="footer">
      <SaveStatus :status="editorState.saveStatus" />
    </footer>
  </main>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from "vue";
import { open, save } from "@tauri-apps/plugin-dialog";
import GraphView from "./components/GraphView.vue";
import InspectorPanel from "./components/InspectorPanel.vue";
import SaveStatus from "./components/SaveStatus.vue";
import TreeView from "./components/TreeView.vue";
import {
  appInitialize,
  ensureSqlPluginConnection,
  exportJsonToFile,
  getMindmapDetail,
  importJsonFromFile,
  listMindmaps,
  saveMindmapSnapshot
} from "./api/runtime";
import {
  addNode,
  editorState,
  loadMindmap,
  markDirty,
  selectNode,
  setActivePath,
  setRuntimeReady,
  updateGraphPosition,
  updateNode,
  upsertEdge
} from "./stores/editor";
import type { SaveMindmapSnapshotRequest } from "./domain/contracts";
import type { CommandNode } from "./domain/models";

const selectedNode = computed(
  () => editorState.nodes.find((node) => node.id === editorState.selectedNodeId) ?? null
);

const toSnapshotRequest = (): SaveMindmapSnapshotRequest => ({
  mindmap: {
    id: editorState.mindmap.id,
    name: editorState.mindmap.name,
    description: editorState.mindmap.description,
    rootNodeId: editorState.mindmap.rootNodeId,
    activePathId: editorState.mindmap.activePathId,
    currentVersion: editorState.mindmap.currentVersion
  },
  nodes: editorState.nodes,
  edges: editorState.edges,
  layouts: editorState.layouts,
  metadata: {
    updatedAt: new Date().toISOString()
  }
});

const onUpdateNode = (payload: { id: string; patch: Partial<CommandNode> }): void => {
  updateNode(payload.id, payload.patch);
};

const onDropNode = (event: DragEvent): void => {
  if (event.dataTransfer?.getData("application/x-node-template") !== "command-node") return;
  addNode(event.offsetX, event.offsetY);
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
      } catch {
        editorState.saveStatus = "error";
      }
    }, 800);
  },
  { deep: true }
);

const bootstrapMindmap = async (): Promise<void> => {
  const list = await listMindmaps();
  if (list.items.length === 0) {
    addNode(180, 120);
    markDirty();
    return;
  }
  const first = await getMindmapDetail(list.items[0].id);
  loadMindmap(first);
};

const handleExport = async (): Promise<void> => {
  const target = await save({
    filters: [{ name: "JSON", extensions: ["json"] }],
    defaultPath: `${editorState.mindmap.name || "mindmap"}.json`
  });
  if (!target) return;
  await exportJsonToFile(target, toSnapshotRequest());
};

const handleImport = async (): Promise<void> => {
  const filePath = await open({
    multiple: false,
    filters: [{ name: "JSON", extensions: ["json"] }]
  });
  if (!filePath || Array.isArray(filePath)) return;
  const result = await importJsonFromFile(filePath);
  const payload = result.payload as SaveMindmapSnapshotRequest;
  loadMindmap({
    mindmap: {
      id: payload.mindmap.id,
      name: payload.mindmap.name,
      description: payload.mindmap.description,
      rootNodeId: payload.mindmap.rootNodeId,
      activePathId: payload.mindmap.activePathId,
      currentVersion: payload.mindmap.currentVersion,
      lastBuildResultId: null,
      createdAt: new Date().toISOString(),
      updatedAt: payload.metadata.updatedAt
    },
    nodes: payload.nodes,
    edges: payload.edges,
    layouts: payload.layouts
  });
  markDirty();
};

onMounted(async () => {
  await appInitialize();
  await ensureSqlPluginConnection();
  setRuntimeReady(true);
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
  align-items: center;
  border-bottom: 1px solid #1d2a3b;
  padding: 12px 16px;
}

.actions {
  display: flex;
  gap: 8px;
}

button {
  border: 1px solid #2e435f;
  border-radius: 6px;
  background: #14263c;
  color: #d3e8ff;
  padding: 8px 12px;
  cursor: pointer;
}

.workspace {
  min-height: 0;
  display: grid;
  grid-template-columns: 240px 1fr 300px;
}

.panel {
  border-right: 1px solid #1d2a3b;
  padding: 12px;
  overflow: auto;
}

.panel.right {
  border-right: none;
  border-left: 1px solid #1d2a3b;
}

.canvas {
  min-height: 0;
}

.footer {
  border-top: 1px solid #1d2a3b;
  padding: 10px 16px;
}
</style>

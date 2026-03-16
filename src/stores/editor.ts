import { reactive } from "vue";
import type {
  CommandEdge,
  CommandNode,
  LayoutState,
  MindmapDetail,
  MindmapMetadata,
  SaveStatus,
  ValidatorDiagnostic
} from "../domain/models";

interface EditorState {
  runtimeReady: boolean;
  mindmap: MindmapMetadata;
  nodes: CommandNode[];
  edges: CommandEdge[];
  layouts: LayoutState;
  selectedNodeId: string | null;
  saveStatus: SaveStatus;
  diagnostics: ValidatorDiagnostic[];
}

const nowIso = (): string => new Date().toISOString();

const emptyMindmap = (): MindmapMetadata => ({
  id: "mm_local_default",
  name: "New Mindmap",
  description: "",
  rootNodeId: null,
  activePathId: null,
  currentVersion: 1,
  lastBuildResultId: null,
  createdAt: nowIso(),
  updatedAt: nowIso()
});

export const editorState = reactive<EditorState>({
  runtimeReady: false,
  mindmap: emptyMindmap(),
  nodes: [],
  edges: [],
  layouts: { tree: {}, graph: {} },
  selectedNodeId: null,
  saveStatus: "idle",
  diagnostics: []
});

const randomId = (prefix: string): string => `${prefix}_${Math.random().toString(36).slice(2, 10)}`;

export const setRuntimeReady = (ready: boolean): void => {
  editorState.runtimeReady = ready;
};

export const markDirty = (): void => {
  editorState.mindmap.updatedAt = nowIso();
  editorState.saveStatus = "dirty";
};

export const loadMindmap = (detail: MindmapDetail): void => {
  editorState.mindmap = detail.mindmap;
  editorState.nodes = detail.nodes;
  editorState.edges = detail.edges;
  editorState.layouts = detail.layouts;
  editorState.selectedNodeId = detail.nodes[0]?.id ?? null;
  editorState.saveStatus = "saved";
};

export const addNode = (x = 120, y = 80, title = "New Command Node"): void => {
  const id = randomId("node");
  const node: CommandNode = {
    id,
    templateId: null,
    title,
    notes: "",
    includeInOutput: true,
    orderOverride: null,
    params: [],
    createdAt: nowIso(),
    updatedAt: nowIso()
  };
  editorState.nodes.push(node);
  editorState.layouts.graph[id] = { x, y };
  editorState.layouts.tree[id] = { x: 0, y: editorState.nodes.length * 48 };
  editorState.selectedNodeId = id;
  if (!editorState.mindmap.rootNodeId) {
    editorState.mindmap.rootNodeId = id;
  }
  if (!editorState.mindmap.activePathId) {
    editorState.mindmap.activePathId = id;
  }
  markDirty();
};

export const updateNode = (id: string, patch: Partial<CommandNode>): void => {
  const target = editorState.nodes.find((node) => node.id === id);
  if (!target) return;
  Object.assign(target, patch, { updatedAt: nowIso() });
  markDirty();
};

export const removeNode = (id: string): void => {
  const nextNodes = editorState.nodes.filter((node) => node.id !== id);
  if (nextNodes.length === editorState.nodes.length) return;

  editorState.nodes = nextNodes;
  editorState.edges = editorState.edges.filter(
    (edge) => edge.sourceNodeId !== id && edge.targetNodeId !== id
  );
  delete editorState.layouts.graph[id];
  delete editorState.layouts.tree[id];

  if (editorState.mindmap.rootNodeId === id) {
    editorState.mindmap.rootNodeId = nextNodes[0]?.id ?? null;
  }
  if (editorState.mindmap.activePathId === id) {
    editorState.mindmap.activePathId = nextNodes[0]?.id ?? null;
  }
  if (editorState.selectedNodeId === id) {
    editorState.selectedNodeId = nextNodes[0]?.id ?? null;
  }

  markDirty();
};

export const setActivePath = (nodeId: string | null): void => {
  editorState.mindmap.activePathId = nodeId;
  markDirty();
};

export const selectNode = (id: string | null): void => {
  editorState.selectedNodeId = id;
};

export const updateGraphPosition = (id: string, x: number, y: number): void => {
  editorState.layouts.graph[id] = { x, y };
  markDirty();
};

export const upsertEdge = (edge: CommandEdge): void => {
  const idx = editorState.edges.findIndex((it) => it.id === edge.id);
  if (idx >= 0) {
    editorState.edges[idx] = edge;
  } else {
    editorState.edges.push(edge);
  }
  markDirty();
};

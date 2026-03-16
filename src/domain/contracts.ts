import type { LayoutState, MindmapDetail, PlatformKind, TemplateDefinition } from "./models";

export const tauriCommands = {
  appInitialize: "app_initialize",
  getStorageStatus: "get_storage_status",
  listMindmaps: "list_mindmaps",
  getMindmapDetail: "get_mindmap_detail",
  saveMindmapSnapshot: "save_mindmap_snapshot",
  listTemplates: "list_templates",
  cloneBuiltinTemplate: "clone_builtin_template",
  exportJsonToFile: "export_json_to_file",
  importJsonFromFile: "import_json_from_file"
} as const;

export interface AppInitializeResponse {
  appVersion: string;
  dbReady: boolean;
  dataDir: string;
  schemaVersion: number;
}

export interface StorageStatusResponse {
  dbReady: boolean;
  lastError: string | null;
}

export interface MindmapSummary {
  id: string;
  name: string;
  updatedAt: string;
  currentVersion: number;
  lastBuildResultId: string | null;
}

export interface ListMindmapsResponse {
  items: MindmapSummary[];
}

export interface SaveMindmapSnapshotRequest {
  mindmap: {
    id: string;
    name: string;
    description?: string;
    rootNodeId: string | null;
    activePathId: string | null;
    currentVersion: number;
  };
  nodes: MindmapDetail["nodes"];
  edges: MindmapDetail["edges"];
  layouts: LayoutState;
  metadata: {
    updatedAt: string;
  };
}

export interface SaveMindmapSnapshotResponse {
  ok: boolean;
  mindmapId: string;
  currentVersion: number;
  updatedAt: string;
}

export interface ListTemplatesRequest {
  platformKind: PlatformKind | null;
  category: string | null;
  includeBuiltIn: boolean;
  includeUser: boolean;
}

export interface ListTemplatesResponse {
  items: TemplateDefinition[];
}

export interface CloneBuiltinTemplateRequest {
  templateId: string;
  newName: string;
}

export interface JsonImportResponse {
  fileName: string;
  payload: unknown;
}

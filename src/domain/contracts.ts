import type { BuildResult, LayoutState, MindmapDetail, OutputMode, PlatformKind, TemplateDefinition } from "./models";

export const tauriCommands = {
  appInitialize: "app_initialize",
  getStorageStatus: "get_storage_status",
  listMindmaps: "list_mindmaps",
  getMindmapDetail: "get_mindmap_detail",
  saveMindmapSnapshot: "save_mindmap_snapshot",
  listTemplates: "list_templates",
  createTemplate: "create_template",
  updateTemplate: "update_template",
  cloneBuiltinTemplate: "clone_builtin_template",
  deleteUserTemplate: "delete_user_template",
  saveBuildResult: "save_build_result",
  listRecentBuildResults: "list_recent_build_results",
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
    lastBuildResultId: string | null;
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

export interface CreateTemplateRequest {
  name: string;
  description: string;
  platformKind: PlatformKind;
  category: string | null;
  commandPattern: string;
  params: TemplateDefinition["params"];
}

export interface UpdateTemplateRequest extends CreateTemplateRequest {
  templateId: string;
}

export interface TemplateMutationResponse {
  templateId: string;
  updatedAt: string;
}

export interface DeleteUserTemplateRequest {
  templateId: string;
}

export interface SaveBuildResultRequest {
  mindmapId: string;
  target: PlatformKind;
  outputMode: OutputMode;
  content: string;
}

export interface SaveBuildResultResponse {
  buildResultId: string;
  createdAt: string;
}

export interface ListRecentBuildResultsRequest {
  mindmapId: string;
  limit?: number;
}

export interface ListRecentBuildResultsResponse {
  items: BuildResult[];
}

export interface JsonImportResponse {
  fileName: string;
  payload: unknown;
}

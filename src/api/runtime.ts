import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";
import type {
  AppInitializeResponse,
  CloneBuiltinTemplateRequest,
  JsonImportResponse,
  ListMindmapsResponse,
  ListTemplatesRequest,
  ListTemplatesResponse,
  SaveMindmapSnapshotRequest,
  SaveMindmapSnapshotResponse,
  StorageStatusResponse
} from "../domain/contracts";
import { tauriCommands } from "../domain/contracts";
import type { MindmapDetail } from "../domain/models";

// Ensures we wire a maintained Tauri SQLite package from day one.
export const ensureSqlPluginConnection = async (): Promise<void> => {
  const db = await Database.load("sqlite:command_mindmaps.db");
  await db.execute("SELECT 1");
};

export const appInitialize = async (): Promise<AppInitializeResponse> =>
  invoke<AppInitializeResponse>(tauriCommands.appInitialize);

export const getStorageStatus = async (): Promise<StorageStatusResponse> =>
  invoke<StorageStatusResponse>(tauriCommands.getStorageStatus);

export const listMindmaps = async (): Promise<ListMindmapsResponse> =>
  invoke<ListMindmapsResponse>(tauriCommands.listMindmaps);

export const getMindmapDetail = async (mindmapId: string): Promise<MindmapDetail> =>
  invoke<MindmapDetail>(tauriCommands.getMindmapDetail, { mindmapId });

export const saveMindmapSnapshot = async (
  request: SaveMindmapSnapshotRequest
): Promise<SaveMindmapSnapshotResponse> =>
  invoke<SaveMindmapSnapshotResponse>(tauriCommands.saveMindmapSnapshot, { request });

export const listTemplates = async (request: ListTemplatesRequest): Promise<ListTemplatesResponse> =>
  invoke<ListTemplatesResponse>(tauriCommands.listTemplates, { request });

export const cloneBuiltinTemplate = async (request: CloneBuiltinTemplateRequest): Promise<void> =>
  invoke<void>(tauriCommands.cloneBuiltinTemplate, { request });

export const exportJsonToFile = async (path: string, payload: unknown): Promise<void> =>
  invoke<void>(tauriCommands.exportJsonToFile, { path, payload });

export const importJsonFromFile = async (path: string): Promise<JsonImportResponse> =>
  invoke<JsonImportResponse>(tauriCommands.importJsonFromFile, { path });

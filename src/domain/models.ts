export type PlatformKind = "linux-shell" | "wsl" | "windows-powershell";
export type OutputMode = "command" | "script";
export type SaveStatus = "idle" | "dirty" | "saving" | "saved" | "error";

export interface MindmapMetadata {
  id: string;
  name: string;
  description?: string;
  rootNodeId: string | null;
  activePathId: string | null;
  currentVersion: number;
  lastBuildResultId: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface NodeParamValue {
  id: string;
  paramKey: string;
  paramType: "text" | "boolean" | "single-select" | "path";
  value: string;
}

export interface CommandNode {
  id: string;
  templateId: string | null;
  title: string;
  notes: string;
  includeInOutput: boolean;
  orderOverride: number | null;
  params: NodeParamValue[];
  createdAt: string;
  updatedAt: string;
}

export interface CommandEdge {
  id: string;
  sourceNodeId: string;
  targetNodeId: string;
  edgeType: "flow" | "dependency";
  priority: number | null;
  enabled: boolean;
}

export interface LayoutPoint {
  x: number;
  y: number;
}

export interface LayoutState {
  tree: Record<string, LayoutPoint>;
  graph: Record<string, LayoutPoint>;
}

export interface TemplateParamDefinition {
  id: string;
  paramKey: string;
  label: string;
  type: "text" | "boolean" | "single-select" | "path";
  required: boolean;
  defaultValue: string | null;
  options: string[];
}

export interface TemplateDefinition {
  id: string;
  name: string;
  description: string;
  platformKind: PlatformKind;
  category: string | null;
  builtIn: boolean;
  commandPattern: string;
  params: TemplateParamDefinition[];
  createdAt: string;
  updatedAt: string;
}

export interface BuildResult {
  id: string;
  mindmapId: string;
  mindmapVersion: number;
  target: PlatformKind;
  outputMode: OutputMode;
  content: string;
  createdAt: string;
}

export interface ValidatorDiagnostic {
  code: string;
  level: "error" | "warning" | "info";
  message: string;
  nodeId?: string;
  edgeId?: string;
  templateId?: string;
}

export interface MindmapDetail {
  mindmap: MindmapMetadata;
  nodes: CommandNode[];
  edges: CommandEdge[];
  layouts: LayoutState;
}

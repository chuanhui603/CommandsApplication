import { describe, expect, it } from "vitest";
import type { CommandEdge, CommandNode, MindmapMetadata, TemplateDefinition } from "../domain/models";
import { buildOutputPreview } from "./outputPreview";

const mindmap: MindmapMetadata = {
  id: "mm_1",
  name: "Preview",
  description: "",
  rootNodeId: "node-root",
  activePathId: "node-leaf",
  currentVersion: 7,
  lastBuildResultId: null,
  createdAt: "2026-03-16T00:00:00Z",
  updatedAt: "2026-03-16T00:00:00Z",
};

const nodes: CommandNode[] = [
  {
    id: "node-root",
    templateId: "tpl_ls",
    title: "Root",
    notes: "",
    includeInOutput: true,
    orderOverride: null,
    params: [{ id: "p1", paramKey: "path", paramType: "path", value: "src" }],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
  {
    id: "node-leaf",
    templateId: "tpl_rg",
    title: "Leaf",
    notes: "",
    includeInOutput: true,
    orderOverride: null,
    params: [{ id: "p2", paramKey: "pattern", paramType: "text", value: "TODO" }],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
  {
    id: "node-side",
    templateId: "tpl_side",
    title: "Side branch",
    notes: "",
    includeInOutput: true,
    orderOverride: null,
    params: [],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
];

const edges: CommandEdge[] = [
  {
    id: "edge_root_leaf",
    sourceNodeId: "node-root",
    targetNodeId: "node-leaf",
    edgeType: "flow",
    priority: null,
    enabled: true,
  },
  {
    id: "edge_root_side",
    sourceNodeId: "node-root",
    targetNodeId: "node-side",
    edgeType: "flow",
    priority: null,
    enabled: true,
  },
];

const templates: TemplateDefinition[] = [
  {
    id: "tpl_ls",
    name: "List files",
    description: "",
    platformKind: "linux-shell",
    category: "files",
    builtIn: true,
    commandPattern: "ls -la {{path}}",
    params: [{ id: "tp1", paramKey: "path", label: "Path", type: "path", required: false, defaultValue: ".", options: [] }],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
  {
    id: "tpl_rg",
    name: "Search text",
    description: "",
    platformKind: "linux-shell",
    category: "search",
    builtIn: true,
    commandPattern: "rg {{pattern}} {{path}}",
    params: [
      { id: "tp2", paramKey: "pattern", label: "Pattern", type: "text", required: true, defaultValue: null, options: [] },
      { id: "tp3", paramKey: "path", label: "Path", type: "path", required: false, defaultValue: ".", options: [] },
    ],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
  {
    id: "tpl_side",
    name: "Side",
    description: "",
    platformKind: "linux-shell",
    category: "misc",
    builtIn: false,
    commandPattern: "echo ignored",
    params: [],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
];

describe("output preview", () => {
  it("builds a single command chain from the active path only", () => {
    const preview = buildOutputPreview({
      mindmap,
      nodes,
      edges,
      templates,
      target: "linux-shell",
      outputMode: "command",
    });

    expect(preview.diagnostics).toHaveLength(0);
    expect(preview.steps).toEqual(["ls -la src", "rg TODO ."]);
    expect(preview.content).toBe("ls -la src &&\nrg TODO .");
  });

  it("returns diagnostics instead of script output when target compatibility fails", () => {
    const preview = buildOutputPreview({
      mindmap,
      nodes,
      edges,
      templates,
      target: "windows-powershell",
      outputMode: "script",
    });

    expect(preview.content).toBe("");
    expect(preview.diagnostics.map((item) => item.code)).toContain("target-platform-mismatch");
  });
});
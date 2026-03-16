import { describe, expect, it } from "vitest";
import type { CommandEdge, CommandNode, MindmapMetadata, TemplateDefinition } from "../domain/models";
import { resolveActivePathNodes, validateGeneration } from "./validators";

const baseMindmap = (overrides: Partial<MindmapMetadata> = {}): MindmapMetadata => ({
  id: "mm_1",
  name: "Test Mindmap",
  description: "",
  rootNodeId: "node-root",
  activePathId: "node-leaf",
  currentVersion: 3,
  lastBuildResultId: null,
  createdAt: "2026-03-16T00:00:00Z",
  updatedAt: "2026-03-16T00:00:00Z",
  ...overrides,
});

const baseNodes = (): CommandNode[] => [
  {
    id: "node-root",
    templateId: "tpl_ls",
    title: "Root",
    notes: "",
    includeInOutput: true,
    orderOverride: null,
    params: [{ id: "p1", paramKey: "path", paramType: "path", value: "." }],
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

const baseEdges = (): CommandEdge[] => [
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

const baseTemplates = (): TemplateDefinition[] => [
  {
    id: "tpl_ls",
    name: "List files",
    description: "",
    platformKind: "linux-shell",
    category: "files-and-directories",
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
    category: "search-and-filtering",
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
    name: "Side branch",
    description: "",
    platformKind: "linux-shell",
    category: "misc",
    builtIn: false,
    commandPattern: "echo side",
    params: [],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
];

describe("validators", () => {
  it("resolves only the active path nodes in root-to-leaf order", () => {
    const path = resolveActivePathNodes(baseMindmap(), baseNodes(), baseEdges());
    expect(path.map((node) => node.id)).toEqual(["node-root", "node-leaf"]);
  });

  it("reports a cycle in the active path", () => {
    const diagnostics = validateGeneration({
      mindmap: baseMindmap({ rootNodeId: "node-missing" }),
      nodes: baseNodes(),
      edges: [
        {
          id: "edge_root_leaf",
          sourceNodeId: "node-root",
          targetNodeId: "node-leaf",
          edgeType: "flow",
          priority: null,
          enabled: true,
        },
        {
          id: "edge_leaf_root",
          sourceNodeId: "node-leaf",
          targetNodeId: "node-root",
          edgeType: "flow",
          priority: null,
          enabled: true,
        },
      ],
      templates: baseTemplates(),
      target: "linux-shell",
    });

    expect(diagnostics.map((item) => item.code)).toContain("cycle-detected");
  });

  it("reports missing required parameters and target mismatches", () => {
    const nodes = baseNodes();
    nodes[1].params = [];
    const templates = baseTemplates();
    templates[1].platformKind = "windows-powershell";

    const diagnostics = validateGeneration({
      mindmap: baseMindmap(),
      nodes,
      edges: baseEdges(),
      templates,
      target: "linux-shell",
    });

    expect(diagnostics.map((item) => item.code)).toEqual(
      expect.arrayContaining(["missing-required-parameter", "target-platform-mismatch"])
    );
  });
});
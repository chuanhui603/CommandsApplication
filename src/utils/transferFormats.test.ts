import { describe, expect, it } from "vitest";
import type { MindmapDetail, TemplateDefinition } from "../domain/models";
import type { MindmapSummary } from "../domain/contracts";
import {
  createMindmapTransferPackage,
  createTemplateLibraryTransferPackage,
  parseMindmapTransferPackage,
  parseTemplateLibraryTransferPackage,
  validateMindmapImportConflicts,
  validateTemplateBundleConflicts,
} from "./transferFormats";

const templates: TemplateDefinition[] = [
  {
    id: "tpl_user_ai_copilot",
    name: "GitHub Copilot Prompt",
    description: "AI helper template",
    platformKind: "linux-shell",
    category: "ai",
    builtIn: false,
    commandPattern: "gh copilot suggest -t shell '{{prompt}}'",
    params: [{ id: "p1", paramKey: "prompt", label: "Prompt", type: "text", required: true, defaultValue: null, options: [] }],
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
];

const detail: MindmapDetail = {
  mindmap: {
    id: "mm_ai",
    name: "AI commands",
    description: "",
    rootNodeId: "node-ai",
    activePathId: "node-ai",
    currentVersion: 2,
    lastBuildResultId: null,
    createdAt: "2026-03-16T00:00:00Z",
    updatedAt: "2026-03-16T00:00:00Z",
  },
  nodes: [
    {
      id: "node-ai",
      templateId: "tpl_user_ai_copilot",
      title: "Ask Copilot",
      notes: "",
      includeInOutput: true,
      orderOverride: null,
      params: [{ id: "np1", paramKey: "prompt", paramType: "text", value: "find slow tests" }],
      createdAt: "2026-03-16T00:00:00Z",
      updatedAt: "2026-03-16T00:00:00Z",
    },
  ],
  edges: [],
  layouts: { tree: {}, graph: {} },
};

describe("transfer formats", () => {
  it("creates and parses a mindmap package with referenced templates", () => {
    const transferPackage = createMindmapTransferPackage(detail, templates);

    expect(transferPackage.referencedTemplates).toHaveLength(1);
    expect(parseMindmapTransferPackage(transferPackage)).toEqual(transferPackage);
  });

  it("creates and parses a template bundle", () => {
    const bundle = createTemplateLibraryTransferPackage(templates);
    expect(parseTemplateLibraryTransferPackage(bundle)).toEqual(bundle);
  });

  it("blocks conflicting mindmap and template imports", () => {
    const existingMindmaps: MindmapSummary[] = [
      {
        id: "mm_existing",
        name: "AI commands",
        updatedAt: "2026-03-16T00:00:00Z",
        currentVersion: 4,
        lastBuildResultId: null,
      },
    ];

    const mindmapConflicts = validateMindmapImportConflicts(
      createMindmapTransferPackage(detail, templates),
      existingMindmaps,
      templates
    );
    const templateConflicts = validateTemplateBundleConflicts(
      createTemplateLibraryTransferPackage(templates),
      templates
    );

    expect(mindmapConflicts.map((item) => item.code)).toContain("mindmap-import-conflict");
    expect(templateConflicts.map((item) => item.code)).toContain("template-bundle-conflict");
  });
});
import type {
  CommandEdge,
  CommandNode,
  MindmapMetadata,
  OutputMode,
  PlatformKind,
  TemplateDefinition,
  ValidatorDiagnostic,
} from "../domain/models";
import { resolveActivePathNodes, validateGeneration } from "./validators";

interface PreviewInput {
  mindmap: MindmapMetadata;
  nodes: CommandNode[];
  edges: CommandEdge[];
  templates: TemplateDefinition[];
  target: PlatformKind;
  outputMode: OutputMode;
}

export interface PreviewResult {
  content: string;
  steps: string[];
  diagnostics: ValidatorDiagnostic[];
}

const applyParams = (pattern: string, node: CommandNode, template: TemplateDefinition): string => {
  const nodeValues = new Map(node.params.map((param) => [param.paramKey, param.value]));
  return pattern.replace(/\{\{\s*([\w.-]+)\s*\}\}/g, (_, key: string) => {
    const value = nodeValues.get(key);
    if (value && value.length > 0) return value;
    const fallback = template.params.find((param) => param.paramKey === key)?.defaultValue;
    return fallback && fallback.length > 0 ? fallback : `<${key}>`;
  });
};

const renderScript = (target: PlatformKind, steps: string[]): string => {
  if (target === "windows-powershell") {
    return ["$ErrorActionPreference = 'Stop'", ...steps].join("\n");
  }

  const body = steps.join("\n");
  return ["#!/usr/bin/env bash", "set -e", body].join("\n");
};

export const buildOutputPreview = ({
  mindmap,
  nodes,
  edges,
  templates,
  target,
  outputMode,
}: PreviewInput): PreviewResult => {
  const diagnostics = validateGeneration({
    mindmap,
    nodes,
    edges,
    templates,
    target,
  });
  if (diagnostics.some((diagnostic) => diagnostic.level === "error")) {
    return {
      content: "",
      steps: [],
      diagnostics,
    };
  }

  const pathNodes = resolveActivePathNodes(mindmap, nodes, edges).filter((node) => node.includeInOutput);
  const templatesById = new Map(templates.map((template) => [template.id, template]));
  const steps = pathNodes.flatMap((node) => {
    if (!node.templateId) return [];
    const template = templatesById.get(node.templateId);
    if (!template) return [];
    return [applyParams(template.commandPattern, node, template)];
  });

  return {
    content: outputMode === "script" ? renderScript(target, steps) : steps.join(" &&\n"),
    steps,
    diagnostics,
  };
};

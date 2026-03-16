import type {
  CommandEdge,
  CommandNode,
  MindmapMetadata,
  PlatformKind,
  TemplateDefinition,
  ValidatorDiagnostic,
} from "../domain/models";

interface GenerationValidationInput {
  mindmap: MindmapMetadata;
  nodes: CommandNode[];
  edges: CommandEdge[];
  templates: TemplateDefinition[];
  target: PlatformKind;
}

interface ActivePathResolution {
  nodes: CommandNode[];
  hasCycle: boolean;
  reachedRoot: boolean;
}

const resolveActivePath = (
  mindmap: MindmapMetadata,
  nodes: CommandNode[],
  edges: CommandEdge[]
): ActivePathResolution => {
  if (!mindmap.activePathId) {
    return {
      nodes: [],
      hasCycle: false,
      reachedRoot: false,
    };
  }

  const nodesById = new Map(nodes.map((node) => [node.id, node]));
  const result: CommandNode[] = [];
  const seen = new Set<string>();
  let currentId: string | null = mindmap.activePathId;
  let hasCycle = false;
  let reachedRoot = false;

  while (currentId) {
    if (seen.has(currentId)) {
      hasCycle = true;
      break;
    }
    seen.add(currentId);

    const currentNode = nodesById.get(currentId);
    if (!currentNode) break;
    result.push(currentNode);

    if (currentId === mindmap.rootNodeId) {
      reachedRoot = true;
      break;
    }

    const incoming = edges.find((edge) => edge.enabled && edge.targetNodeId === currentId);
    currentId = incoming?.sourceNodeId ?? null;
  }

  return {
    nodes: result.reverse(),
    hasCycle,
    reachedRoot,
  };
};

export const resolveActivePathNodes = (
  mindmap: MindmapMetadata,
  nodes: CommandNode[],
  edges: CommandEdge[]
): CommandNode[] => {
  return resolveActivePath(mindmap, nodes, edges).nodes;
};

export const validateGeneration = ({
  mindmap,
  nodes,
  edges,
  templates,
  target,
}: GenerationValidationInput): ValidatorDiagnostic[] => {
  const diagnostics: ValidatorDiagnostic[] = [];
  const nodesById = new Map(nodes.map((node) => [node.id, node]));
  const templatesById = new Map(templates.map((template) => [template.id, template]));

  if (!mindmap.rootNodeId || !nodesById.has(mindmap.rootNodeId)) {
    diagnostics.push({
      code: "missing-root-node",
      level: "error",
      message: "The mindmap does not have a valid root node.",
    });
  }

  if (!mindmap.activePathId || !nodesById.has(mindmap.activePathId)) {
    diagnostics.push({
      code: "missing-active-path",
      level: "error",
      message: "The mindmap does not have a valid active path.",
    });
    return diagnostics;
  }

  const activePath = resolveActivePath(mindmap, nodes, edges);
  const pathNodes = activePath.nodes;
  if (pathNodes.length === 0) {
    diagnostics.push({
      code: "empty-active-path",
      level: "error",
      message: "The selected active path cannot be resolved.",
    });
    return diagnostics;
  }

  if (mindmap.rootNodeId && !activePath.reachedRoot) {
    diagnostics.push({
      code: "active-path-not-rooted",
      level: "error",
      message: "The active path does not resolve back to the root node.",
      nodeId: mindmap.activePathId,
    });
  }

  if (activePath.hasCycle) {
    diagnostics.push({
      code: "cycle-detected",
      level: "error",
      message: "A cycle was detected in the active path.",
      nodeId: mindmap.activePathId,
    });
  }

  for (const node of pathNodes) {
    if (!node.includeInOutput) continue;
    if (!node.templateId) {
      diagnostics.push({
        code: "missing-template",
        level: "error",
        message: `Node \"${node.title}\" does not have a template assigned.`,
        nodeId: node.id,
      });
      continue;
    }

    const template = templatesById.get(node.templateId);
    if (!template) {
      diagnostics.push({
        code: "unknown-template",
        level: "error",
        message: `Node \"${node.title}\" references a missing template.`,
        nodeId: node.id,
        templateId: node.templateId,
      });
      continue;
    }

    if (template.platformKind !== target) {
      diagnostics.push({
        code: "target-platform-mismatch",
        level: "error",
        message: `Template \"${template.name}\" targets ${template.platformKind}, not ${target}.`,
        nodeId: node.id,
        templateId: template.id,
      });
    }

    for (const definition of template.params) {
      const value = node.params.find((param) => param.paramKey === definition.paramKey)?.value ?? "";
      if (definition.required && !value && !definition.defaultValue) {
        diagnostics.push({
          code: "missing-required-parameter",
          level: "error",
          message: `Node \"${node.title}\" is missing required parameter \"${definition.paramKey}\".`,
          nodeId: node.id,
          templateId: template.id,
        });
      }
    }
  }

  return diagnostics;
};
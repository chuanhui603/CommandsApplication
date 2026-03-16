import type {
  MindmapDetail,
  TemplateDefinition,
  ValidatorDiagnostic,
} from "../domain/models";
import type { MindmapSummary } from "../domain/contracts";

export interface MindmapTransferPackage {
  format: "command-mindmap";
  version: 1;
  payload: MindmapDetail;
  referencedTemplates: TemplateDefinition[];
}

export interface TemplateLibraryTransferPackage {
  format: "command-template-library";
  version: 1;
  templates: TemplateDefinition[];
}

export const createMindmapTransferPackage = (
  detail: MindmapDetail,
  templates: TemplateDefinition[]
): MindmapTransferPackage => {
  const referencedTemplateIds = new Set(detail.nodes.map((node) => node.templateId).filter(Boolean));
  return {
    format: "command-mindmap",
    version: 1,
    payload: detail,
    referencedTemplates: templates.filter((template) => referencedTemplateIds.has(template.id) && !template.builtIn),
  };
};

export const createTemplateLibraryTransferPackage = (
  templates: TemplateDefinition[]
): TemplateLibraryTransferPackage => ({
  format: "command-template-library",
  version: 1,
  templates,
});

export const parseMindmapTransferPackage = (payload: unknown): MindmapTransferPackage | null => {
  if (!payload || typeof payload !== "object") return null;
  const candidate = payload as Partial<MindmapTransferPackage>;
  return candidate.format === "command-mindmap" && candidate.version === 1 && candidate.payload
    ? (candidate as MindmapTransferPackage)
    : null;
};

export const parseTemplateLibraryTransferPackage = (
  payload: unknown
): TemplateLibraryTransferPackage | null => {
  if (!payload || typeof payload !== "object") return null;
  const candidate = payload as Partial<TemplateLibraryTransferPackage>;
  return candidate.format === "command-template-library" && candidate.version === 1 && Array.isArray(candidate.templates)
    ? (candidate as TemplateLibraryTransferPackage)
    : null;
};

export const validateMindmapImportConflicts = (
  transferPackage: MindmapTransferPackage,
  existingMindmaps: MindmapSummary[],
  existingTemplates: TemplateDefinition[]
): ValidatorDiagnostic[] => {
  const diagnostics: ValidatorDiagnostic[] = [];
  const conflictingMindmap = existingMindmaps.find(
    (mindmap) =>
      mindmap.id === transferPackage.payload.mindmap.id ||
      mindmap.name.toLowerCase() === transferPackage.payload.mindmap.name.toLowerCase()
  );
  if (conflictingMindmap) {
    diagnostics.push({
      code: "mindmap-import-conflict",
      level: "error",
      message: `Mindmap import conflicts with existing mindmap \"${conflictingMindmap.name}\".`,
    });
  }

  for (const template of transferPackage.referencedTemplates) {
    const conflictingTemplate = existingTemplates.find(
      (existing) => existing.id === template.id || existing.name.toLowerCase() === template.name.toLowerCase()
    );
    if (conflictingTemplate) {
      diagnostics.push({
        code: "template-import-conflict",
        level: "error",
        message: `Referenced template \"${template.name}\" conflicts with existing template \"${conflictingTemplate.name}\".`,
        templateId: template.id,
      });
    }
  }

  return diagnostics;
};

export const validateTemplateBundleConflicts = (
  transferPackage: TemplateLibraryTransferPackage,
  existingTemplates: TemplateDefinition[]
): ValidatorDiagnostic[] => {
  const diagnostics: ValidatorDiagnostic[] = [];
  for (const template of transferPackage.templates) {
    const conflictingTemplate = existingTemplates.find(
      (existing) => existing.id === template.id || existing.name.toLowerCase() === template.name.toLowerCase()
    );
    if (conflictingTemplate) {
      diagnostics.push({
        code: "template-bundle-conflict",
        level: "error",
        message: `Template \"${template.name}\" conflicts with existing template \"${conflictingTemplate.name}\".`,
        templateId: template.id,
      });
    }
  }
  return diagnostics;
};
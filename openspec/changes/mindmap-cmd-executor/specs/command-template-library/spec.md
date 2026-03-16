## ADDED Requirements

### Requirement: The system provides built-in and user-defined templates
The system SHALL provide built-in read-only templates and allow users to create, save, and manage user-defined templates in a separate editable library.

#### Scenario: View built-in templates
- **WHEN** the user browses the template library
- **THEN** the system SHALL show built-in templates as available for use but not directly editable

#### Scenario: Create a custom template
- **WHEN** the user creates a new template in the user library
- **THEN** the system SHALL persist the template as an editable user-defined template with a platform kind, parameter definitions, and command pattern data

### Requirement: Built-in templates can only be customized through cloning
The system SHALL require users to clone a built-in template before making changes to it.

#### Scenario: Attempt to edit a built-in template
- **WHEN** the user tries to modify a built-in template directly
- **THEN** the system SHALL prevent direct editing and offer a clone action instead

#### Scenario: Clone built-in template into user library
- **WHEN** the user clones a built-in template
- **THEN** the system SHALL create an editable user-defined copy that preserves the original template structure and marks the clone as user-owned

### Requirement: Templates declare one explicit platform kind
Each template SHALL declare exactly one platform kind from Linux shell, WSL, or Windows PowerShell.

#### Scenario: Define template platform kind
- **WHEN** the user creates or edits a template
- **THEN** the system SHALL require selection of exactly one supported platform kind for that template

#### Scenario: Use platform-specific templates in the library
- **WHEN** the user filters or browses templates by platform kind
- **THEN** the system SHALL display templates according to their declared platform kind

### Requirement: Template libraries can be imported and exported in bulk JSON form
The system SHALL support bulk export and bulk import of template libraries through a template-library-specific JSON format separate from mindmap JSON.

#### Scenario: Export template library bundle
- **WHEN** the user exports templates from the library
- **THEN** the system SHALL produce a template-library JSON bundle containing the selected templates and their metadata

#### Scenario: Bulk import template library bundle
- **WHEN** the user imports a valid template-library JSON bundle
- **THEN** the system SHALL create all non-conflicting templates defined in the bundle as one import operation

#### Scenario: Block conflicting template bundle import
- **WHEN** the imported template-library bundle contains an identifier or unique name that conflicts with local stored templates
- **THEN** the system SHALL block the import and report the conflicts instead of overwriting or renaming templates automatically

### Requirement: AI command-set templates are user-added rather than built-in defaults
The system SHALL allow AI-related command-set templates such as GitHub Copilot, Claude, and OpenSpec to exist as user-defined template sets without shipping them as built-in defaults.

#### Scenario: Add AI command-set templates as user content
- **WHEN** the user imports or creates AI-related command templates for GitHub Copilot, Claude, or OpenSpec
- **THEN** the system SHALL store them in the editable user template library rather than the built-in template set
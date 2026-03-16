## ADDED Requirements

### Requirement: Validation is a first-class capability independent of generation implementation
The system SHALL define validation behavior through a dedicated validator capability so future validation changes can evolve independently from editor and generation behavior.

#### Scenario: Extend validator rules in a future change
- **WHEN** a future change adds or modifies validation logic
- **THEN** the validation requirements SHALL be changed through the validator capability spec without requiring unrelated capability specs to be redefined

### Requirement: Validation blocks generation when required structural rules fail
The system SHALL prevent output generation when blocking validation rules fail.

#### Scenario: Missing root node
- **WHEN** the current mindmap has no root node defined
- **THEN** the validator SHALL report a blocking error and generation SHALL be denied

#### Scenario: Missing active output path
- **WHEN** the current mindmap does not define a valid active output path
- **THEN** the validator SHALL report a blocking error and generation SHALL be denied

#### Scenario: Cyclic dependency in the active path
- **WHEN** the active path or required dependencies contain a cycle
- **THEN** the validator SHALL report a blocking error and generation SHALL be denied

### Requirement: Validation blocks generation when required content rules fail
The system SHALL prevent output generation when required template, parameter, or target compatibility rules fail.

#### Scenario: Required parameter is missing
- **WHEN** a node on the active path lacks a required parameter value
- **THEN** the validator SHALL report a blocking error identifying the missing parameter and generation SHALL be denied

#### Scenario: Template platform kind is incompatible with build target
- **WHEN** a node on the active path references a template whose platform kind does not match the current build target
- **THEN** the validator SHALL report a blocking error and generation SHALL be denied

### Requirement: Validation returns actionable diagnostics
The validator SHALL return diagnostics that identify the failing entity and the reason for the failure.

#### Scenario: Report import conflicts
- **WHEN** a JSON import is blocked because of identifier or unique-name conflicts
- **THEN** the validator SHALL return diagnostics that identify each conflicting field or entity

#### Scenario: Report generation-blocking errors
- **WHEN** the validator blocks generation
- **THEN** the system SHALL present diagnostics that identify the affected node, edge, template, or graph-level rule
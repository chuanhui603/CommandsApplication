## ADDED Requirements

### Requirement: The system generates output from one active path at a time
The system SHALL generate output only from a single active path selected within the current mindmap.

#### Scenario: Generate from active path
- **WHEN** the user requests output generation for a mindmap with a valid active path
- **THEN** the system SHALL resolve only the nodes and edges on that active path, excluding nodes not marked for inclusion in the selected path

#### Scenario: Keep alternative branches without generating them
- **WHEN** the mindmap contains additional branches outside the active path
- **THEN** the system SHALL preserve those branches in the graph without including them in the generated output

### Requirement: The system supports live preview and explicit generation
The system SHALL provide live output preview during editing and a separate explicit generation action that records the current result.

#### Scenario: Update preview during editing
- **WHEN** the user changes node parameters, template references, target selection, or active-path selection
- **THEN** the system SHALL refresh the preview output to reflect the current graph state

#### Scenario: Record generated output
- **WHEN** the user explicitly generates output
- **THEN** the system SHALL persist the generated content as the last generated result together with the active target and generation timestamp

### Requirement: The system supports command and script output modes
The system SHALL allow users to generate either a final command string or a target-specific script representation.

#### Scenario: Generate final command string
- **WHEN** the user selects command output mode and generates output
- **THEN** the system SHALL produce one final command string for the selected target based on the active path

#### Scenario: Generate script output
- **WHEN** the user selects script output mode and generates output
- **THEN** the system SHALL produce a target-appropriate script representation for the selected active path

### Requirement: Supported parameter types are constrained in the first release
The system SHALL support text, boolean, single-select, and path parameter types in the first release.

#### Scenario: Edit supported parameter types
- **WHEN** the user configures parameter values on a node or template
- **THEN** the system SHALL allow values only for the supported parameter types defined for the first release

### Requirement: Generated output retains current version metadata
The system SHALL associate generated output with the current stored version metadata of the mindmap.

#### Scenario: Persist generation metadata
- **WHEN** output is generated successfully
- **THEN** the system SHALL store the current version reference, last generated result, and updated timestamp for that mindmap
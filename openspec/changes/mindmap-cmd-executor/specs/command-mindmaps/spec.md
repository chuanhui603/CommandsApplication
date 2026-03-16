## ADDED Requirements

### Requirement: Users can create and manage command mindmaps
The system SHALL allow users to create, rename, edit, save, and reopen command mindmaps composed of command-fragment nodes and edges.

#### Scenario: Create a new mindmap
- **WHEN** the user creates a new command mindmap
- **THEN** the system creates an empty mindmap with a unique identifier, a root-ready state, and persisted metadata for current version, last generated result reference, and update timestamp

#### Scenario: Reopen a saved mindmap
- **WHEN** the user opens a previously saved command mindmap
- **THEN** the system restores the stored nodes, edges, active path state, per-view layout state, and metadata from local persistence

#### Scenario: Persist edits through autosave
- **WHEN** the user makes a meaningful edit to a mindmap such as changing node content, edges, parameters, layout, or active-path state
- **THEN** the system SHALL persist the updated mindmap automatically without requiring an explicit save action

### Requirement: The editor supports both Tree View and Graph View over one shared graph
The system SHALL provide Tree View and Graph View for the same underlying command graph while keeping command content and generation semantics shared across both views.

#### Scenario: Edit content in Tree View
- **WHEN** the user edits a node title, template reference, parameter value, or inclusion state in Tree View
- **THEN** the same changes SHALL appear in Graph View because both views are backed by the same graph content model

#### Scenario: Preserve separate layouts for each view
- **WHEN** the user repositions nodes in Graph View and later adjusts layout in Tree View
- **THEN** the system SHALL persist each view's layout state separately without overwriting the other view's positions

### Requirement: The free-graph editor uses an established graph-editing package
The system SHALL implement Graph View on top of a maintained graph-editing package rather than a custom renderer in the first release.

#### Scenario: Render graph editing interactions
- **WHEN** the user pans, zooms, drags nodes, or creates edges in Graph View
- **THEN** the system SHALL provide those interactions through the selected graph-editing package while keeping command-graph data mapped into the shared application model

### Requirement: The editor supports flexible node creation flows
The system SHALL support creating nodes through both toolbox drag-and-drop and contextual creation actions, with node details editable in a side inspector.

#### Scenario: Create from toolbox
- **WHEN** the user drags a node or template entry from the toolbox onto the canvas
- **THEN** the system SHALL create a new node at the drop location and open it for further editing in the inspector

#### Scenario: Create from context menu
- **WHEN** the user invokes the canvas or node context menu to add a node
- **THEN** the system SHALL create the node in the current graph context and allow editing through the inspector

### Requirement: Mindmaps can be exported and imported as a dedicated JSON format
The system SHALL support exporting and importing complete mindmaps through a mindmap-specific JSON format that is separate from template-library JSON.

#### Scenario: Export a mindmap with referenced templates
- **WHEN** the user exports a command mindmap
- **THEN** the system SHALL produce a mindmap JSON package that includes the graph structure, metadata, and the referenced templates required to reconstruct the graph on another machine

#### Scenario: Reject template-library JSON as a mindmap import
- **WHEN** the user attempts to import a template-library JSON file into the mindmap import flow
- **THEN** the system SHALL reject the file as an incompatible format

#### Scenario: Block mindmap import conflicts
- **WHEN** the imported mindmap package contains an identifier or unique name that conflicts with local stored data
- **THEN** the system SHALL block the import and report the conflicting fields instead of auto-renaming or overwriting local data
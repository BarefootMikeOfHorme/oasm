# OASM Native Modular Component Architecture

**Philosophy:** OASM-native components first, maximum modularity, assembler compatibility as optional invokable layer

**Version:** 1.0.0

---

## 🎯 Design Principles

1. **Native OASM First** - Build our own instruction set, execution model, data flow
2. **Maximum Modularity** - Each component is standalone, composable, testable
3. **Clear Interfaces** - Trait-based boundaries between all components
4. **Optional Compatibility** - NASM/MASM compatibility is a separate layer, not core
5. **Template vs Module** - < 100 LOC → Template, High-frequency → Module

---

## 📦 Complete Component Map

### TIER 1: Core Language Components (Always Loaded)

#### 1.1 Instruction Parser
**Location:** `crates/oasm-core/src/parser/`
**Type:** Module (high-frequency)
**Purpose:** Parse OASM native syntax into IR

```rust
// Native OASM parser - NOT assembler compatibility
pub trait InstructionParser {
    fn parse_line(&self, line: &str) -> Result<Instruction>;
    fn parse_file(&self, path: &Path) -> Result<Vec<Instruction>>;
    fn parse_block(&self, source: &str) -> Result<InstructionBlock>;
}

pub struct OasmNativeParser {
    // Parses native OASM syntax
}

pub struct AssemblerCompatParser {
    // Optional: NASM/MASM syntax → OASM IR (separate invokable layer)
}
```

#### 1.2 Instruction Executor
**Location:** `crates/oasm-core/src/executor/`
**Type:** Module (main pipeline)
**Purpose:** Execute OASM instructions

```rust
pub trait InstructionExecutor {
    fn execute(&mut self, instruction: Instruction, ctx: &mut Context) -> Result<ExecutionResult>;
    fn execute_batch(&mut self, block: CommandBlock, ctx: &mut Context) -> Result<BatchResult>;
    fn rollback(&mut self, checkpoint: Checkpoint) -> Result<()>;
}

pub struct NativeExecutor {
    // Executes OASM instructions natively
}
```

#### 1.3 Type System
**Location:** `crates/oasm-core/src/types/`
**Type:** Module (core)
**Purpose:** OASM native type system

```rust
pub enum OasmType {
    // Primitive types
    U8, U16, U32, U64, I8, I16, I32, I64,
    F32, F64,
    Bool, Char, String,

    // Composite types
    Array { element_type: Box<OasmType>, size: usize },
    Struct { name: String, fields: Vec<Field> },
    Enum { name: String, variants: Vec<Variant> },

    // Geometric types (CAD-specific)
    Vector2, Vector3, Vector4,
    Matrix3x3, Matrix4x4,
    BoundingBox, Mesh,

    // Object types
    Object { object_type: String },
}

pub trait TypeChecker {
    fn infer_type(&self, value: &Value) -> OasmType;
    fn check_assignment(&self, target: &OasmType, value: &OasmType) -> Result<()>;
    fn validate_operation(&self, op: &Operation, operands: &[OasmType]) -> Result<OasmType>;
}
```

#### 1.4 Context Manager
**Location:** `crates/oasm-core/src/context/`
**Type:** Module (always needed)
**Purpose:** Manage execution context, scope, variables

```rust
pub struct ExecutionContext {
    pub run_id: RunId,
    pub seq: Seq,
    pub actor: Actor,
    pub working_directory: PathBuf,
    pub variables: HashMap<String, Variable>,
    pub objects: HashMap<String, Object>,
    pub scope_stack: Vec<Scope>,
}

pub trait ContextManager {
    fn push_scope(&mut self);
    fn pop_scope(&mut self);
    fn declare_variable(&mut self, name: String, var_type: OasmType) -> Result<()>;
    fn assign_variable(&mut self, name: &str, value: Value) -> Result<()>;
    fn get_variable(&self, name: &str) -> Result<&Variable>;
    fn create_object(&mut self, object_type: String, id: Option<String>) -> Result<String>;
}
```

---

### TIER 2: Format & Data Components

#### 2.1 Format Converters
**Location:** `crates/asm-formats/src/converters/`
**Type:** Module (high-frequency)
**Purpose:** Convert between HDF5/CBOR/YAML/JSON

```rust
pub trait FormatConverter {
    fn convert(&self, input: &[u8], from: Format, to: Format) -> Result<Vec<u8>>;
}

pub struct HDF5ToCBORConverter;
pub struct CBORToYAMLConverter;
pub struct YAMLToCBORConverter;
pub struct CBORToJSONLineageConverter;

pub struct ConversionPipeline {
    converters: HashMap<(Format, Format), Box<dyn FormatConverter>>,
}
```

#### 2.2 Template Loader
**Location:** `crates/oasm-core/src/templates/`
**Type:** Module
**Purpose:** Load templates from HDF5/YAML

```rust
pub trait TemplateLoader {
    fn load_template(&self, template_id: &str) -> Result<Template>;
    fn list_templates(&self, category: TemplateCategory) -> Result<Vec<TemplateInfo>>;
    fn instantiate(&self, template_id: &str, params: HashMap<String, Value>) -> Result<InstructionBlock>;
}

pub struct HDF5TemplateLoader;
pub struct YAMLTemplateLoader;
```

#### 2.3 Artifact Store Interface
**Location:** `crates/asm-formats/src/artifact_store/`
**Type:** Module
**Purpose:** Manage immutable artifacts in HDF5

```rust
pub trait ArtifactStore {
    fn store(&mut self, artifact: ImmutableArtifact) -> Result<String>;  // Returns artifact_id
    fn retrieve(&self, artifact_id: &str) -> Result<ImmutableArtifact>;
    fn list_artifacts(&self, filter: ArtifactFilter) -> Result<Vec<ArtifactInfo>>;
    fn create_working_copy(&self, artifact_id: &str, run_id: RunId, seq: Seq) -> Result<WorkingCopy>;
}

pub struct HDF5ArtifactStore {
    root_path: PathBuf,
}
```

---

### TIER 3: Execution Support Components

#### 3.1 Command Block Builder
**Location:** `crates/oasm-core/src/command_blocks/`
**Type:** Module
**Purpose:** Build batched command blocks

```rust
pub trait CommandBlockBuilder {
    fn new(block_type: BlockType) -> Self;
    fn add_instruction(&mut self, instruction: Instruction) -> &mut Self;
    fn add_target(&mut self, target: String) -> &mut Self;
    fn add_rule(&mut self, rule_id: String) -> &mut Self;
    fn build(self) -> Result<CommandBlock>;
}

pub struct BatchBuilder {
    block_type: BlockType,
    instructions: Vec<Instruction>,
    targets: Vec<String>,
    rules: Vec<String>,
    execution_mode: ExecutionMode,
}

impl CommandBlockBuilder for BatchBuilder { ... }
```

#### 3.2 Rule Engine
**Location:** `crates/oasm-core/src/rules/`
**Type:** Module
**Purpose:** Load and enforce rules hierarchy

```rust
pub trait RuleEngine {
    fn load_rules(&mut self, hierarchy: RuleHierarchy) -> Result<()>;
    fn evaluate(&self, instruction: &Instruction, ctx: &Context) -> Result<RuleEvaluation>;
    fn get_applicable_rules(&self, instruction_type: &str) -> Vec<Rule>;
}

pub struct HierarchicalRuleEngine {
    core_rules: Vec<Rule>,        // Universal, immutable
    domain_rules: HashMap<String, Vec<Rule>>,  // Domain-specific
    project_rules: Vec<Rule>,     // Project overrides
    session_rules: Vec<Rule>,     // Ephemeral
}

pub struct RuleHierarchy {
    // Priority: Session → Project → Domain → Core
}
```

#### 3.3 Validator
**Location:** `crates/oasm-core/src/validation/`
**Type:** Module
**Purpose:** Validate instructions, objects, data

```rust
pub trait Validator {
    fn validate_instruction(&self, instruction: &Instruction) -> Result<ValidationResult>;
    fn validate_object(&self, object: &Object, rules: &[Rule]) -> Result<ValidationResult>;
    fn validate_type(&self, value: &Value, expected_type: &OasmType) -> Result<()>;
}

pub struct TopologyValidator;  // CAD-specific
pub struct TypeValidator;       // Type system
pub struct RuleValidator;       // Rule compliance
```

#### 3.4 Checkpoint Manager
**Location:** `crates/oasm-core/src/checkpoints/`
**Type:** Module
**Purpose:** Create and restore checkpoints

```rust
pub trait CheckpointManager {
    fn create_checkpoint(&mut self, ctx: &ExecutionContext) -> Result<Checkpoint>;
    fn restore_checkpoint(&mut self, checkpoint: &Checkpoint, ctx: &mut ExecutionContext) -> Result<()>;
    fn list_checkpoints(&self) -> Vec<CheckpointInfo>;
    fn cleanup_old_checkpoints(&mut self, before: DateTime<Utc>) -> Result<usize>;
}

pub struct InMemoryCheckpointManager;
pub struct PersistentCheckpointManager;
```

---

### TIER 4: Provenance & Lineage Components

#### 4.1 Provenance Tracker
**Location:** `crates/asm-formats/src/provenance/`
**Type:** Module
**Purpose:** Track execution provenance

```rust
pub trait ProvenanceTracker {
    fn record_execution(&mut self, instruction: &Instruction, result: &ExecutionResult) -> Result<ProvenanceRecord>;
    fn get_provenance(&self, lineage_id: &str) -> Result<ProvenanceRecord>;
    fn get_lineage_chain(&self, lineage_id: &str) -> Result<Vec<ProvenanceRecord>>;
}

pub struct ProvenanceRecord {
    pub lineage_id: String,
    pub parent_lineage_id: Option<String>,
    pub run_id: RunId,
    pub seq: Seq,
    pub actor: Actor,
    pub instruction: Instruction,
    pub outcome: ExecutionOutcome,
    pub timestamp: DateTime<Utc>,
    pub tool_versions: ToolVersions,
}
```

#### 4.2 Lineage Manager
**Location:** `crates/asm-formats/src/lineage/`
**Type:** Module
**Purpose:** Manage JSON lineage logs

```rust
pub trait LineageManager {
    fn write_lineage(&mut self, record: JSONLineage) -> Result<String>;  // Returns lineage_id
    fn read_lineage(&self, lineage_id: &str) -> Result<JSONLineage>;
    fn get_revert_command(&self, lineage_id: &str) -> Result<String>;
    fn execute_revert(&mut self, lineage_id: &str) -> Result<()>;
}

pub struct FileBasedLineageManager {
    lineage_dir: PathBuf,
}
```

#### 4.3 Diff Generator
**Location:** `crates/asm-formats/src/diffs/`
**Type:** Module
**Purpose:** Generate diffs for changes

```rust
pub trait DiffGenerator {
    fn generate_diff(&self, before: &Snapshot, after: &Snapshot) -> Result<Diff>;
    fn apply_diff(&self, snapshot: &mut Snapshot, diff: &Diff) -> Result<()>;
    fn revert_diff(&self, snapshot: &mut Snapshot, diff: &Diff) -> Result<()>;
}

pub struct UnifiedDiffGenerator;
pub struct StructuredDiffGenerator;  // For objects, not just text
```

---

### TIER 5: Analysis & Diagnostics Components

#### 5.1 Scanner
**Location:** `compiler/src/scanner.rs` (exists)
**Type:** Module
**Purpose:** Scan codebases for metrics

```rust
pub trait Scanner {
    fn scan(&self, root: &Path) -> Result<ScanReport>;
    fn scan_file(&self, file: &Path) -> Result<FileMetrics>;
}

pub struct CodebaseScanner {
    // Deep codebase analysis
}
```

#### 5.2 Diagnostics Engine
**Location:** `compiler/src/diagnostics.rs` (exists)
**Type:** Module
**Purpose:** Structured error reporting

```rust
pub trait DiagnosticsEngine {
    fn add_error(&mut self, code: DiagnosticCode, message: String, location: SourceLocation);
    fn add_warning(&mut self, code: DiagnosticCode, message: String, location: SourceLocation);
    fn add_suggestion(&mut self, suggestion: String);
    fn to_report(&self) -> DiagnosticReport;
}

pub struct DiagnosticBag {
    // Exists
}
```

#### 5.3 Profiler
**Location:** `crates/oasm-core/src/profiler/` (new)
**Type:** Module
**Purpose:** Profile execution performance

```rust
pub trait Profiler {
    fn start_profiling(&mut self);
    fn stop_profiling(&mut self) -> Result<ProfileReport>;
    fn mark(&mut self, label: &str);
    fn get_metrics(&self) -> ProfileMetrics;
}

pub struct ExecutionProfiler {
    start_time: Instant,
    marks: Vec<(String, Instant)>,
}
```

---

### TIER 6: I/O & Integration Components

#### 6.1 Import Manager
**Location:** `crates/oasm-core/src/import/`
**Type:** Module
**Purpose:** Import modules and libraries

```rust
pub trait ImportManager {
    fn import_module(&mut self, module_path: &str, alias: Option<String>) -> Result<Module>;
    fn resolve_import(&self, name: &str) -> Result<PathBuf>;
    fn list_available_modules(&self) -> Result<Vec<ModuleInfo>>;
}

pub struct ModuleImporter {
    search_paths: Vec<PathBuf>,
    loaded_modules: HashMap<String, Module>,
}
```

#### 6.2 Export Manager
**Location:** `crates/oasm-core/src/export/`
**Type:** Module
**Purpose:** Export to various formats

```rust
pub trait ExportManager {
    fn export(&self, object: &Object, format: ExportFormat, path: &Path) -> Result<ExportResult>;
    fn validate_before_export(&self, object: &Object, format: ExportFormat) -> Result<ValidationResult>;
    fn compute_checksum(&self, path: &Path) -> Result<String>;
}

pub struct CADExporter;  // STEP, IGES, STL
pub struct DataExporter;  // JSON, CSV, Parquet
pub struct CodeExporter;  // Generate code from OASM
```

#### 6.3 FFI Bridge
**Location:** `crates/oasm-core/src/ffi/`
**Type:** Module
**Purpose:** Interface with native libraries

```rust
pub trait FFIBridge {
    fn load_library(&mut self, path: &Path) -> Result<LibraryHandle>;
    fn call_function(&self, handle: LibraryHandle, func_name: &str, args: Vec<Value>) -> Result<Value>;
    fn unload_library(&mut self, handle: LibraryHandle) -> Result<()>;
}

pub struct NativeFFIBridge {
    loaded_libs: HashMap<LibraryHandle, libloading::Library>,
}
```

---

### TIER 7: Domain-Specific Components (CAD, Engine, etc.)

#### 7.1 CAD Operations
**Location:** `crates/oasm-domains/src/cad/`
**Type:** Module (domain-specific)
**Purpose:** CAD-specific operations

```rust
pub trait CADOperations {
    fn create_primitive(&mut self, primitive_type: PrimitiveType) -> Result<Object>;
    fn extrude(&mut self, object: &mut Object, direction: Vector3, distance: f64) -> Result<()>;
    fn fillet(&mut self, object: &mut Object, edges: Vec<usize>, radius: f64) -> Result<()>;
    fn boolean_op(&mut self, obj1: &Object, obj2: &Object, op: BooleanOp) -> Result<Object>;
}

pub struct NativeCADEngine {
    // OASM's native CAD operations
}

pub struct ExternalCADAdapter {
    // Adapter for external CAD kernels (OpenCASCADE, etc.)
}
```

#### 7.2 Game Engine Operations
**Location:** `crates/oasm-domains/src/engine/`
**Type:** Module (domain-specific)
**Purpose:** Game engine operations

```rust
pub trait EngineOperations {
    fn create_entity(&mut self) -> Result<EntityId>;
    fn attach_component(&mut self, entity: EntityId, component: Component) -> Result<()>;
    fn update_scene(&mut self, delta_time: f64) -> Result<()>;
}
```

#### 7.3 Document Operations
**Location:** `crates/oasm-domains/src/document/`
**Type:** Module (domain-specific)
**Purpose:** Document generation

```rust
pub trait DocumentOperations {
    fn insert_content(&mut self, content: Content, position: Position) -> Result<()>;
    fn apply_style(&mut self, style: Style, range: Range) -> Result<()>;
    fn generate_toc(&self) -> Result<TableOfContents>;
}
```

---

### TIER 8: Shell Integration Components

#### 8.1 PowerShell Bridge
**Location:** `shells/psmodule/rust_bridge/`
**Type:** Module
**Purpose:** Rust ↔ PowerShell FFI

```rust
pub trait PowerShellBridge {
    fn invoke_cmdlet(&self, cmdlet: &str, params: HashMap<String, Value>) -> Result<Value>;
    fn register_callback(&mut self, name: String, callback: Box<dyn Fn(Vec<Value>) -> Result<Value>>);
}
```

#### 8.2 Python Bridge (PyO3)
**Location:** `crates/oasm-python/src/`
**Type:** Module
**Purpose:** Rust ↔ Python via PyO3

```rust
pub trait PythonBridge {
    fn call_python_function(&self, module: &str, func: &str, args: Vec<Value>) -> Result<Value>;
    fn import_python_module(&mut self, module: &str) -> Result<()>;
}
```

---

### TIER 9: UI & Dashboard Components

#### 9.1 CLI Dashboard
**Location:** `compiler/src/cli_dashboard.rs` (exists)
**Type:** Module
**Purpose:** CLI output formatting

```rust
pub trait DashboardFormatter {
    fn format_row(&self, row: &DashboardRow) -> String;
    fn format_totals(&self, totals: &Totals) -> String;
}

pub struct PlainTextFormatter;
pub struct JSONLFormatter;
pub struct HighDensityFormatter;
```

#### 9.2 TUI (Terminal UI)
**Location:** `ui/rust_ui/src/tui/` (future)
**Type:** Module
**Purpose:** Ratatui-based TUI

```rust
pub trait TUIComponent {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_input(&mut self, key: KeyEvent) -> Result<Action>;
}

pub struct ExecutionMonitor;  // Live execution view
pub struct ProvenanceExplorer;  // Browse lineage
pub struct TemplateManager;  // Manage templates
```

---

### TIER 10: Optional Compatibility Layer

#### 10.1 Assembler Compatibility (Optional, Invokable)
**Location:** `crates/oasm-compat/src/assembler/`
**Type:** Module (optional feature)
**Purpose:** NASM/MASM compatibility

```rust
#[cfg(feature = "assembler_compat")]
pub trait AssemblerCompat {
    fn parse_nasm(&self, source: &str) -> Result<Vec<Instruction>>;
    fn parse_masm(&self, source: &str) -> Result<Vec<Instruction>>;
    fn translate_to_oasm(&self, asm_instructions: Vec<AsmInstruction>) -> Result<Vec<Instruction>>;
}

#[cfg(feature = "assembler_compat")]
pub struct NASMCompatLayer;

#[cfg(feature = "assembler_compat")]
pub struct MASMCompatLayer;
```

**Usage:**
```rust
// Enable via feature flag
// Cargo.toml: oasm-core = { version = "0.1", features = ["assembler_compat"] }

let parser = NASMCompatLayer::new();
let oasm_instructions = parser.parse_nasm(nasm_source)?;
// Now execute as native OASM
executor.execute_batch(oasm_instructions, &mut ctx)?;
```

---

## 🏗️ Modular Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    OASM NATIVE CORE                              │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Parser     │→│   Executor   │→│   Context    │         │
│  │   (Native)   │  │   (Native)   │  │   Manager    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│         ↓                 ↓                  ↓                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Type System  │  │ Rule Engine  │  │  Validator   │         │
│  │  (Native)    │  │ (Hierarchy)  │  │   (Multi)    │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│                  FORMAT & DATA LAYER                             │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   HDF5 ↔    │  │   CBOR ↔    │  │   YAML ↔    │         │
│  │   Artifact   │→│   Runtime   │→│   Overlay   │         │
│  │   Store      │  │   Objects   │  │   (Human)   │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
│         ↓                                  ↓                     │
│  ┌──────────────────────────────┐  ┌──────────────┐           │
│  │    Working Copy Manager       │  │   JSON       │           │
│  │   (Copy-on-Work Pattern)      │  │   Lineage    │           │
│  └──────────────────────────────┘  └──────────────┘           │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│              EXECUTION SUPPORT LAYER                             │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │  Command     │  │  Template    │  │  Checkpoint  │         │
│  │  Block       │  │  Loader      │  │  Manager     │         │
│  │  Builder     │  │              │  │              │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│           PROVENANCE & LINEAGE LAYER                             │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Provenance   │→│   Lineage    │→│     Diff     │         │
│  │   Tracker    │  │   Manager    │  │  Generator   │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│              DOMAIN-SPECIFIC LAYER                               │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │     CAD      │  │    Engine    │  │   Document   │         │
│  │  Operations  │  │  Operations  │  │  Operations  │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│          INTEGRATION & COMPATIBILITY LAYER                       │
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ PowerShell   │  │   Python     │  │   NASM/MASM  │         │
│  │   Bridge     │  │   Bridge     │  │   Compat     │         │
│  │  (Native)    │  │  (PyO3)      │  │  (Optional)  │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📝 Component Dependency Matrix

| Component | Depends On | Used By | Standalone |
|-----------|-----------|---------|------------|
| Parser | Type System | Executor | No |
| Executor | Parser, Context, Rule Engine | Runtime Daemon | No |
| Type System | - | Parser, Validator | Yes |
| Context Manager | - | Executor | Yes |
| Rule Engine | - | Executor, Validator | Yes |
| Validator | Type System, Rule Engine | Executor | No |
| Format Converters | - | Template Loader, Artifact Store | Yes |
| Template Loader | Format Converters, Artifact Store | Executor | No |
| Artifact Store | Format Converters | Template Loader, Working Copy Manager | Yes |
| Command Block Builder | Parser | Executor | No |
| Provenance Tracker | - | Executor | Yes |
| Lineage Manager | - | Provenance Tracker | Yes |
| Diff Generator | - | Lineage Manager | Yes |
| CAD Operations | Executor, Context | Domain Layer | No |
| Assembler Compat | Parser | (Optional) External | Yes |

---

## 🔧 Implementation Priority

### Phase 1: Core Language (Do First)
1. **Instruction Parser** (native OASM syntax)
2. **Type System** (native types)
3. **Context Manager**
4. **Instruction Executor** (native execution)
5. **Rule Engine** (hierarchy resolution)
6. **Validator**

### Phase 2: Format & Data
7. **Format Converters** (HDF5↔CBOR↔YAML↔JSON)
8. **Template Loader**
9. **Artifact Store Interface**
10. **Working Copy Manager**

### Phase 3: Execution Support
11. **Command Block Builder**
12. **Checkpoint Manager**
13. **Provenance Tracker**
14. **Lineage Manager**
15. **Diff Generator**

### Phase 4: Domain-Specific
16. **CAD Operations** (if CAD is priority domain)
17. **Engine Operations** (if game engine is priority)
18. **Document Operations** (if document gen is priority)

### Phase 5: Integration
19. **PowerShell Bridge**
20. **Python Bridge**
21. **Export Manager**
22. **Import Manager**

### Phase 6: Optional Compatibility (Last)
23. **Assembler Compatibility Layer** (NASM/MASM)

---

## 🎯 Trait-Based Modularity

Every component follows this pattern:

```rust
// 1. Define trait (interface)
pub trait ComponentName {
    fn primary_operation(&self, input: Input) -> Result<Output>;
    fn secondary_operation(&mut self, data: Data) -> Result<()>;
}

// 2. Implement for native OASM
pub struct NativeComponent {
    // Native implementation fields
}

impl ComponentName for NativeComponent {
    // Native OASM implementation
}

// 3. Optional: Implement for compatibility layer
#[cfg(feature = "compat_layer")]
pub struct CompatComponent {
    native: NativeComponent,  // Wraps native
    // Compatibility-specific fields
}

#[cfg(feature = "compat_layer")]
impl ComponentName for CompatComponent {
    // Translates compat format → native → execute
}
```

---

## 📦 Crate Organization

```
oasm/
├── crates/
│   ├── oasm-core/              # Core language components (Tier 1)
│   │   ├── src/
│   │   │   ├── parser/         # Native parser
│   │   │   ├── executor/       # Native executor
│   │   │   ├── types/          # Type system
│   │   │   ├── context/        # Context manager
│   │   │   ├── rules/          # Rule engine
│   │   │   ├── validation/     # Validators
│   │   │   ├── command_blocks/ # Command block builder
│   │   │   ├── checkpoints/    # Checkpoint manager
│   │   │   ├── templates/      # Template loader
│   │   │   ├── import/         # Import manager
│   │   │   ├── export/         # Export manager
│   │   │   ├── ffi/            # FFI bridge
│   │   │   └── profiler/       # Profiler
│   │
│   ├── asm-formats/            # Format & data layer (Tier 2)
│   │   ├── src/
│   │   │   ├── converters/     # Format converters
│   │   │   ├── artifact_store/ # HDF5 artifact store
│   │   │   ├── provenance/     # Provenance tracker
│   │   │   ├── lineage/        # Lineage manager
│   │   │   └── diffs/          # Diff generator
│   │
│   ├── oasm-domains/           # Domain-specific (Tier 7)
│   │   ├── src/
│   │   │   ├── cad/            # CAD operations
│   │   │   ├── engine/         # Game engine ops
│   │   │   └── document/       # Document ops
│   │
│   ├── oasm-compat/            # Optional compatibility (Tier 10)
│   │   ├── src/
│   │   │   ├── assembler/      # NASM/MASM compat
│   │   │   └── ...
│   │
│   ├── oasm-python/            # Python bridge (Tier 8)
│   │   └── src/
│   │       └── bridge.rs
│   │
│   └── oasm-api/               # External API (existing)
│
├── compiler/                   # Compiler (uses oasm-core)
├── runtime/daemon/             # Runtime daemon (orchestrates all)
└── shells/                     # Shell integration
```

---

## 🚀 Next Steps

1. **Implement Core Language Components** (Parser, Executor, Type System, Context)
2. **Create trait definitions for all components**
3. **Build native OASM implementations first**
4. **Add domain-specific components based on priority**
5. **Compatibility layers are feature-gated and optional**

**Focus:** Build a powerful, modular, native OASM system. Compatibility is just sugar on top.

---

**Document Version:** 1.0.0
**Last Updated:** 2025-12-18

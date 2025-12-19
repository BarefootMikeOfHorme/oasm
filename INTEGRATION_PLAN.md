# OASM Integration & Evolution Plan

**Status:** Planning Phase
**Purpose:** Integrate asm-formats, establish template-first philosophy, prepare wpshell 2026 Docker build
**Context:** Builds on existing OASM system (manifest, modules, scanner, templates, PowerShell integration)

---

## 🎯 Overview

This plan integrates the new **asm-formats crate** (HDF5/CBOR/YAML/JSON pipeline) with OASM's existing architecture, establishes **template-first modularity**, and prepares for **wpshell 2026 + Rust + PyO3 Docker build** (future phase).

---

## 📋 Part 1: asm-formats Integration with Existing Serialization

### Current State (Existing)

**OASM Serialization:**
- CBOR - Canonical runtime state
- YAML - Human-readable mirrors with comments
- JSON - Structured logs and indexes

**Objex Serialization:**
- HDF5 - Archival storage
- CBOR - Live SmartObject state
- STEP/IGES/STL - CAD exports
- YAML - Export metadata

### New Addition (asm-formats crate)

**Provides:**
- Unified HDF5→CBOR→YAML→JSON pipeline
- Copy-on-work pattern (immutable artifacts → working copies)
- Domain-specific schemas (folders, logs, shells, Rust/Python modules)
- Strict conversion rules with provenance tracking
- Command block batching

### Integration Strategy

#### 1.1 Extend Existing Schema Registry

**Current schemas (8 registered):**
- oasm_config, oasm_protocol, cli_state, diagnostic_index
- bindings_manifest, smartobject_container, scan_template, workflow_template

**Add to registry:**
- `hdf5_template` - Immutable artifact templates (asm-formats)
- `cbor_runtime` - Runtime execution objects (asm-formats)
- `yaml_overlay` - Human-readable overlays (asm-formats)
- `json_lineage` - Audit trail and revert logs (asm-formats)
- `working_copy` - Copy-on-work pattern (asm-formats)
- `command_block` - Batched execution units (asm-formats)
- `provenance_log` - Enhanced provenance tracking (asm-formats)

**Total schemas:** 15 registered

#### 1.2 Update Master Manifest

**File:** `manifests/master_manifest.yaml`

**Add section:**
```yaml
schemas:
  # ... existing 8 schemas ...

  # asm-formats schemas
  - id: hdf5_template
    path: crates/asm-formats/schemas/hdf5_template_schema.yaml
    format: YAML
    purpose: Immutable artifact templates

  - id: cbor_runtime
    path: crates/asm-formats/schemas/cbor_runtime_schema.yaml
    format: YAML
    purpose: Runtime execution objects

  - id: yaml_overlay
    path: crates/asm-formats/schemas/yaml_overlay_schema.yaml
    format: YAML
    purpose: Human-readable overlays with comments

  - id: json_lineage
    path: crates/asm-formats/schemas/json_lineage_schema.yaml
    format: YAML
    purpose: Audit trail and revert capability

  - id: working_copy
    path: crates/asm-formats/schemas/working_copy_schema.yaml
    format: YAML
    purpose: Copy-on-work pattern

  - id: command_block
    path: crates/asm-formats/schemas/command_block_schema.yaml
    format: YAML
    purpose: Batched execution units

  - id: provenance_log
    path: crates/asm-formats/schemas/provenance_log_schema.yaml
    format: YAML
    purpose: Enhanced provenance tracking

modules:
  # ... existing 6 modules ...

  # Add asm-formats
  - id: asm-formats
    name: asm-formats
    type: library
    language: Rust
    entry_point: crates/asm-formats/src/lib.rs
    auto_start: false
    purpose: Multi-format schema system (HDF5/CBOR/YAML/JSON)
    capabilities:
      - format_conversion
      - copy_on_work
      - provenance_tracking
      - command_block_batching
```

#### 1.3 Serialization Flow Integration

**Existing OASM flow:**
```
Assembly code → Compiler → CBOR (runtime) ⟷ YAML (human)
                                ↓
                          JSON (logs/index)
```

**Enhanced flow with asm-formats:**
```
HDF5 (immutable artifacts) → Template extraction
                                    ↓
                        CBOR (runtime execution)
                           ↙           ↘
                  Execute            YAML (overlay with comments)
                     ↓
              JSON (lineage + revert capability)
                     ↓
         Working copies (modify, test, commit/discard)
```

**Integration points:**
1. **Compiler output** → Can be stored as HDF5 template
2. **Runtime daemon** → Uses CBOR objects from asm-formats
3. **Scanner output** → Can reference HDF5 baselines
4. **Template library** → Templates stored as HDF5 artifacts

---

## 📋 Part 2: Template-First Modularity (< 100 LOC Philosophy)

### Decision Matrix

| **Criteria** | **Module (Permanent)** | **Template (Ephemeral)** |
|--------------|------------------------|--------------------------|
| **LOC** | Any size | < 100 LOC preferred |
| **Frequency** | High (main pipeline, idle-level) | Low (task-specific) |
| **Variation** | Stable, rarely changes | Varies per task |
| **Loading** | Always loaded at startup | Loaded on-demand, created at time of use |
| **Format** | Rust crate, compiled binary | YAML/CBOR, interpreted |
| **Storage** | Workspace crate | HDF5 immutable store → working copy |
| **Examples** | Diagnostics, converters, parsers, scanner | Repair blocks, lint rules, formatters, validators |

### Current Modules (Keep as Permanent)

**From existing system:**
1. `runtime_daemon` - Service, supervisor loop
2. `compiler` - Compilation, always needed
3. `scanner` - Pre-compile diagnostics, high frequency
4. `oasm_shell` - Interactive CLI
5. `rust_ui` - Visual interface
6. `bindings` - Native DLL bindings
7. `asm-formats` - Format system (NEW)

**All are high-frequency or main pipeline → Keep as modules**

### Templates (Create/Load on Demand)

**Existing in `templates/` directory:**
- Schemas (SmartObject containers, validation rules)
- Scripts (code generation: PS, Python, Rust)
- Commands (diagnostic blocks, health checks)
- Workflows (CAD automation presets)
- Scans (deep codebase analysis)

**New templates to add (< 100 LOC, task-specific):**

```
templates/
├── command_blocks/                    # Batched execution units
│   ├── repair/
│   │   ├── fix_unsafe_blocks.yaml
│   │   ├── add_error_handling.yaml
│   │   ├── fix_imports.yaml
│   │   └── add_logging.yaml
│   ├── lint/
│   │   ├── check_formatting.yaml
│   │   ├── unused_variables.yaml
│   │   └── deprecated_apis.yaml
│   ├── test/
│   │   ├── unit_test_runner.yaml
│   │   ├── integration_test.yaml
│   │   └── coverage_check.yaml
│   └── cad/                          # CAD-specific
│       ├── topology_validation.yaml
│       ├── parametric_constraint.yaml
│       └── export_step.yaml
│
├── rules/                            # Rule hierarchy
│   ├── core/                         # Universal, immutable
│   │   ├── syntax_validation.yaml
│   │   ├── type_checking.yaml
│   │   └── memory_safety.yaml
│   ├── domain/                       # Domain-specific
│   │   ├── rust/
│   │   │   ├── unsafe_block_rules.yaml
│   │   │   ├── lifetime_rules.yaml
│   │   │   └── ownership_rules.yaml
│   │   ├── python/
│   │   │   ├── type_hint_rules.yaml
│   │   │   └── import_rules.yaml
│   │   ├── powershell/
│   │   │   ├── cmdlet_rules.yaml
│   │   │   └── security_rules.yaml
│   │   └── cad/
│   │       ├── geometry_rules.yaml
│   │       └── export_rules.yaml
│   ├── project/                      # Project-specific overrides
│   │   └── oasm_custom_rules.yaml
│   └── session/                      # Ephemeral, runtime
│       └── active_filters.yaml
│
└── schemas/                          # Schema definitions
    ├── existing schemas...
    └── asm-formats schemas...
```

### Rule Hierarchy Resolution

**Priority:** Session → Project → Domain → Core (most specific wins)

**Example:**
```yaml
# Core rule (always enforced)
core/memory_safety.yaml:
  no_raw_pointers_without_unsafe: required

# Domain rule (Rust-specific)
domain/rust/unsafe_block_rules.yaml:
  unsafe_blocks_require_comment: required
  unsafe_blocks_require_audit: required

# Project rule (OASM override)
project/oasm_custom_rules.yaml:
  unsafe_blocks_require_audit: optional  # Override for developer mode

# Session rule (runtime filter)
session/active_filters.yaml:
  skip_unsafe_audit: true  # Temporarily disable during prototyping
```

---

## 📋 Part 3: Logging Schema Integration

### Current Logging Structure

**Existing:**
```
logs/
└── StructureDebug/
    ├── structure_{timestamp}.log
    ├── baseline_index_{timestamp}.json
    └── cli_state_{timestamp}.json
```

### Enhanced Logging Structure

**Add directories for asm-formats:**
```
logs/
├── StructureDebug/          # Existing scanner outputs
├── provenance/              # NEW: HDF5/CBOR references, command blocks
│   ├── {run_id}_{seq}.yaml  # Human-readable provenance
│   └── {run_id}_{seq}.cbor  # Binary provenance (if needed)
├── lineage/                 # NEW: JSON audit trail (revert capability)
│   ├── {lineage_id}.json    # Individual lineage records
│   └── lineage_chain.json   # Linked lineage chain
├── execution/               # NEW: Runtime execution logs
│   ├── {run_id}.log         # Structured execution log
│   └── errors_{run_id}.log  # Error-only log
└── working_copies/          # NEW: Working copy metadata
    └── {copy_id}_metadata.json
```

### Logging Schemas (Hierarchical)

#### Base Schema (Foundation)

```yaml
# schemas/logging_base_schema.yaml
schema_version: "1.0"
schema_type: "logging_base"
extends: null

required_fields:
  timestamp:
    type: DateTime<Utc>
    format: ISO8601
    description: When the event occurred

  run_id:
    type: RunId (UUID)
    description: Unique execution identifier

  seq:
    type: Seq (u64)
    description: Sequence number within run

  actor:
    type: Actor
    description: Who/what performed the action
    enum:
      - Human { username: String }
      - Automation { rule_id: String }
      - AI { model: String, confidence: f64 }
      - System

  log_level:
    type: LogLevel
    enum: [Trace, Debug, Info, Warn, Error, Critical]

optional_fields:
  context:
    type: HashMap<String, String>
    description: Additional contextual metadata

  source_location:
    type: SourceLocation
    description: File/line where log originated
```

#### Provenance Log Schema (Extends Base)

```yaml
# schemas/provenance_log_schema.yaml
schema_version: "1.0"
schema_type: "provenance_log"
extends: logging_base_schema

additional_required_fields:
  command_block_id:
    type: String
    description: ID of command block executed

  hdf5_reference:
    type: String
    description: Reference to immutable artifact in HDF5
    example: "/artifacts/template_001/v1.0"

  cbor_object_id:
    type: String
    description: Runtime CBOR object ID
    format: "{run_id}_{seq}"

  outcome:
    type: ExecutionOutcome
    enum: [Success, Failed, PartialSuccess, Cancelled]

  impact:
    type: Impact
    description: Metrics for changes made
    fields:
      - files_changed: usize
      - lines_added: usize
      - lines_removed: usize
      - functions_affected: usize
      - modules_affected: Vec<String>

additional_optional_fields:
  tool_versions:
    type: ToolVersions
    description: Tool versions used in execution

  config_hash:
    type: String
    description: Hash of configuration at execution time
```

#### Lineage Log Schema (Extends Base, JSON format)

```yaml
# schemas/lineage_log_schema.yaml
schema_version: "1.0"
schema_type: "lineage_log"
extends: logging_base_schema
format: JSON  # Always JSON for Git-friendly, revert capability

additional_required_fields:
  lineage_id:
    type: String (UUID)
    description: Unique audit record ID

  parent_lineage_id:
    type: Option<String>
    description: Links to parent in lineage chain (null for root)

  summary:
    type: String
    description: Human-readable summary of action

  intent:
    type: String
    description: Why this action was performed

  command_executed:
    type: String
    description: Exact command/instruction executed

  diff_id:
    type: Option<String>
    description: Reference to diff snapshot

  git_sha:
    type: Option<String>
    description: Git commit SHA if changes were committed

  revert_command:
    type: String
    description: Command to revert this change
    example: "oasm revert --lineage-id {lineage_id}"

  tests:
    type: Vec<TestRecord>
    description: Test execution records

  provenance:
    type: Provenance
    description: Full provenance chain
    fields:
      - lineage_chain: Vec<String>  # All parent lineage IDs
      - hdf5_template_id: String
      - cbor_object_id: String
      - actor_chain: Vec<Actor>     # All actors involved
```

#### Execution Log Schema (Runtime logs)

```yaml
# schemas/execution_log_schema.yaml
schema_version: "1.0"
schema_type: "execution_log"
extends: logging_base_schema

additional_required_fields:
  phase:
    type: ExecutionPhase
    enum: [Init, Parse, Validate, Execute, Cleanup, Rollback]

  status:
    type: ExecutionStatus
    enum: [Started, InProgress, Completed, Failed, Cancelled]

  duration_ms:
    type: Option<u64>
    description: Execution duration in milliseconds (null if not completed)

additional_optional_fields:
  memory_usage_mb:
    type: f64
    description: Peak memory usage in megabytes

  cpu_usage_percent:
    type: f64
    description: Average CPU usage percentage
```

---

## 📋 Part 4: Folder Structure (Refined)

### Complete OASM Structure

```
oasm/
├── manifests/                        # Central registry
│   ├── master_manifest.yaml          # EXTENDED with asm-formats
│   ├── oasm_manifest.yaml
│   └── shell_profile.yaml
│
├── crates/                           # Rust crates
│   ├── oasm-api/                     # External API (existing)
│   ├── oasm-core/                    # Internal core (existing)
│   └── asm-formats/                  # Multi-format system (NEW)
│       ├── Cargo.toml
│       ├── ARCHITECTURE.md
│       └── src/
│           ├── lib.rs                # Core types (RunId, Seq, Actor)
│           ├── schemas.rs            # Format schemas
│           ├── converters.rs         # Format conversions
│           ├── domains.rs            # Domain-specific schemas
│           ├── templates.rs          # HDF5 template management
│           ├── runtime.rs            # CBOR runtime objects
│           └── lineage.rs            # JSON lineage tracking
│
├── compiler/                         # Compilation (existing)
│   └── src/
│       ├── main.rs
│       ├── diagnostics.rs
│       ├── cli_dashboard.rs
│       ├── scanner.rs
│       └── bin/
│           ├── oasm-scan.rs
│           └── oasm-phase1.rs
│
├── runtime/daemon/                   # Runtime daemon (existing)
│   └── src/
│       ├── manifest_loader.rs
│       └── ...
│
├── shells/                           # Shell integration
│   ├── oasm-shell/                   # Interactive shell (existing)
│   └── psmodule/                     # PowerShell module (existing)
│       ├── OASM.PowerShell.psd1
│       ├── OASM.PowerShell.psm1
│       └── README.md
│
├── ui/                               # User interfaces
│   ├── rust_ui/                      # Rust UI (existing)
│   └── python_ui_plugins/            # Python plugins (existing)
│
├── templates/                        # Template library (EXPANDED)
│   ├── schemas/                      # Schema definitions
│   │   ├── [existing 8 schemas]
│   │   └── [7 new asm-formats schemas]
│   ├── command_blocks/               # NEW: Batched execution units
│   │   ├── repair/
│   │   ├── lint/
│   │   ├── test/
│   │   └── cad/
│   ├── rules/                        # NEW: Rule hierarchy
│   │   ├── core/
│   │   ├── domain/
│   │   │   ├── rust/
│   │   │   ├── python/
│   │   │   ├── powershell/
│   │   │   └── cad/
│   │   ├── project/
│   │   └── session/
│   ├── scripts/                      # Existing code generation
│   ├── commands/                     # Existing diagnostic blocks
│   ├── workflows/                    # Existing CAD automation
│   └── scans/                        # Existing scan templates
│
├── schemas/                          # Validation schemas (existing)
│   ├── oasm.schema.yaml
│   └── oasm_protocol.cddl
│
├── config/                           # Runtime config (existing)
│   ├── runtime.yaml
│   └── oasm.default.yaml
│
├── logs/                             # Logging output (EXPANDED)
│   ├── StructureDebug/               # Existing scanner outputs
│   ├── provenance/                   # NEW: Provenance logs
│   ├── lineage/                      # NEW: Lineage audit trail
│   ├── execution/                    # NEW: Runtime execution logs
│   └── working_copies/               # NEW: Working copy metadata
│
├── artifacts/                        # NEW: Generated artifacts
│   ├── hdf5/                         # Immutable artifact store
│   │   ├── templates/
│   │   ├── baselines/
│   │   └── snapshots/
│   ├── working/                      # Working copies (ephemeral)
│   └── diffs/                        # Diff snapshots
│
├── scripts/                          # Scripts (existing)
│   ├── PS/                           # PowerShell scripts
│   │   ├── Invoke-OasmScan.ps1
│   │   ├── Get-OasmManifest.ps1
│   │   └── Invoke-Phase1.ps1
│   └── py/                           # Python scripts
│
├── examples/                         # Examples (existing)
│   ├── cad_gear.oasm
│   ├── engine_scene.oasm
│   └── test_module.ps1
│
├── docs/                             # Documentation
│   ├── COMPLETE_SYSTEM_SUMMARY.md    # Existing
│   ├── README.md                     # Existing
│   ├── MODULAR_ARCHITECTURE.md       # Existing
│   ├── INTEGRATION_PLAN.md           # This document
│   └── WPSHELL_2026_DOCKER_PLAN.md   # Future (Phase 2)
│
├── bin/                              # Native libraries (existing)
│   ├── core_math/
│   ├── ipc_bridge/
│   ├── validation/
│   └── plugin_loader/
│
├── environments/                     # Python environments (existing)
│   └── coding/
│
└── Cargo.toml                        # Workspace manifest
```

---

## 📋 Part 5: wpshell 2026 + Docker Build Plan (Future Phase)

**Status:** PLANNED - Execute After OASM Core Completion

### Prerequisites (Must Be Complete First)

- [x] OASM core compilation working
- [x] asm-formats crate complete and tested
- [ ] Command block system functional
- [ ] Template system validated
- [ ] All logging schemas implemented
- [ ] Root manifest and folder structure finalized
- [ ] Rule hierarchy implemented

### Stripdown → Weave → Layer Philosophy

**Phase 1: Stripdown (Identify what to keep/remove)**

**Keep (Essential Substrate):**
- Process execution: `Start-Process`, `Invoke-Command`
- File system navigation: `cd`, `ls`, `dir`, `Get-ChildItem`
- Environment variables: `Get-Item Env:`, `Set-Item Env:`
- Basic piping and redirection
- Minimal scripting engine: Execute `.ps1`, `.cmd` files

**Remove (Legacy Bloat):**
- Legacy modules: Printing, MSMQ, legacy networking
- Full registry manipulation (keep minimal read access)
- GUI automation cmdlets
- Legacy remoting: Older WSMan, DCOM
- Heavy management packs: Exchange, SharePoint, AD modules

**Phase 2: Weave (Rust Cockpit Integration)**

**Rust Crates to Weave In:**
```
wpshell-cockpit/
├── asm_core          # Parser, diagnostics, CFG/DFG
├── asm_formats       # HDF5/CBOR/YAML/JSON
├── asm_provenance    # Lineage tracking
├── asm_diff          # Diff generation, rollback
└── shell_substrate   # Minimal PS/CMD operations

wpshell-ffi/
├── ps_to_rust_bridge   # PowerShell → Rust callbacks
├── rust_to_ps_cmdlets  # Rust → PowerShell cmdlet exposure
└── provenance_hooks    # Auto-logging on every command
```

**PyO3 Bridge:**
```
py-bridge/
├── pattern_lab       # ML clustering, rule suggestions
├── semi_fixers       # Macro parameterization
└── analysis_viz      # CFG/DFG visualization
```

**Phase 3: Layer Back (Refined Modules)**

| Module | Stripped | Layer Back (Rust) | Layer Back (PS) | Skip |
|--------|----------|-------------------|-----------------|------|
| Process execution | ✓ | | | |
| File navigation | ✓ | | | |
| Env vars | ✓ | | | |
| Basic piping | ✓ | | | |
| Compiler integration | | ✓ (orchestrator) | ✓ (wrapper) | |
| Test harnesses | | ✓ (core) | ✓ (runner) | |
| Provenance logging | | ✓ (automatic) | | |
| Diff management | | ✓ (CBOR-based) | | |
| Registry manipulation | | | | ✓ |
| Printing (legacy) | | | | ✓ |
| MSMQ | | | | ✓ |
| Security emulator | | ✓ (WSL sidecar) | | |

**Phase 4: Docker Container Build**

**Base Image Selection:**
- Option A: Windows Server Core container (native PowerShell)
- Option B: Linux + PowerShell Core (lighter weight)

**Container Contents:**
```dockerfile
# Base: Windows Server Core or Linux with PS Core
FROM mcr.microsoft.com/windows/servercore:ltsc2022  # or linux base

# Install stripped PowerShell 2026
COPY wpshell-minimal/ /wpshell/

# Copy Rust binaries
COPY target/release/wpshell-cockpit.exe /oasm/bin/
COPY target/release/asm-*.dll /oasm/bin/

# Copy Python environment
COPY py-bridge/ /oasm/py/
COPY requirements.txt /oasm/py/

# Copy OASM templates and manifests
COPY templates/ /oasm/templates/
COPY manifests/ /oasm/manifests/

# Setup environment
ENV OASM_ROOT=/oasm
ENV WPSHELL_ROOT=/wpshell

# Entry point
CMD ["wpshell-cockpit.exe", "--interactive"]
```

**Phase 5: Testing Protocol (In Container Only)**

```powershell
# Inside Docker container

# Test 1: Stripped PowerShell basics
cd /test
ls
$env:TEST = "value"
Start-Process notepad  # Should work

# Test 2: Rust cockpit command blocks
oasm execute --template repair_unsafe --target src/main.rs

# Test 3: PyO3 bridge
oasm analyze --module pattern_lab --file src/lib.rs

# Test 4: Provenance logging
oasm history --lineage-id abc123
oasm revert --lineage-id abc123

# Test 5: Full OASM test suite
oasm test --suite full --output logs/
```

**Success Criteria:**
- Container builds without errors
- Stripped PowerShell executes essential commands
- Rust cockpit executes command blocks from templates
- PyO3 bridge operational (pattern_lab, semi_fixers, analysis_viz)
- Provenance logging works automatically
- Full OASM test suite passes in container
- No local environment corruption during testing

**Rollback Plan:**
- Docker container destroyed if tests fail
- Local environment untouched until explicit deployment decision
- All testing artifacts logged for analysis

---

## 📋 Part 6: Immediate Action Items (Priority Order)

### Phase 1: Foundation (Do Now)

1. **Extend Master Manifest**
   - File: `manifests/master_manifest.yaml`
   - Action: Add 7 new schemas from asm-formats
   - Action: Add asm-formats module to module registry
   - Status: Ready to implement

2. **Create Logging Schema Files**
   - Files:
     - `schemas/logging_base_schema.yaml`
     - `schemas/provenance_log_schema.yaml`
     - `schemas/lineage_log_schema.yaml`
     - `schemas/execution_log_schema.yaml`
   - Action: Write YAML schema definitions
   - Status: Ready to implement

3. **Create Directory Structure**
   - Directories:
     - `logs/provenance/`
     - `logs/lineage/`
     - `logs/execution/`
     - `logs/working_copies/`
     - `artifacts/hdf5/`
     - `artifacts/working/`
     - `artifacts/diffs/`
     - `templates/command_blocks/`
     - `templates/rules/`
   - Status: Ready to implement

4. **Create Template Structure**
   - Files:
     - `templates/command_blocks/repair/*.yaml`
     - `templates/command_blocks/lint/*.yaml`
     - `templates/command_blocks/test/*.yaml`
     - `templates/rules/core/*.yaml`
     - `templates/rules/domain/rust/*.yaml`
   - Status: Ready to implement

### Phase 2: Integration (After Foundation)

5. **Update Manifest Loader**
   - File: `runtime/daemon/src/manifest_loader.rs`
   - Action: Add support for loading asm-formats schemas
   - Action: Add schema hierarchy resolution (Session → Project → Domain → Core)

6. **Connect asm-formats to Existing Modules**
   - Compiler: Output to HDF5 templates
   - Scanner: Reference HDF5 baselines
   - Runtime daemon: Use CBOR objects from asm-formats

7. **Implement Command Block System**
   - File: `crates/asm-formats/src/command_blocks.rs` (new)
   - Action: Command block builder
   - Action: Template loader
   - Action: Batch execution

### Phase 3: Testing (After Integration)

8. **Test asm-formats Integration**
   - Test: HDF5 → CBOR conversion
   - Test: CBOR → YAML overlay generation
   - Test: YAML → CBOR validation
   - Test: CBOR → JSON lineage recording
   - Test: Copy-on-work pattern
   - Test: Command block batching

9. **Test Logging Schemas**
   - Test: Provenance log writing
   - Test: Lineage chain tracking
   - Test: Execution log capture
   - Test: Revert command generation

10. **Test Template System**
    - Test: Template discovery
    - Test: Command block loading
    - Test: Rule hierarchy resolution

### Phase 4: Documentation (After Testing)

11. **Update Documentation**
    - Update: `COMPLETE_SYSTEM_SUMMARY.md` with asm-formats
    - Update: `MODULAR_ARCHITECTURE.md` with template-first philosophy
    - Create: `LOGGING_GUIDE.md` for new logging schemas
    - Create: `TEMPLATE_GUIDE.md` for template authoring

### Phase 5: Future (Post-OASM Completion)

12. **wpshell 2026 Docker Build**
    - Document: Create `WPSHELL_2026_DOCKER_PLAN.md`
    - Research: PowerShell module stripping strategy
    - Design: FFI bridge architecture
    - Prototype: Minimal Docker container
    - Test: In-container validation
    - Deploy: Only after thorough testing

---

## 📋 Part 7: Module vs Template Decisions

### Analysis of Existing Components

| Component | Current | LOC | Frequency | Decision |
|-----------|---------|-----|-----------|----------|
| runtime_daemon | Module | 500+ | Always running | **Keep as module** |
| compiler | Module | 1000+ | High (compilation) | **Keep as module** |
| scanner | Module | 800+ | High (diagnostics) | **Keep as module** |
| oasm_shell | Module | 400+ | User-initiated | **Keep as module** |
| rust_ui | Module | 600+ | User-initiated | **Keep as module** |
| bindings | Module | 300+ | High (DLL loading) | **Keep as module** |
| asm-formats | Module | 1000+ | High (format system) | **Keep as module** |
| diagnostics | Module | 300+ | High (error tracking) | **Keep as module** |
| cli_dashboard | Module | 400+ | High (output formatting) | **Keep as module** |

### New Components (Template Candidates)

| Component | Type | LOC Est. | Frequency | Decision |
|-----------|------|----------|-----------|----------|
| Repair unsafe blocks | Task | ~50 | Low | **Template** |
| Add error handling | Task | ~60 | Low | **Template** |
| Fix imports | Task | ~40 | Low | **Template** |
| Add logging | Task | ~50 | Low | **Template** |
| Check formatting | Task | ~30 | Medium | **Template** |
| Unit test runner | Task | ~80 | Medium | **Template** |
| Topology validation (CAD) | Task | ~90 | Low | **Template** |
| Export STEP (CAD) | Task | ~70 | Low | **Template** |
| Rule validators | Task | ~50 each | Low | **Template** |

**Summary:**
- **9 permanent modules** (existing + asm-formats)
- **~50+ templates** (command blocks, rules, validators)

---

## 📋 Part 8: Success Criteria

### Phase 1 Complete When:
- [x] asm-formats crate builds and tests pass
- [ ] Master manifest extended with 7 new schemas
- [ ] Logging schema YAML files created
- [ ] Directory structure created (logs/, artifacts/, templates/ subdirs)
- [ ] Template structure created (command_blocks/, rules/)

### Phase 2 Complete When:
- [ ] Manifest loader supports asm-formats schemas
- [ ] Schema hierarchy resolution implemented
- [ ] Command block system functional
- [ ] Compiler outputs to HDF5
- [ ] Scanner references HDF5 baselines
- [ ] Runtime daemon uses CBOR objects

### Phase 3 Complete When:
- [ ] All asm-formats conversion tests pass
- [ ] Provenance logging validated
- [ ] Lineage chain tracking validated
- [ ] Template discovery working
- [ ] Rule hierarchy resolution validated

### Phase 4 Complete When:
- [ ] Documentation updated
- [ ] Template authoring guide complete
- [ ] Logging guide complete

### Phase 5 (Future) Complete When:
- [ ] wpshell 2026 Docker plan documented
- [ ] Docker container builds successfully
- [ ] All in-container tests pass
- [ ] Ready for local deployment (user decision)

---

## 📋 Part 9: Risk Mitigation

### Risk 1: HDF5 Library Dependency
**Mitigation:** HDF5 support is optional feature flag, system works without it using CBOR/YAML/JSON only

### Risk 2: Template Complexity Creep
**Mitigation:** Enforce < 100 LOC limit, code review for template additions

### Risk 3: Rule Hierarchy Conflicts
**Mitigation:** Clear priority order (Session → Project → Domain → Core), conflict resolution documented

### Risk 4: Docker Container Size
**Mitigation:** Multi-stage build, minimal base image, stripped PowerShell

### Risk 5: Local Environment Corruption
**Mitigation:** Docker-first testing, no local deployment until thorough validation

### Risk 6: Performance Overhead (Logging)
**Mitigation:** Async logging, configurable log levels, log rotation

---

## 📋 Part 10: Timeline Estimate

**Phase 1 (Foundation):** 1-2 days
- Extend manifest, create schemas, create directories

**Phase 2 (Integration):** 3-5 days
- Update manifest loader, connect asm-formats to modules, implement command blocks

**Phase 3 (Testing):** 2-3 days
- Test conversions, logging, templates

**Phase 4 (Documentation):** 1-2 days
- Update docs, create guides

**Phase 5 (Future - wpshell 2026 Docker):** 2-4 weeks (future phase)
- Research, design, build, test in container

**Total for Phases 1-4:** ~1-2 weeks
**Total including Phase 5:** ~3-6 weeks (when initiated)

---

## 🎯 Summary

This plan:
1. ✅ Integrates asm-formats with existing OASM architecture
2. ✅ Establishes template-first modularity (< 100 LOC philosophy)
3. ✅ Creates hierarchical logging schemas (Base → Provenance → Lineage → Execution)
4. ✅ Refines folder structure with proper nesting
5. ✅ Documents wpshell 2026 Docker build for future phase
6. ✅ Provides clear action items with priority order
7. ✅ Defines success criteria for each phase
8. ✅ Mitigates risks with clear strategies

**Next Step:** Review this plan, then begin Phase 1 (Foundation) implementation.

---

**Document Version:** 1.0
**Last Updated:** 2025-12-18
**Status:** Ready for Review

# OASM Data Format Architecture

## Overview

The `asm-formats` crate implements the multi-layer data format architecture for OASM, maintaining strict separation of concerns across HDF5, CBOR, YAML, and JSON formats.

## Format Hierarchy & Relationships

```
┌─────────────────────────────────────────────────────────────┐
│                    HDF5 (Immutable Canonical)                │
│  • Deep artifacts (CFG, DFG, symbol tables)                 │
│  • Large datasets (test fixtures, baselines)                │
│  • Never modified after creation                            │
│  • Queried for depth, rollback                              │
└──────────────┬──────────────────────────────────────────────┘
               │ extract template
               ▼
┌─────────────────────────────────────────────────────────────┐
│                  CBOR (Runtime Execution)                    │
│  • Binary, compact, deterministic                           │
│  • Fast IPC between daemons                                 │
│  • Immutable once created for a run                         │
│  • Ephemeral (discarded after execution)                    │
└──────────────┬──────────────────────────────────────────────┘
               │ execute → outcome
               ▼
┌─────────────────────────────────────────────────────────────┐
│                 JSON (Lineage & Audit)                       │
│  • Git-friendly format                                      │
│  • Execution outcomes and provenance                        │
│  • Persistent audit trail                                   │
│  • References HDF5/CBOR by ID (not duplicate)               │
└─────────────────────────────────────────────────────────────┘

               Human overlay ↓
┌─────────────────────────────────────────────────────────────┐
│           YAML (Annotated Human Overlay)                     │
│  • Mirrors CBOR structure with comments                     │
│  • Auto-populated fields                                    │
│  • Validated → converted → executed as CBOR                 │
│  • Ephemeral (annotations logged in JSON)                   │
└──────────────┬──────────────────────────────────────────────┘
               │ validate & convert
               ▼
              CBOR
```

## Conversion Rules (STRICT)

### 1. HDF5 → CBOR
- **Purpose**: Extract template, generate runtime object
- **Rule**: Deep artifacts (CFG/DFG, datasets) **STAY in HDF5**
- **Data**: Only metadata and execution parameters go to CBOR
- **Reference**: Artifacts referenced by `data_path`, not embedded

### 2. HDF5 → YAML
- **Purpose**: Generate human-readable overlay
- **Rule**: Deep artifacts **referenced by path**, not embedded
- **Data**: Auto-populated fields + annotations
- **Format**: Comments explain rationale

### 3. YAML → CBOR
- **Purpose**: Validate and convert for execution
- **Rule**: Validate structure, strip comments
- **Data**: Compact binary, annotations logged separately
- **Outcome**: Ready for execution

### 4. CBOR → JSON Lineage
- **Purpose**: Record execution outcome
- **Rule**: Reference CBOR/HDF5 by ID, don't duplicate
- **Data**: Provenance, impact, test results
- **Persistence**: Audit trail persists

### 5. FORBIDDEN Conversions
- ❌ CBOR → HDF5 (HDF5 is immutable canonical)
- ❌ JSON → HDF5 (JSON is audit trail, not source)
- ❌ Large datasets → YAML/JSON (use HDF5 references)

## Execution Flow

### Primary Flow: HDF5 → CBOR → Execute → JSON

```rust
// 1. Load immutable template from HDF5
let template = template_store.load_template("template_001")?;

// 2. Generate CBOR runtime object (no deep artifacts)
let cbor_obj = converter.hdf5_to_cbor("template_001", run_id, seq, actor)?;

// 3. Execute CBOR object
let result = runtime_manager.execute(&cbor_obj)?;

// 4. Record outcome in JSON lineage
let lineage = converter.cbor_to_json_lineage(&cbor_obj, result.outcome, impact)?;

// 5. CBOR object discarded (ephemeral)
// 6. Lineage persists
```

### YAML Role: Template/Documentation (NOT per-task editing)

**IMPORTANT**: YAML is a **read-only template** that explains structure and usage.
- It's NOT filled out per-task
- It's NOT edited dynamically
- It serves as **documentation** of what command blocks can do

### Command Block Batching

Lightweight functions are **grouped into command blocks** and executed together:

```rust
// Batch multiple operations into a single command block
let command_block = CommandBlockBuilder::new(BlockType::RepairBlock)
    .target_file("src/main.rs")
    .target_file("src/lib.rs")
    .rule("fix_imports")
    .rule("fix_unsafe")
    .rule("add_error_handling")
    .build();

// Create CBOR object (batched execution unit)
let cbor_obj = runtime_manager.create_object(run_id, seq, actor, command_block);

// Execute entire batch together
let result = runtime_manager.execute(&cbor_obj)?;

// Result covers all batched operations
let lineage = converter.cbor_to_json_lineage(&cbor_obj, result.outcome, impact)?;
```

**Benefits of Batching**:
- Multiple fixes applied atomically
- All tests run together
- Single provenance record for related operations
- Efficient IPC (one CBOR object, not N)
- Rollback entire batch if any operation fails

## Data Structures

### HDF5Template
- `template_id`: Unique identifier
- `template_type`: AssemblerPass, CompilerStage, TestHarness, etc.
- `artifacts[]`: Deep state (CFG, DFG, datasets)
- `baseline`: Snapshot metrics
- **Storage**: Immutable, never modified

### CBORRuntimeObject
- `object_id`: Run + sequence identifier
- `metadata`: ExecutionMetadata (run_id, seq, actor, timestamps)
- `command`: CommandBlock (lightweight, no deep artifacts)
- `auto_fields`: Auto-populated by system
- `decisions`: User popup decisions
- **Lifecycle**: Created → Executed → Discarded

### YAMLOverlay
- `comment`: Human explanation
- `metadata`: ExecutionMetadata
- `command`: CommandBlock (same as CBOR)
- `auto_populated`: System-filled fields
- `annotations[]`: Explanations and rationale
- **Lifecycle**: Created → Validated → Converted to CBOR → Discarded

### JSONLineage
- `lineage_id`: Unique audit record
- `run_id`, `seq`: Execution identifiers
- `actor`: Who/what performed the action
- `summary`, `intent`: Human-readable description
- `outcome`: Success/Failed/PartialSuccess/Cancelled
- `provenance`: Tool versions, config hash, lineage chain
- `impact`: Files/lines changed, functions affected
- `tests[]`: Test execution records
- `diff_id`, `git_sha`: Links to diffs and commits
- **Persistence**: Permanent audit trail

## Auto-Populated Fields

System automatically populates:
- `run_id`, `seq`, `timestamp`
- `actor`, `file_path`, `rule_group`
- `tool_versions`, `config_hash`
- `tests_planned`, `impact` metrics
- `confidence` (for AI/automation)

## Popup Decision Points

When automation encounters ambiguity, ephemeral popups request:
- Encoding choice (UTF-8 vs ASCII)
- Duplicate handling (Erase | EvaluateDiff | Ignore)
- Language/runtime choice (C++ | Python | Rust)
- Confidence below threshold confirmations

Decisions logged in JSON lineage for audit trail.

## Benefits

### Immutable Depth (HDF5)
- Canonical templates never modified
- Deep artifacts preserved
- Rollback capability
- Reproducible baselines

### Runtime Truth (CBOR)
- Fast, deterministic execution
- Compact binary for IPC
- No parsing overhead
- Type-safe schemas

### Readable Overlay (YAML)
- Humans understand intent
- Comments explain rationale
- Easy to author/review
- Validated before execution

### Audit Trail (JSON)
- Git-friendly diffs
- Permanent provenance
- Lineage chains
- Reproducible history

## Modules

- `lib.rs`: Core types (RunId, Seq, Actor, Confidence, Impact)
- `schemas.rs`: Format-specific schemas (HDF5, CBOR, YAML, JSON)
- `templates.rs`: HDF5 template management
- `runtime.rs`: CBOR runtime objects
- `lineage.rs`: JSON lineage tracking
- `converters.rs`: Format conversion with strict rules

## Future Enhancements

- [ ] Full HDF5 integration (requires HDF5 C library)
- [ ] Compression support for diffs (gzip/zstd)
- [ ] Retention policies (hot/warm/cold storage)
- [ ] Session indexing for batch operations
- [ ] Visual diff previews
- [ ] Git integration for lineage

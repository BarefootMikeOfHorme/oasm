# OASM Modular Architecture

**Assembly Module for wpshell - Complete Implementation**

## Overview

OASM is now properly structured as a **wpshell module** with a complete modular architecture:

- **External API** (`oasm-api`) - For programs running WITH OASM
- **Internal Core** (`oasm-core`) - Modules, blocks, rules, instruction parsing
- **PowerShell Module** - wpshell integration with cmdlets
- **Examples** - Assembly code examples for different program types

## Crate Structure

### 1. oasm-api (External API)

**Location:** `crates/oasm-api/`

**Purpose:** Public API for external programs (CAD editors, engines, etc.) to integrate with OASM.

**Key Types:**
```rust
pub enum ProgramType {
    CAD, Engine, Document, Compression, Debug, Custom(String)
}

pub struct OasmContext { ... }
pub enum Instruction { CREATE, DEFINE, SET, EXTRUDE, MOVE, ... }
pub enum Value { Integer, Float, String, Boolean, Array }
```

**API Functions:**
- `register_program(name, program_type)` - Register a program with OASM
- `execute(instruction, context)` - Execute an instruction
- `parse(source)` - Parse OASM assembly into instructions

### 2. oasm-core (Internal Core)

**Location:** `crates/oasm-core/`

**Purpose:** Internal implementation - modules, blocks, rules, instruction parsing/execution.

**Modules:**
- `modules.rs` - Module system (CAD, Engine, Document, etc.)
- `blocks.rs` - Program-specific instruction blocks
- `rules.rs` - Validation and constraint rules
- `instructions.rs` - Assembly parser and executor

**Key Types:**
```rust
pub struct Module { id, name, module_type, location, capabilities }
pub struct Block { id, program_type, instructions, rules, optimizations }
pub struct Rule { id, program_type, category, conditions }
```

**Registries:**
- `ModuleRegistry` - Manages available modules
- `BlockRegistry` - Manages instruction blocks per program type
- `RuleEngine` - Validates and enforces rules

## PowerShell Module (wpshell Integration)

**Location:** `shells/psmodule/`

**Files:**
- `OASM.PowerShell.psd1` - Module manifest
- `OASM.PowerShell.psm1` - Module implementation
- `README.md` - Full documentation

**Cmdlets:**

| Cmdlet | Purpose |
|--------|---------|
| `Initialize-Oasm` | Initialize OASM for current session |
| `New-OasmContext` | Create context for program type |
| `Invoke-OasmAssembly` | Execute OASM assembly code |
| `Get-OasmRules` | Get rules for program type |
| `Get-OasmBlocks` | Get blocks for program type |
| `Invoke-OasmCompile` | Compile manifest |
| `Invoke-OasmScan` | Pre-compile diagnostics |
| `Start-OasmDaemon` | Start runtime daemon |

**Example Usage:**
```powershell
# Initialize
Initialize-Oasm

# Create CAD context
$ctx = New-OasmContext -ProgramType CAD

# Execute assembly
$source = @'
CREATE gear
  SET teeth = 20
EXTRUDE z_axis, 10
VALIDATE topology
EXPORT step, output/gear.step
'@

Invoke-OasmAssembly -Source $source -Context $ctx
```

## Assembly Language Syntax

### Instructions by Category

**Object Creation:**
```
CREATE object_type
  SET parameter = value
DEFINE name
  SET setting = value
```

**Transformations:**
```
MOVE object, [x, y, z]
ROTATE object, [rx, ry, rz]
SCALE object, factor
EXTRUDE direction, distance
```

**Analysis:**
```
SCAN metric
  TRACK value
VALIDATE check_type
```

**Export:**
```
EXPORT format, path
```

## Program-Specific Blocks

### CAD Blocks
1. **Primitives Block**
   - Instructions: CREATE, EXTRUDE, FILLET, CHAMFER
   - Rules: geometric_validation, parametric_constraints
   - Optimizations: parallel_primitive_creation, batch_operations

2. **Export Block**
   - Instructions: EXPORT, VALIDATE
   - Rules: export_compatibility
   - Optimizations: compression

### Engine Blocks
1. **Scene Block**
   - Instructions: CREATE, ATTACH, SET
   - Rules: scene_graph_validation, physics_constraints
   - Optimizations: entity_pooling, batch_rendering

### Document Blocks
1. **Content Block**
   - Instructions: INSERT, APPLY, EXPORT
   - Rules: structure_validation, style_consistency
   - Optimizations: incremental_rendering

## Rules System

### CAD Rules
- `cad_topology_validation` - Validates edge connectivity, face closure, no self-intersections
- `cad_parameter_range` - Validates parameters are in recommended bounds

### Engine Rules
- `engine_scene_graph_validation` - Validates no circular references in scene graph

### Document Rules
- `document_structure_validation` - Validates document hierarchy

## Examples

**Location:** `examples/`

- `cad_gear.oasm` - Parametric gear creation
- `engine_scene.oasm` - Game engine scene setup
- `document_report.oasm` - Document generation
- `test_module.ps1` - PowerShell module test suite

## Integration Points

### For wpshell
- Auto-loads via `shell_profile.yaml`
- Provides assembly-like control for programs
- Integration with PS Insider 2026

### For External Programs
```rust
use oasm_api::{ProgramType, register_program, Instruction, execute};

// Register your program
let mut ctx = register_program("MyCAD", ProgramType::CAD)?;

// Execute instructions
let instr = Instruction::Create {
    object_type: "gear".to_string(),
    params: ...
};
execute(instr, &mut ctx)?;
```

### For PowerShell
```powershell
Import-Module OASM.PowerShell
Initialize-Oasm
$ctx = New-OasmContext -ProgramType CAD
Invoke-OasmAssembly -Source $assembly -Context $ctx
```

## Build and Test

### Build All Crates
```bash
cd crates/oasm-api && cargo build
cd crates/oasm-core && cargo build
cd ../.. && cargo build --workspace
```

### Test PowerShell Module
```powershell
cd examples
.\test_module.ps1
```

### Run Scanner
```powershell
Invoke-OasmScan -Path . -Output logs\scan
```

## Architecture Benefits

✅ **Modular** - Separate API and core implementation
✅ **Extensible** - Easy to add new program types, blocks, rules
✅ **wpshell Native** - PowerShell cmdlets for seamless integration
✅ **Assembly-like** - Low-level control with high-level abstractions
✅ **Type-safe** - Rust ensures safety and performance
✅ **Growing** - Rule system expands as needed per program type

## Next Steps

1. **Implement Ratatui Terminal** - Blazing-fast TUI for program output
2. **Expand Rule Library** - Add more program-specific rules
3. **Native Execution** - Connect instruction executor to actual operations
4. **Template Expansion** - Build out template library
5. **wpshell Profile** - Deep integration with custom wpshell

---

**OASM is now a complete wpshell module providing assembly-like enhancement for any program type.**

# OASM.PowerShell Module

**OASM assembly module for wpshell**

This PowerShell module provides assembly-like control for programs running on the OASM platform. Like NASM/MASM for assembly, but for high-level programs (CAD, engines, documents, etc.).

## Installation

### For wpshell

```powershell
# Module auto-loads via shell_profile.yaml
# Or manually import:
Import-Module .\shells\psmodule\OASM.PowerShell.psd1
```

### For PowerShell 2026 Insider

```powershell
# Install to user modules
$modulePath = "$env:USERPROFILE\Documents\PowerShell\Modules\OASM.PowerShell"
Copy-Item .\shells\psmodule\* $modulePath -Recurse
Import-Module OASM.PowerShell
```

## Quick Start

```powershell
# Initialize OASM
Initialize-Oasm

# Create a context for CAD program
$ctx = New-OasmContext -ProgramType CAD

# Execute assembly code
$source = @"
CREATE gear
  SET teeth = 20
  SET module = 2.5
EXTRUDE z_axis, 10
VALIDATE topology
EXPORT step, "output/gear.step"
"@

Invoke-OasmAssembly -Source $source -Context $ctx
```

## Cmdlets

### Initialize-Oasm
Initialize the OASM assembly module for the current session.

```powershell
Initialize-Oasm
```

### New-OasmContext
Create a new OASM context for a program type.

**Program Types:**
- `CAD` - CAD editors (Objex, etc.)
- `Engine` - Game/physics engines
- `Document` - Word processors, document generators
- `Compression` - Compressors, archivers
- `Debug` - Debuggers, loggers

```powershell
$ctx = New-OasmContext -ProgramType CAD
```

### Invoke-OasmAssembly
Execute OASM assembly code.

```powershell
$source = @"
CREATE gear
  SET teeth = 20
"@
Invoke-OasmAssembly -Source $source -Context $ctx
```

### Get-OasmRules
Get rules for a program type.

```powershell
Get-OasmRules -ProgramType CAD
# Returns: cad_topology_validation, cad_parameter_range, etc.
```

### Get-OasmBlocks
Get blocks (instruction sets + rules) for a program type.

```powershell
Get-OasmBlocks -ProgramType CAD
# Returns: cad_primitives_block, cad_export_block, etc.
```

### Invoke-OasmCompile
Compile an OASM manifest.

```powershell
Invoke-OasmCompile -ManifestPath .\manifests\test.yaml
```

### Invoke-OasmScan
Run OASM scanner on a directory for pre-compile diagnostics.

```powershell
Invoke-OasmScan -Path . -Output logs\scan
```

### Start-OasmDaemon
Start the OASM runtime daemon (supervisor loop).

```powershell
Start-OasmDaemon
```

## Assembly Language

OASM uses an assembly-like syntax:

### Instructions

**Object Creation:**
```
CREATE object_type
  SET parameter = value
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
VALIDATE check_type
```

**Export:**
```
EXPORT format, "path"
```

### Example - CAD Gear

```oasm
; Create parametric gear
CREATE gear
  SET teeth = 20
  SET module = 2.5
  SET pressure_angle = 20

; Operations
EXTRUDE z_axis, 10
FILLET edges[0..3], 2

; Validation
VALIDATE topology
VALIDATE constraints

; Export
EXPORT step, "output/gear.step"
```

### Example - Engine Scene

```oasm
; Create scene
CREATE scene "main_level"
  CREATE entity "player"
    SET position = [0, 0, 0]
    ATTACH component "rigidbody"

; Performance scan
SCAN profiler
  TRACK entity_count
  TRACK frame_time
```

## Program-Specific Blocks

Each program type has specific instruction blocks:

### CAD
- **Primitives Block:** CREATE, EXTRUDE, FILLET, CHAMFER
- **Export Block:** EXPORT, VALIDATE

### Engine
- **Scene Block:** CREATE, ATTACH, SET
- **Physics Block:** DEFINE constraints, APPLY forces

### Document
- **Content Block:** INSERT, APPLY, EXPORT
- **Formatting Block:** STYLE, LAYOUT

## Integration with wpshell

OASM is designed to work with:
- Custom wpshell
- PowerShell Insider 2026
- Insider cmd

The module provides:
- ✅ Assembly-like instruction set
- ✅ Program-specific rules and blocks
- ✅ Modular and growing rule system
- ✅ Pre-compile scanning and diagnostics
- ✅ Manifest-based configuration

## Architecture

```
OASM Module (wpshell)
├── oasm-api (Rust)        → External API for programs
├── oasm-core (Rust)       → Internal modules, blocks, rules
├── OASM.PowerShell (.ps1) → PowerShell interface
└── Examples (.oasm)       → Assembly examples
```

## See Also

- `OASM_VISION.md` - Full vision and architecture
- `MANIFEST_SYSTEM.md` - Manifest system documentation
- `examples/` - Example assembly files

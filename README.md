# OASM - Objex AsSeMbly Module

**Assembly-like enhancement module for wpshell - Like NASM/MASM, but for ANY program**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![PowerShell](https://img.shields.io/badge/powershell-7.0+-blue.svg)](https://docs.microsoft.com/en-us/powershell/)

OASM provides **assembly-like control and enhancement** for any program running on the OASM setup: CAD editors, game engines, word processors, compressors, debuggers, and more.

## 🚀 Features

- ✅ **Assembly-like instruction set** - CREATE, SET, EXTRUDE, VALIDATE, EXPORT, SCAN
- ✅ **Program-specific blocks** - Tailored instruction sets for different program types
- ✅ **Modular rule system** - Growing library of validation and constraint rules
- ✅ **wpshell integration** - Native PowerShell cmdlets
- ✅ **Pre-compile diagnostics** - Advanced codebase scanning
- ✅ **Manifest system** - Easy module/file/settings location
- ✅ **Rust + PowerShell** - Type-safe core with scripting flexibility

## 🎯 Quick Start

### PowerShell Module

```powershell
# Import OASM module
Import-Module .\shells\psmodule\OASM.PowerShell.psd1

# Initialize
Initialize-Oasm

# Create CAD context
$ctx = New-OasmContext -ProgramType CAD

# Execute assembly
$source = @'
CREATE gear
  SET teeth = 20
  SET module = 2.5
EXTRUDE z_axis, 10
VALIDATE topology
EXPORT step, output/gear.step
'@

Invoke-OasmAssembly -Source $source -Context $ctx
```

## 🔧 Assembly Language

### CAD Example
```asm
; Create parametric gear
CREATE gear
  SET teeth = 20
  SET module = 2.5

; Operations
EXTRUDE z_axis, 10
FILLET edges[0..3], 2

; Validation
VALIDATE topology

; Export
EXPORT step, "output/gear.step"
```

## 📚 Program Types

| Program Type | Blocks | Rules |
|--------------|--------|-------|
| **CAD** | Primitives, Export | Topology validation |
| **Engine** | Scene, Physics | Scene graph validation |
| **Document** | Content, Formatting | Structure validation |
| **Compression** | Algorithms | Format detection |
| **Debug** | Breakpoints, Profiling | Performance rules |

## 🛠️ Build

```bash
cargo build --workspace
cargo build --bin oasm-scan
```

## 📖 PowerShell Cmdlets

- `Initialize-Oasm` - Initialize OASM
- `New-OasmContext` - Create program context
- `Invoke-OasmAssembly` - Execute assembly
- `Get-OasmRules` - Get rules
- `Invoke-OasmScan` - Run scanner

## 📂 Project Structure

```
oasm/
├── crates/oasm-api/       # External API
├── crates/oasm-core/      # Internal core
├── compiler/              # Manifest compiler
├── runtime/daemon/        # Supervisor
├── shells/psmodule/       # PowerShell module
├── examples/              # Assembly examples
└── manifests/             # Manifest system
```

## 📜 Documentation

- [OASM Vision](OASM_VISION.md)
- [Manifest System](MANIFEST_SYSTEM.md)
- [Modular Architecture](MODULAR_ARCHITECTURE.md)
- [PowerShell Module](shells/psmodule/README.md)

---

**OASM: Assembly-like enhancement for every program** ⚡

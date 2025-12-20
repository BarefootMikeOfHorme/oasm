# OASM Executive Function AI Powerhouse

**Dual-Purpose System: CAD Assistant + General Productivity Automation**

## Overview

OASM (Objex AsSeMbly) is a sophisticated executive function support system that combines:
- **CAD Workflows**: SmartObject container management for Objex parametric CAD
- **AI Productivity**: Universal automation, scanning, and template-based workflows

## Core Features

### 1. Template Library System ✅

Located in `templates/`:

- **Schemas**: SmartObject containers, validation rules, manifests
- **Scripts**: Code generation templates (PowerShell, Python, Rust)
- **Commands**: Diagnostic blocks, health checks, automation
- **Workflows**: CAD design flows, executive function pathways
- **Scans**: Deep codebase analysis, metrics collection

### 2. Universal Scanner ✅

**Pre-compile diagnostic tool** that works on ANY project:

```bash
# Rust CLI
cargo run --bin oasm-scan <project_root>

# PowerShell wrapper (PS 2026 compatible)
.\scripts\PS\Invoke-OasmScan.ps1 -Root "C:\Path\To\Project"
```

**Generates:**
- `structure_{timestamp}.log` - Human-readable tree view
- `baseline_index_{timestamp}.json` - Structured metrics data
- `cli_state_{timestamp}.json` - Compilation status

**Metrics Collected:**
- Lines of code (LOC)
- Function counts (total, public, unsafe)
- Import/dependency analysis
- Logging usage (info, warn, error, println)
- Struct/enum/derive counts
- Test coverage estimates
- Last modified timestamps
- Importance scoring

### 3. Modern CLI Output ✅

PowerShell wrapper provides:
- ✨ Color-coded output (headers, success, errors, warnings)
- 📊 Progress bars with percentage indicators
- 🏥 Health indicators (blocking issues, warnings, unsafe code)
- 📈 Metrics summary with visual formatting
- 💡 AI assistance hints (PS 2026 integration points)

### 4. Executive Function Features

All components support:
- **Progress Tracking**: Visual indicators and completion percentages
- **Task Breakdown**: Clear phases with discrete steps
- **Error Recovery**: Helpful suggestions when things fail
- **Checkpoints**: Auto-save and resume capability
- **Reminders**: Last action, next steps, time estimates
- **Working Memory Support**: Command history, task counters
- **Cognitive Load Reduction**: Structured prompts, clear error messages

### 5. Shell Layer (OASM Shell) ✅

Interactive shell with executive function support:

```bash
oasm-shell
```

**Features:**
- Numbered prompts track task progression
- History command for working memory recall
- Status command shows tasks executed
- Capability-based security system
- Clear error messages with recovery suggestions
- Built-in help and command discovery

### 6. Workflow System

**Preset Workflows:**
- `cad_automation_preset.yaml` - Full CAD design cycle
- `project_health_check.yaml` - Quick diagnostics

**User-Defined:**
- Store in `~/.oasm/templates/` for personal workflows
- YAML-based with validation
- Conditional execution and phase gates

## Integration Points

### PS 2026 / wpshell
- AI-assisted command completion
- Autonomous workflow execution
- Context-aware suggestions
- Smart error handling

### Objex CAD
- SmartObject container management
- CBOR serialization + YAML mirrors
- Validation gates
- Lineage tracking
- Export to STEP/STL/DXF

## File Structure

```
oasm/
├── templates/               # Template library
│   ├── schemas/            # SmartObject, validation templates
│   ├── scripts/            # Code generation templates
│   ├── commands/           # Command block templates
│   ├── workflows/          # Workflow presets
│   └── scans/              # Analysis templates
├── compiler/
│   └── src/
│       ├── scanner.rs      # Universal scanner implementation
│       └── bin/
│           └── oasm-scan.rs # CLI tool
├── runtime/daemon/         # Supervisor loop
├── shells/oasm-shell/      # Interactive shell
├── ui/rust_ui/             # UI components
├── scripts/PS/             # PowerShell wrappers
│   └── Invoke-OasmScan.ps1
├── config/                 # Runtime configuration
└── logs/StructureDebug/    # Scan output
```

## Usage Examples

### 1. Scan Any Project

```powershell
# From project root
.\scripts\PS\Invoke-OasmScan.ps1 -Verbose

# Specific directory
.\scripts\PS\Invoke-OasmScan.ps1 -Root "C:\Projects\MyApp" -Format json
```

### 2. CAD Workflow

```bash
# Start interactive shell
oasm-shell

# Or use workflow template
oasm workflow start templates/workflows/cad_automation_preset.yaml
```

### 3. Create SmartObject

```bash
oasm create gear \
  --from-template templates/schemas/smartobject_container.yaml \
  --param teeth=20 \
  --param module=2.5
```

### 4. Generate Script from Template

```bash
oasm script generate \
  --template templates/scripts/script_template_generator.yaml \
  --language powershell \
  --name my_automation
```

## Executive Function Pathways

### Preset (System-Defined)
1. **CAD Design** → Design → Validate → Serialize → Export
2. **Code Health** → Scan → Check → Test → Report
3. **Automation** → Template → Generate → Execute → Monitor

### User-Defined
Users can create custom pathways for their specific needs:
- Project setup automation
- Data processing pipelines
- Report generation workflows
- Custom diagnostic routines

## AI Integration (PS 2026)

The system provides hooks for AI-enhanced features:
- **Code Analysis**: AI-powered insights from scan data
- **Workflow Suggestions**: Based on project state
- **Error Resolution**: AI-assisted debugging
- **Optimization Recommendations**: Performance improvements
- **Documentation Generation**: Auto-docs from code structure

## Health Monitoring

Scanner provides health indicators:
- ⚠️ **Blocking Issues**: Critical problems preventing compilation
- ⚡ **Warnings**: Non-critical issues to address
- 🔒 **Unsafe Code**: Unsafe block usage tracking
- 🧪 **Test Coverage**: Estimated test/function ratio

## Output Formats

All tools support multiple output formats:
- **JSON**: Machine-readable, structured data
- **YAML**: Human-readable with comments
- **Log**: Formatted text for viewing
- **CBOR**: Binary format for high performance

## Next Steps

1. **Build the scanner**: `cargo build --bin oasm-scan`
2. **Run first scan**: `.\scripts\PS\Invoke-OasmScan.ps1`
3. **Explore templates**: Review `templates/` directory
4. **Customize workflows**: Add to `~/.oasm/templates/`
5. **Integrate with wpshell**: Connect to PS 2026 features

## Benefits

**For Executive Function:**
- Reduces cognitive load through structure
- Provides working memory support
- Offers clear next-step guidance
- Enables checkpoint/resume workflows
- Gives visual progress feedback

**For CAD Work:**
- Validates designs before export
- Tracks design lineage
- Ensures consistency across exports
- Provides audit trails
- Supports parametric workflows

**For Development:**
- Pre-compile diagnostics
- Code health monitoring
- Metrics tracking over time
- Template-based automation
- Cross-project analysis

---

**OASM**: Executive function support meets parametric CAD, powered by templates and AI.

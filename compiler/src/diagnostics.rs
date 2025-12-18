use std::fmt;
use std::path::PathBuf;
use crate::cli_dashboard::{DashboardRow, Totals};

/// Severity level for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
            Severity::Hint => write!(f, "hint"),
        }
    }
}

/// Source location information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl SourceLocation {
    pub fn new(file: PathBuf, line: usize, column: usize, length: usize) -> Self {
        Self { file, line, column, length }
    }

    pub fn unknown() -> Self {
        Self {
            file: PathBuf::from("<unknown>"),
            line: 0,
            column: 0,
            length: 0,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// Diagnostic code for categorization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    // Syntax errors (E0001-E0099)
    E0001, // Unexpected token
    E0002, // Missing token
    E0003, // Invalid syntax
    E0004, // Unclosed delimiter

    // Type errors (E0100-E0199)
    E0100, // Type mismatch
    E0101, // Undefined type
    E0102, // Invalid type conversion

    // Semantic errors (E0200-E0299)
    E0200, // Undefined symbol
    E0201, // Duplicate definition
    E0202, // Invalid operation
    E0203, // Undefined instruction

    // Module/Import errors (E0300-E0399)
    E0300, // Module not found
    E0301, // Circular dependency
    E0302, // Invalid module path

    // ARM/Architecture errors (E0400-E0499)
    E0400, // Unsupported instruction for target
    E0401, // Invalid register
    E0402, // Invalid addressing mode
    E0403, // Misaligned memory access

    // Manifest errors (E0500-E0599)
    E0500, // Invalid manifest format
    E0501, // Missing required field
    E0502, // Invalid field value

    // Warnings (W0001-W9999)
    W0001, // Unused variable
    W0002, // Deprecated feature
    W0003, // Unreachable code
    W0004, // Performance warning
}

impl DiagnosticCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            // Syntax
            DiagnosticCode::E0001 => "E0001",
            DiagnosticCode::E0002 => "E0002",
            DiagnosticCode::E0003 => "E0003",
            DiagnosticCode::E0004 => "E0004",
            // Type
            DiagnosticCode::E0100 => "E0100",
            DiagnosticCode::E0101 => "E0101",
            DiagnosticCode::E0102 => "E0102",
            // Semantic
            DiagnosticCode::E0200 => "E0200",
            DiagnosticCode::E0201 => "E0201",
            DiagnosticCode::E0202 => "E0202",
            DiagnosticCode::E0203 => "E0203",
            // Module
            DiagnosticCode::E0300 => "E0300",
            DiagnosticCode::E0301 => "E0301",
            DiagnosticCode::E0302 => "E0302",
            // ARM
            DiagnosticCode::E0400 => "E0400",
            DiagnosticCode::E0401 => "E0401",
            DiagnosticCode::E0402 => "E0402",
            DiagnosticCode::E0403 => "E0403",
            // Manifest
            DiagnosticCode::E0500 => "E0500",
            DiagnosticCode::E0501 => "E0501",
            DiagnosticCode::E0502 => "E0502",
            // Warnings
            DiagnosticCode::W0001 => "W0001",
            DiagnosticCode::W0002 => "W0002",
            DiagnosticCode::W0003 => "W0003",
            DiagnosticCode::W0004 => "W0004",
        }
    }

    pub fn default_severity(&self) -> Severity {
        match self.as_str().chars().next() {
            Some('E') => Severity::Error,
            Some('W') => Severity::Warning,
            _ => Severity::Info,
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A single diagnostic message
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    pub location: SourceLocation,
    pub suggestions: Vec<String>,
    pub related: Vec<RelatedDiagnostic>,
}

/// Related diagnostic information (for multi-location errors)
#[derive(Debug, Clone)]
pub struct RelatedDiagnostic {
    pub message: String,
    pub location: SourceLocation,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: impl Into<String>, location: SourceLocation) -> Self {
        Self {
            severity: code.default_severity(),
            code,
            message: message.into(),
            location,
            suggestions: Vec::new(),
            related: Vec::new(),
        }
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    pub fn with_related(mut self, message: impl Into<String>, location: SourceLocation) -> Self {
        self.related.push(RelatedDiagnostic {
            message: message.into(),
            location,
        });
        self
    }

    pub fn error(code: DiagnosticCode, message: impl Into<String>, location: SourceLocation) -> Self {
        let mut diag = Self::new(code, message, location);
        diag.severity = Severity::Error;
        diag
    }

    pub fn warning(code: DiagnosticCode, message: impl Into<String>, location: SourceLocation) -> Self {
        let mut diag = Self::new(code, message, location);
        diag.severity = Severity::Warning;
        diag
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: [{}] {}", self.severity, self.code, self.message)?;
        writeln!(f, "  --> {}", self.location)?;

        if !self.suggestions.is_empty() {
            writeln!(f, "Suggestions:")?;
            for suggestion in &self.suggestions {
                writeln!(f, "  - {}", suggestion)?;
            }
        }

        if !self.related.is_empty() {
            writeln!(f, "Related:")?;
            for related in &self.related {
                writeln!(f, "  {} at {}", related.message, related.location)?;
            }
        }

        Ok(())
    }
}

/// Collector for multiple diagnostics
#[derive(Debug, Default)]
pub struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn add_error(&mut self, code: DiagnosticCode, message: impl Into<String>, location: SourceLocation) {
        self.add(Diagnostic::error(code, message, location));
    }

    pub fn add_warning(&mut self, code: DiagnosticCode, message: impl Into<String>, location: SourceLocation) {
        self.add(Diagnostic::warning(code, message, location));
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Error).count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.severity == Severity::Warning).count()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn sort_by_severity(&mut self) {
        self.diagnostics.sort_by_key(|d| d.severity);
    }

    pub fn print_all(&self) {
        for diagnostic in &self.diagnostics {
            println!("{}", diagnostic);
        }
    }

    pub fn print_summary(&self) {
        let errors = self.error_count();
        let warnings = self.warning_count();

        if errors > 0 || warnings > 0 {
            println!("\nCompilation finished with {} error(s) and {} warning(s)", errors, warnings);
        } else {
            println!("\nCompilation finished successfully");
        }
    }

    /// Convert diagnostics to dashboard totals
    /// Maps Error -> crit, Warning -> warn, blocking errors -> block
    pub fn to_dashboard_totals(&self) -> Totals {
        let mut crit = 0;
        let mut block = 0;
        let mut warn = 0;

        for diag in &self.diagnostics {
            match diag.severity {
                Severity::Error => {
                    crit += 1;
                    // Check if it's a blocking error (syntax, type errors)
                    match diag.code {
                        DiagnosticCode::E0001 | DiagnosticCode::E0002 | DiagnosticCode::E0003
                        | DiagnosticCode::E0004 | DiagnosticCode::E0100 | DiagnosticCode::E0101 => {
                            block += 1;
                        }
                        _ => {}
                    }
                }
                Severity::Warning => warn += 1,
                _ => {}
            }
        }

        Totals::new(crit, block, warn)
    }

    /// Attach diagnostics to a dashboard row
    pub fn attach_to_dashboard(&self, row: &mut DashboardRow) {
        row.totals = self.to_dashboard_totals();
        row.diagnostics = self.diagnostics
            .iter()
            .map(|d| format!("[{}] {}", d.code, d.message))
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_creation() {
        let loc = SourceLocation::new(PathBuf::from("test.oasm"), 10, 5, 3);
        let diag = Diagnostic::new(DiagnosticCode::E0001, "Unexpected token", loc);

        assert_eq!(diag.severity, Severity::Error);
        assert_eq!(diag.code, DiagnosticCode::E0001);
        assert_eq!(diag.message, "Unexpected token");
    }

    #[test]
    fn test_diagnostic_bag() {
        let mut bag = DiagnosticBag::new();

        bag.add_error(
            DiagnosticCode::E0001,
            "Test error",
            SourceLocation::unknown()
        );

        bag.add_warning(
            DiagnosticCode::W0001,
            "Test warning",
            SourceLocation::unknown()
        );

        assert_eq!(bag.error_count(), 1);
        assert_eq!(bag.warning_count(), 1);
        assert!(bag.has_errors());
    }

    #[test]
    fn test_diagnostic_with_suggestions() {
        let loc = SourceLocation::new(PathBuf::from("test.oasm"), 10, 5, 3);
        let diag = Diagnostic::new(DiagnosticCode::E0200, "Undefined symbol 'foo'", loc)
            .with_suggestion("Did you mean 'bar'?")
            .with_suggestion("Check if the module is imported");

        assert_eq!(diag.suggestions.len(), 2);
    }
}

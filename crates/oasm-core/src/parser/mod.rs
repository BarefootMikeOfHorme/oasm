/// OASM Native Parser
/// Parses OASM's own instruction syntax (not assembly mnemonics)

use crate::types::Value;
use serde::{Deserialize, Serialize};

/// Parsed instruction (native OASM)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instruction {
    pub mnemonic: String,
    pub operands: Vec<Operand>,
    pub line_number: usize,
}

/// Operand types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Identifier(String),
    Literal(Value),
    Property { object: String, property: String },
    Array(Vec<Operand>),
    Assignment { target: String, value: Box<Operand> },
}

/// Parser trait
pub trait InstructionParser {
    fn parse_line(&self, line: &str, line_number: usize) -> Result<Option<Instruction>, ParseError>;
    fn parse_file(&self, source: &str) -> Result<Vec<Instruction>, ParseError>;
}

/// Parse errors
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken { line: usize, token: String },
    InvalidSyntax { line: usize, message: String },
    UnterminatedString { line: usize },
    InvalidNumber { line: usize, value: String },
}

/// Native OASM parser
pub struct NativeParser;

impl InstructionParser for NativeParser {
    fn parse_line(&self, line: &str, line_number: usize) -> Result<Option<Instruction>, ParseError> {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with(';') || trimmed.starts_with('#') {
            return Ok(None);
        }

        // Split into tokens
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.is_empty() {
            return Ok(None);
        }

        // First token is the mnemonic
        let mnemonic = tokens[0].to_uppercase();
        
        // Parse operands
        let operands = self.parse_operands(&tokens[1..], line_number)?;

        Ok(Some(Instruction {
            mnemonic,
            operands,
            line_number,
        }))
    }

    fn parse_file(&self, source: &str) -> Result<Vec<Instruction>, ParseError> {
        let mut instructions = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            if let Some(instr) = self.parse_line(line, line_num + 1)? {
                instructions.push(instr);
            }
        }

        Ok(instructions)
    }
}

impl NativeParser {
    fn parse_operands(&self, tokens: &[&str], line_number: usize) -> Result<Vec<Operand>, ParseError> {
        let mut operands = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            let token = tokens[i];

            // Skip commas
            if token == "," {
                i += 1;
                continue;
            }

            // Assignment: name = value
            if i + 2 < tokens.len() && tokens[i + 1] == "=" {
                let target = tokens[i].to_string();
                let value = self.parse_value(tokens[i + 2], line_number)?;
                operands.push(Operand::Assignment {
                    target,
                    value: Box::new(value),
                });
                i += 3;
                continue;
            }

            // Property access: object.property
            if token.contains('.') {
                let parts: Vec<&str> = token.split('.').collect();
                if parts.len() == 2 {
                    operands.push(Operand::Property {
                        object: parts[0].to_string(),
                        property: parts[1].to_string(),
                    });
                    i += 1;
                    continue;
                }
            }

            // Array: [1, 2, 3]
            if token.starts_with('[') {
                // TODO: Implement array parsing
                i += 1;
                continue;
            }

            // Otherwise, parse as value
            let operand = self.parse_value(token, line_number)?;
            operands.push(operand);
            i += 1;
        }

        Ok(operands)
    }

    fn parse_value(&self, token: &str, line_number: usize) -> Result<Operand, ParseError> {
        // String literal
        if token.starts_with('"') {
            if !token.ends_with('"') || token.len() < 2 {
                return Err(ParseError::UnterminatedString { line: line_number });
            }
            let s = token[1..token.len() - 1].to_string();
            return Ok(Operand::Literal(Value::String(s)));
        }

        // Boolean
        if token == "true" {
            return Ok(Operand::Literal(Value::Bool(true)));
        }
        if token == "false" {
            return Ok(Operand::Literal(Value::Bool(false)));
        }

        // Number
        if let Ok(n) = token.parse::<u32>() {
            return Ok(Operand::Literal(Value::U32(n)));
        }
        if let Ok(n) = token.parse::<f64>() {
            return Ok(Operand::Literal(Value::F64(n)));
        }

        // Otherwise, it's an identifier
        Ok(Operand::Identifier(token.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create() {
        let parser = NativeParser;
        let instr = parser.parse_line("CREATE gear", 1).unwrap().unwrap();
        
        assert_eq!(instr.mnemonic, "CREATE");
        assert_eq!(instr.operands.len(), 1);
        assert!(matches!(instr.operands[0], Operand::Identifier(_)));
    }

    #[test]
    fn test_parse_set() {
        let parser = NativeParser;
        let instr = parser.parse_line("SET teeth = 20", 1).unwrap().unwrap();
        
        assert_eq!(instr.mnemonic, "SET");
        assert_eq!(instr.operands.len(), 1);
        
        if let Operand::Assignment { target, value } = &instr.operands[0] {
            assert_eq!(target, "teeth");
            assert!(matches!(**value, Operand::Literal(Value::U32(20))));
        } else {
            panic!("Expected assignment operand");
        }
    }

    #[test]
    fn test_parse_property_access() {
        let parser = NativeParser;
        let instr = parser.parse_line("VALIDATE gear.topology", 1).unwrap().unwrap();
        
        assert_eq!(instr.mnemonic, "VALIDATE");
        assert_eq!(instr.operands.len(), 1);
        
        if let Operand::Property { object, property } = &instr.operands[0] {
            assert_eq!(object, "gear");
            assert_eq!(property, "topology");
        } else {
            panic!("Expected property operand");
        }
    }

    #[test]
    fn test_parse_file() {
        let parser = NativeParser;
        let source = r#"
CREATE gear
SET teeth = 20
SET module = 2.5
VALIDATE topology
"#;

        let instructions = parser.parse_file(source).unwrap();
        assert_eq!(instructions.len(), 4);
        assert_eq!(instructions[0].mnemonic, "CREATE");
        assert_eq!(instructions[1].mnemonic, "SET");
        assert_eq!(instructions[2].mnemonic, "SET");
        assert_eq!(instructions[3].mnemonic, "VALIDATE");
    }

    #[test]
    fn test_skip_comments() {
        let parser = NativeParser;
        let source = r#"
; This is a comment
CREATE gear
# Another comment
SET teeth = 20
"#;

        let instructions = parser.parse_file(source).unwrap();
        assert_eq!(instructions.len(), 2);
    }
}

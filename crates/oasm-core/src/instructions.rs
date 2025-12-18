/// Instruction parser and executor for OASM assembly

use std::collections::HashMap;

/// Instruction token
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(f64),
    String(String),
    Comma,
    Equals,
    LeftBracket,
    RightBracket,
    Newline,
}

/// Parse OASM assembly source into tokens
pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in source.chars() {
        match ch {
            ',' => {
                if !current.is_empty() {
                    tokens.push(classify_token(&current));
                    current.clear();
                }
                tokens.push(Token::Comma);
            }
            '=' => {
                if !current.is_empty() {
                    tokens.push(classify_token(&current));
                    current.clear();
                }
                tokens.push(Token::Equals);
            }
            '[' => {
                if !current.is_empty() {
                    tokens.push(classify_token(&current));
                    current.clear();
                }
                tokens.push(Token::LeftBracket);
            }
            ']' => {
                if !current.is_empty() {
                    tokens.push(classify_token(&current));
                    current.clear();
                }
                tokens.push(Token::RightBracket);
            }
            '\n' => {
                if !current.is_empty() {
                    tokens.push(classify_token(&current));
                    current.clear();
                }
                tokens.push(Token::Newline);
            }
            ' ' | '\t' => {
                if !current.is_empty() {
                    tokens.push(classify_token(&current));
                    current.clear();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        tokens.push(classify_token(&current));
    }

    tokens
}

fn classify_token(s: &str) -> Token {
    // Check if it's a number
    if let Ok(num) = s.parse::<f64>() {
        return Token::Number(num);
    }

    // Check if it's a string literal
    if s.starts_with('"') && s.ends_with('"') {
        return Token::String(s[1..s.len()-1].to_string());
    }

    // Check if it's a keyword
    let keywords = vec![
        "CREATE", "DEFINE", "SET", "EXTRUDE", "MOVE", "ROTATE", "SCALE",
        "VALIDATE", "SCAN", "EXPORT", "INSERT", "APPLY", "ATTACH",
    ];

    if keywords.contains(&s.to_uppercase().as_str()) {
        Token::Keyword(s.to_uppercase())
    } else {
        Token::Identifier(s.to_string())
    }
}

/// Instruction definition
#[derive(Debug, Clone)]
pub struct InstructionDef {
    pub opcode: String,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(String),
    Immediate(f64),
    Label(String),
    Array(Vec<f64>),
}

/// Parse tokens into instruction definitions
pub fn parse_instructions(tokens: &[Token]) -> Result<Vec<InstructionDef>, String> {
    let mut instructions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(kw) => {
                let mut operands = Vec::new();
                i += 1;

                // Collect operands until newline
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Newline => break,
                        Token::Comma => { i += 1; continue; }
                        Token::Number(n) => {
                            operands.push(Operand::Immediate(*n));
                            i += 1;
                        }
                        Token::Identifier(id) => {
                            operands.push(Operand::Label(id.clone()));
                            i += 1;
                        }
                        Token::String(s) => {
                            operands.push(Operand::Label(s.clone()));
                            i += 1;
                        }
                        _ => i += 1,
                    }
                }

                instructions.push(InstructionDef {
                    opcode: kw.clone(),
                    operands,
                });
            }
            Token::Newline => i += 1,
            _ => i += 1,
        }
    }

    Ok(instructions)
}

/// Execute a single instruction
pub fn execute_instruction(
    instruction: &InstructionDef,
    context: &mut HashMap<String, f64>
) -> Result<(), String> {
    match instruction.opcode.as_str() {
        "CREATE" => {
            // Create object logic
            Ok(())
        }
        "SET" => {
            // Set parameter logic
            if instruction.operands.len() >= 2 {
                if let (Operand::Label(name), Operand::Immediate(value)) =
                    (&instruction.operands[0], &instruction.operands[1]) {
                    context.insert(name.clone(), *value);
                }
            }
            Ok(())
        }
        "VALIDATE" => {
            // Validation logic
            Ok(())
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let source = "CREATE gear\nSET teeth = 20";
        let tokens = tokenize(source);
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_parse_simple() {
        let source = "CREATE gear\nSET teeth = 20";
        let tokens = tokenize(source);
        let instructions = parse_instructions(&tokens).unwrap();
        assert_eq!(instructions.len(), 2);
        assert_eq!(instructions[0].opcode, "CREATE");
        assert_eq!(instructions[1].opcode, "SET");
    }
}

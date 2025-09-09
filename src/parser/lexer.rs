use crate::parser::token::{Token, TokenKind};
use crate::util::Location;
use std::iter::Peekable;
use std::str::Chars;
use std::string::ToString;

/// Map of reserved keywords
fn keyword_to_token_kind(word: &str) -> Option<TokenKind> {
    match word {
        "fn" => Some(TokenKind::FnKeyword),
        "let" => Some(TokenKind::LetKeyword),
        "Unit" => Some(TokenKind::TyUnit),
        _ => None,
    }
}

#[derive(PartialEq)]
enum LexerState {
    Start,
    ReadingId,
}

/// On-demand lexer
pub struct Lexer<'a> {
    source: &'a str,
    src_iter: Peekable<Chars<'a>>,
    pos: Location,
    offset: usize,
    state: LexerState,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            src_iter: source.chars().peekable(),
            pos: (0, 0),
            offset: 0,
            state: LexerState::Start,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    /// Get next token in input stream
    fn next(&mut self) -> Option<Self::Item> {
        let prev_offset = self.offset;

        loop {
            let char = self.src_iter.next()?;
            match char {
                _ if char.is_alphabetic() || char.is_alphanumeric() => {
                    let peek = self.src_iter.peek().unwrap_or(&'\n');
                    if peek.is_whitespace() || *peek == '(' || *peek == ')' || *peek == ':' {
                        self.offset += 1;
                        self.pos.0 += 1;

                        let word = self.source[prev_offset..self.offset].trim();

                        return if let Ok(numeric) = word.parse::<i32>() {
                            Some(Token::new(self.pos, numeric.to_string(), TokenKind::TyInt))
                        } else if let Some(keyword_kind) = keyword_to_token_kind(word) {
                            Some(Token::new(self.pos, word.to_string(), keyword_kind))
                        } else {
                            Some(Token::new(self.pos, word.to_string(), TokenKind::Id))
                        };
                    } else {
                        self.offset += 1;
                        self.pos.0 += 1;
                    }
                }
                '(' => {
                    self.offset += 1;
                    self.pos.0 += 1;
                    return Some(Token::new(self.pos, String::from("("), TokenKind::LParen));
                }
                ')' => {
                    self.offset += 1;
                    self.pos.0 += 1;
                    return Some(Token::new(self.pos, String::from(")"), TokenKind::RParen));
                }
                '{' => {
                    self.offset += 1;
                    self.pos.0 += 1;
                    return Some(Token::new(self.pos, String::from("{"), TokenKind::LCurly));
                }
                '}' => {
                    self.offset += 1;
                    self.pos.0 += 1;
                    return Some(Token::new(self.pos, String::from("}"), TokenKind::RCurly));
                }
                '=' => {
                    self.offset += 1;
                    self.pos.0 += 1;
                    return Some(Token::new(self.pos, String::from("="), TokenKind::Eq));
                }
                '\n' => {
                    self.offset += 1;
                    self.pos.0 = 0;
                    self.pos.1 += 1;
                }
                '-' => {
                    self.pos.0 += 1;
                    self.offset += 1;

                    let peek = self.src_iter.peek().unwrap_or(&'\n');

                    if *peek == '>' {
                        self.pos.0 += 1;
                        self.offset += 1;
                        self.src_iter.next();
                        return Some(Token::new(self.pos, String::from("->"), TokenKind::Arrow));
                    }

                    return Some(Token::new(self.pos, String::from("-"), TokenKind::Minus));
                }
                '+' => {
                    self.pos.0 += 1;
                    self.offset += 1;

                    return Some(Token::new(self.pos, String::from("+"), TokenKind::Plus));
                }
                ' ' => {
                    self.offset += 1;
                    self.pos.0 += 1;
                }
                ':' => {
                    self.offset += 1;
                    self.pos.0 += 1;

                    return Some(Token::new(self.pos, String::from(":"), TokenKind::Colon));
                }
                _ => panic!("Unknown token at {:?}", self.pos),
            }
        }
    }
}

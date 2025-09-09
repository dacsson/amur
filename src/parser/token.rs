use crate::util::{Location, Spannable};

#[derive(Debug)]
pub enum TokenKind {
    ErrorToken,
    Eof,

    LParen,
    RParen,
    LCurly,
    RCurly,
    Eq,
    Comma,
    Colon,
    Arrow,
    Plus,
    Minus,
    Star,
    Slash,

    FnKeyword,
    LetKeyword,

    Id,
    TyInt,
    TyUnit,

    Unknown,
}

#[derive(Debug)]
pub struct Token {
    loc: Location, // token position in src file
    value: String,
    kind: TokenKind,
}

impl Token {
    pub fn new(loc: Location, value: String, kind: TokenKind) -> Self {
        Self { loc, value, kind }
    }
}

impl Spannable for Token {
    fn get_loc(&self) -> Location {
        self.loc
    }
}

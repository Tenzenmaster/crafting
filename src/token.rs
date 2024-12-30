use std::{
    iter::Peekable,
    str::Chars,
};

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    String,
    Number,
    Identifier,

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub source: &'a str,
    pub line: u32,
}

#[derive(Debug)]
pub struct Tokens<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        self.start = self.current;

        use TokenKind::*;
        let kind = match self.advance()? {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ';' => Semicolon,
            ',' => Comma,
            '.' => Dot,
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '=' => if self.advance_if_eq('=') { EqualEqual } else { Equal },
            '!' => if self.advance_if_eq('=') { BangEqual } else { Bang },
            '>' => if self.advance_if_eq('=') { GreaterEqual } else { Greater },
            '<' => if self.advance_if_eq('=') { LessEqual } else { Less },
            '"' => match self.string() {
                Ok(kind) => kind,
                Err(err) => return Some(Err(err)),
            },
            ch if ch.is_numeric() => self.number(),
            ch if ch.is_alphabetic() || ch == '_' => self.word(),
            _ => return Some(Err("Unexpected character")),
        };

        Some(Ok(self.make_token(kind)))
    }
}

impl<'a> Tokens<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(ch) => {
                if ch == '\n' {
                    self.line += 1;
                }
                self.current += ch.len_utf8();
                Some(ch)
            },
            None => None,
        }
    }

    fn advance_if_eq(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(ch) if ch == expected => {
                self.advance();
                true
            },
            _ => false,
        }
    }

    fn advance_while(&mut self, cond: fn(char) -> bool) -> bool {
        loop {
            match self.peek() {
                Some(ch) if !cond(ch) => return true,
                None => return false,
                _ => self.advance(),
            };
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn skip_whitespace(&mut self) {
        // TODO: skip comments
        self.advance_while(|ch| ch.is_whitespace());
    }

    fn string(&mut self) -> Result<TokenKind, &'static str> {
        if self.advance_while(|ch| ch != '"') {
            Ok(TokenKind::String)
        } else {
            Err("Unterminated String")
        }
    }

    fn number(&mut self) -> TokenKind {
        self.advance_while(|ch| ch.is_numeric());
        if self.peek() == Some('.') {
            self.advance();
            self.advance_while(|ch| ch.is_numeric());
        }

        TokenKind::Number
    }

    fn word(&mut self) -> TokenKind {
        self.advance_while(|ch| ch.is_alphanumeric() || ch == '_');

        use TokenKind::*;
        match self.slice() {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Identifier,
        }
    }

    fn slice(&self) -> &'a str {
        &self.source[self.start..self.current]
    }

    fn make_token(&self, kind: TokenKind) -> Token<'a> {
        Token {
            kind,
            source: self.slice(),
            line: self.line,
        }
    }
}

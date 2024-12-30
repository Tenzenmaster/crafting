use crate::*;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Compiler<'a> {
    tokens: Peekable<Tokens<'a>>,
    chunk: Chunk,
}

impl<'a> Compiler<'a> {
    pub fn new(tokens: Tokens<'a>) -> Self {
        Self {
            tokens: tokens.peekable(),
            chunk: Chunk::new(),
        }
    }

    pub fn compile(mut self) -> Result<Chunk, &'static str> {
    }
}

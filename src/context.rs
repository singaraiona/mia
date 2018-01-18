use stack::Stack;
use mia::{Error, AST, symbol_to_str};

pub struct Context {
    pub stack: Stack,
}

impl Context {
    pub fn new() -> Self {
        let mut c = Context { stack: Stack::new() };
        c.init_builtin_symbols();
        c
    }

    pub fn insert_entry(&mut self, sym: usize, ast: AST) { self.stack.insert(sym, ast); }

    pub fn entry(&self, sym: usize) -> Result<&AST, Error> {
        Ok(self.stack.entry(sym))
        //.ok_or_else(|| undef_error!(symbol_to_str(sym)))
    }

    pub fn push_frame(&mut self) { self.stack.push_frame() }

    pub fn pop_frame(&mut self) { self.stack.pop_frame() }

    fn init_builtin_symbol(&mut self, sym: &str, ast: AST) {
        let id = sym!(sym).symbol();
        self.insert_entry(id, ast)
    }

    fn init_builtin_symbols(&mut self) {
        self.init_builtin_symbol("NIL",  NIL!());
        self.init_builtin_symbol("T",    T!());
        self.init_builtin_symbol("@",    NIL!());
    }
}


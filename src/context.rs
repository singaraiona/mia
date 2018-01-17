use stack::Stack;
use mia::AST;

pub struct Context {
    pub syms: Vec<String>,
    pub stack: Stack,
}

impl Context {
    pub fn new() -> Self {
        let mut c = Context { syms: Vec::with_capacity(256), stack: Stack::new() };
        c.init_builtin_symbols();
        c
    }

    pub fn new_symbol(&mut self, sym: String) -> usize {
        for (i, x) in self.syms.iter().enumerate() { if *x == sym { return i; } }
        self.syms.push(sym);
        self.syms.len() - 1
    }

    pub fn symbol_to_str(&self, sym: usize) -> &str { self.syms[sym].as_str() }

    pub fn insert_entry(&mut self, sym: usize, ast: AST) { self.stack.insert(sym, ast); }

    pub fn entry(&self, sym: usize) -> Result<&AST, Error> {
        self.stack.entry(sym).ok_or_else(|| undef_error!(symbol_to_str(sym)))
    }

    pub fn push_frame(&mut self) { self.stack.push_frame() }

    pub fn pop_frame(&mut self) { self.stack.pop_frame() }

    fn init_builtin_symbol(&mut self, sym: &str, ast: AST) { self.insert_entry(sym!(sym).symbol(), ast) }

    fn init_builtin_symbols(&mut self) {
        self.init_builtin_symbol("NIL",  NIL!());
        self.init_builtin_symbol("T",    T!());
        self.init_builtin_symbol("@",    NIL!());
    }
}


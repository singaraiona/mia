use mia::*;
use fnv::FnvHashMap;

struct Frame(Vec<AST>);

impl Frame {
    fn new() -> Self { Frame(vec![NIL!();256]) }

    fn insert(&mut self, key: usize, val: AST) { self.0[key] = val; }

    fn entry(&self, key: usize) -> &AST { &self.0[key] }
}

pub struct Stack(Vec<Frame>);

impl Stack {
    pub fn new() -> Self { Stack(vec![Frame::new()]) }

    pub fn push_frame(&mut self) { self.0.push(Frame::new()) }

    pub fn pop_frame(&mut self) { self.0.pop(); }

    pub fn insert(&mut self, key: usize, val: AST) { self.last().insert(key, val); }

    pub fn entry(&self, key: usize) -> &AST {
        &self.0[self.0.len() -1].0[key]
        //for e in self.0.iter().rev() {
            //let r = e.entry(key);
            //if !r.is_nil() { return r }
        //}
        //None
    }

    // Assume we always have at least one frame.
    fn last(&mut self) -> &mut Frame { let l = self.0.len() - 1; &mut self.0[l] }
}

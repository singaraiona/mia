use mia::*;
use fnv::FnvHashMap;

struct Frame(FnvHashMap<usize, AST>);

impl Frame {
    fn new() -> Self { Frame(FnvHashMap::with_capacity_and_hasher(10000, Default::default())) }

    fn insert(&mut self, key: usize, val: AST) { self.0.insert(key, val); }

    fn entry(&self, key: usize) -> Option<AST> { self.0.get(&key).map(|a| a.clone()) }
}

pub struct Stack(Vec<Frame>);

impl Stack {
    pub fn new() -> Self { Stack(vec![Frame::new()]) }

    pub fn push_frame(&mut self) { self.0.push(Frame::new()) }

    pub fn pop_frame(&mut self) { self.0.pop(); }

    pub fn insert(&mut self, key: usize, val: AST) { self.last().insert(key, val); }

    pub fn entry(&mut self, key: usize) -> Option<AST> { self.last().entry(key).or_else(|| self.0[0].entry(key)) }

    // Assume we always have at least one frame.
    fn last(&mut self) -> &mut Frame { let l = self.0.len() - 1; &mut self.0[l] }
}
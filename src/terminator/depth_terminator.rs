use super::Terminator;

pub struct DepthTerminator(usize);

impl DepthTerminator {
    pub fn new(max_depth: usize) -> Self {
        Self(max_depth)
    }
}

impl Terminator for DepthTerminator {
    fn terminate(&self, depth: usize) -> bool {
        depth >= self.0
    }
}
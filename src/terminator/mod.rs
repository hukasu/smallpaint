mod depth_terminator;
pub use depth_terminator::DepthTerminator;

pub trait Terminator: std::marker::Sync {
    fn terminate(&self, depth: usize) -> bool;
    fn factor(&self, depth: usize) -> f64;
}
mod depth_terminator;
pub use depth_terminator::DepthTerminator;

mod russian_roulette_terminator;
pub use russian_roulette_terminator::RussianRouletteTerminator;

pub trait Terminator: std::marker::Sync {
    fn terminate(&self, depth: usize) -> bool;
    fn factor(&self, depth: usize) -> f64;
}
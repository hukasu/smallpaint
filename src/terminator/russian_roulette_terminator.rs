use crate::common::RandomGen;

use super::*;

pub struct RussianRouletteTerminator {
    roulette_start_depth: usize,
    stop_probability: f64 
}

impl RussianRouletteTerminator {
    pub fn new(
        roulette_start_depth: usize,
        stop_probability: f64
    ) -> Self {
        Self {
            roulette_start_depth,
            stop_probability
        }
    }
}

impl Terminator for RussianRouletteTerminator {
    fn terminate(&self, depth: usize) -> bool {
        if depth >= self.roulette_start_depth && RandomGen::rand2() <= self.stop_probability {
            true
        } else {
            false
        }
    }

    fn factor(&self, depth: usize) -> f64 {
        if depth >= self.roulette_start_depth {
            1. / ( 1. - self.stop_probability)
        } else {
            1.
        }
    }
}
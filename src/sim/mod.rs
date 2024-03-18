use std::any::Any;
use std::fmt::Debug;

use crate::data::{Molecule, Puzzle, Solution};

mod parts;

// Data types

#[derive(Clone, Debug)]
pub struct Sim{
    pub parts: Vec<AnyPart>,
    pub molecules: Vec<Molecule>
}

impl Sim{
    pub fn create(puzzle: &Puzzle, solution: &Solution) -> Result<Sim, &'static str>{
        let sol_clean = puzzle.clean_solution(solution)?;
        Ok(Sim{
            parts: Vec::new(),
            molecules: Vec::new()
        })
    }
}

// Parts

/// A part "in flight", storing its own relevant state.
pub trait SimPart: Any + Debug{
    /// Act on the board for this cycle.
    fn tick(&mut self, s: &mut Sim, is_cycle_start: bool);

    /// Make a boxed clone of this part.
    fn clone_boxed(&self) -> AnyPart;
}

pub type AnyPart = Box<dyn SimPart>;

impl Clone for AnyPart{
    fn clone(&self) -> Self{
        self.clone_boxed()
    }
}
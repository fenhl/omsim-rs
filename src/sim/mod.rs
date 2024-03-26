use std::fmt::Debug;

use crate::data::{Atom, HexIndex, Molecule, Part, PartType, Puzzle, Solution};

// Data types

#[derive(Clone, Debug)]
pub struct Sim{
    pub parts: Vec<SimPart>,
    pub molecules: Vec<SimMolecule>
}

// it's like Molecule but we copy less and offset more
#[derive(Clone, Debug)]
pub struct SimMolecule{
    pub layout: Molecule,
    pub pos: HexIndex,
    pub rotation: u8,
    pub grabbed: bool
}

impl SimMolecule{
    pub fn contains_pos(&self, pos: HexIndex) -> bool{
        self.layout.contains_pos(pos - self.pos)
    }

    pub fn atom_at(&self, pos: HexIndex) -> Atom{
        self.layout.atoms[&(pos - self.pos)]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AtomLookupResult<'a>{
    pub atom_ty: Atom,
    pub molecule: &'a SimMolecule
}

impl Sim{
    pub fn create(puzzle: &Puzzle, solution: &Solution) -> Result<Sim, &'static str>{
        let sol_clean = puzzle.clean_solution(solution)?;
        Ok(Sim{
            parts: Vec::new(),
            molecules: Vec::new()
        })
    }

    pub fn lookup_atom<T>(&self, pos: HexIndex, f: impl for<'a> FnOnce(AtomLookupResult<'a>) -> T) -> Option<T>{
        for molecule in &self.molecules{
            if molecule.contains_pos(pos){
                return Some(f(AtomLookupResult{
                    atom_ty: molecule.atom_at(pos),
                    molecule: &molecule
                }))
            }
        }

        None
    }
}

// Parts

#[derive(Clone, Debug)]
pub struct SimPart{
    pub pos: HexIndex,
    pub rotation: u8,
    pub ty: SimPartType
}

#[derive(Clone, Debug)]
pub enum SimPartType{
    Input(Molecule),
    Output(Molecule, u64)
}

impl SimPart{
    pub fn from_solution_part(part: &Part, puzzle: &Puzzle, solution: &Solution) -> SimPart{
        SimPart{
            pos: part.pos,
            rotation: (part.rotation.unsigned_abs() % 6) as u8,
            ty: SimPartType::from_solution_part(part, puzzle, solution)
        }
    }

    pub fn tick(&mut self, sim: &mut Sim){
        match &mut self.ty{
            SimPartType::Input(m) => {}
            SimPartType::Output(m, outputs) => {
                for atom in &m.atoms{

                }
            }
        }
    }
}

impl SimPartType{
    pub fn from_solution_part(part: &Part, puzzle: &Puzzle, solution: &Solution) -> SimPartType{
        match part.ty{
            PartType::Input => SimPartType::Input(puzzle.reagents[part.index as usize].clone()),
            PartType::Output => SimPartType::Output(puzzle.products[part.index as usize].clone(), 0),
            _ => unimplemented!()
        }
    }
}
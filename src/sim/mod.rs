mod collision;

use std::fmt::Debug;

use crate::data::{Atom, Bond, HexIndex, HexRotation, Molecule, Part, PartType, Puzzle, Solution};

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
    pub grabbed: bool
}

impl SimMolecule{
    pub fn contains_pos(&self, pos: HexIndex) -> bool{
        self.layout.contains_pos(pos - self.pos)
    }

    pub fn atom_at(&self, pos: HexIndex) -> Atom{
        self.layout.atoms[&(pos - self.pos)]
    }

    pub fn bond_at(&self, pos_a: HexIndex, pos_b: HexIndex) -> Option<Bond>{
        todo!()
    }

    // also wrong, should accept other molecule's position and rotation
    pub fn is(&self, m: &Molecule) -> bool{
        // if two molecules have the same number of atoms, bond layout, & the first contains all the atoms of the latter, they're the same
        if self.layout.atoms.len() != m.atoms.len(){
            return false
        }

        if self.layout.bonds != m.bonds{
            return false;
        }

        for atom in &m.atoms{
            if !self.contains_pos(*atom.0) || self.atom_at(*atom.0) != *atom.1{
                return false
            }
        }

        return true;
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
            parts: sol_clean.parts.iter().map(|p| SimPart::from_solution_part(p, puzzle, solution)).collect::<Result<Vec<_>, _>>()?,
            molecules: Vec::new()
        })
    }

    // need a way to remove or modify the molecule (or schedule those)
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
    pub rotation: HexRotation,
    pub ty: SimPartType
}

#[derive(Clone, Debug)]
pub enum SimPartType{
    Input(Molecule),
    Output(Molecule, u64),
    Arms,
    Track,
    Bonding, MultiBonding, Unbonding, Calcification,
    Animismus,
    Projection, Purification,
    Conduit,
}

impl SimPart{
    pub fn from_solution_part(part: &Part, puzzle: &Puzzle, solution: &Solution) -> Result<SimPart, &'static str>{
        Ok(SimPart{
            pos: part.pos,
            rotation: HexRotation::from_signed(part.rotation),
            ty: SimPartType::from_solution_part(part, puzzle, solution)?
        })
    }

    pub fn tick(&mut self, sim: &mut Sim){
        match &mut self.ty{
            SimPartType::Input(m) => {}
            SimPartType::Output(m, outputs) => {
                // we need exactly 1 molecule that touches the output everywhere
                // so we can just lookup for an arbitrary position (here the centre)
                sim.lookup_atom(self.pos, |result| {
                    if result.molecule.is(m){
                        // wrong
                        todo!()
                    }
                });
            }
            _ => panic!("a")
        }
    }
}

impl SimPartType{
    pub fn from_solution_part(part: &Part, puzzle: &Puzzle, solution: &Solution) -> Result<SimPartType, &'static str>{
        Ok(match part.ty{
            PartType::Input => SimPartType::Input(puzzle.reagents[part.index as usize].clone()),
            PartType::Output => SimPartType::Output(puzzle.products[part.index as usize].clone(), 0),
            PartType::Arm | PartType::BiArm | PartType::TriArm | PartType::HexArm | PartType::PistonArm => SimPartType::Arms,
            PartType::Track => SimPartType::Track,
            PartType::Bonding => SimPartType::Bonding,
            PartType::MultiBonding => SimPartType::MultiBonding,
            PartType::Unbonding => SimPartType::Unbonding,
            PartType::Calcification => SimPartType::Calcification,
            PartType::Animismus => SimPartType::Animismus,
            PartType::Projection => SimPartType::Projection,
            PartType::Purification => SimPartType::Purification,
            PartType::Conduit => SimPartType::Conduit,
            _ => { println!("{:?}", part.ty); return Err("unknown part type"); }
        })
    }
}
// the Big Switch

use crate::data::{HexIndex, Molecule, Part, PartType, Puzzle};
use crate::sim::{AnyPart, Sim, SimPart};

pub fn from_file_part(part: &Part, puzzle: &Puzzle) -> Result<AnyPart, &'static str>{
    let pos = part.pos;
    let rotation = (((part.rotation % 6) + 6) % 6) as u8;
    Ok(match part.ty{
        PartType::Input => Box::new(Input{ m: puzzle.reagents[part.index as usize].clone(), pos, rotation }),
        PartType::Output => panic!(),
        PartType::Conduit => panic!(),
        PartType::Arm | PartType::BiArm | PartType::TriArm | PartType::HexArm | PartType::PistonArm => panic!(),
        PartType::Bonding => panic!(),
        PartType::Unbonding => panic!(),
        PartType::Purification => panic!(),
        PartType::Projection => panic!(),
        _ => return Err("unsupported part type")
    })
}

// IO

#[derive(Debug, Clone)]
pub struct Input{
    m: Molecule,
    pos: HexIndex,
    rotation: u8
}

impl SimPart for Input{
    fn tick(&mut self, s: &mut Sim, is_cycle_start: bool){

    }

    fn clone_boxed(&self) -> AnyPart{
        Box::new(self.clone())
    }
}

#[derive(Debug)]
pub struct Output{

}

#[derive(Debug)]
pub struct Conduit{

}

// Arms

#[derive(Debug)]
pub struct SimArm{

}

// Glyph of Calcification

#[derive(Debug)]
pub struct SimCalcification{

}

// Glyph of Bonding

#[derive(Debug)]
pub struct SimBonding{

}

// Glyph of Unbonding

#[derive(Debug)]
pub struct SimUnbonding{

}

// Glyph of Projection

#[derive(Debug)]
pub struct SimProjection{

}

// Glyph of Purification

#[derive(Debug)]
pub struct SimPurification{

}
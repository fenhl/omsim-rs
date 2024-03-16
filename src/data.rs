use std::collections::HashMap;
use std::fmt::Debug;

// Puzzle and solution files

#[derive(Debug)]
pub struct Puzzle{
    pub name: String,
    pub reagents: Vec<Molecule>,
    pub products: Vec<Molecule>,
    pub product_multiplier: i32,

    pub production_info: Option<ProductionInfo>
}

#[derive(Debug)]
pub struct Solution{
    pub name: String,
    pub metrics: Option<Metrics>,
    pub parts: Vec<Part>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Metrics{
    pub cycles: i32,
    pub cost: i32,
    pub area: i32,
    pub instructions: i32
}

// Production info

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionInfo{
    pub chambers: Vec<Chamber>,
    pub conduits: Vec<Conduit>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chamber{
    pub pos: HexIndex,
    pub ty: ChamberType
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conduit{
    pub pos_a: HexIndex,
    pub pos_b: HexIndex,
    pub hexes: Vec<HexIndex>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChamberType{
    Medium,
    // ...
}

// Atoms and molecules

#[derive(Debug, PartialEq, Eq)]
pub struct Molecule{
    pub atoms: HashMap<HexIndex, Atom>,
    pub bonds: Vec<Bond>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bond{
    pub start: HexIndex,
    pub end: HexIndex,
    pub ty: BondType
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum Atom{
    #[default] Salt, Air, Earth, Fire, Water,
    Quicksilver, Vitae, Mors,
    Lead, Tin, Iron, Copper, Silver, Gold,
    Quintessence,
    Repeat
}

impl Atom {
    pub fn from_id(id: u8) -> Option<Atom>{
        Some(match id {
            1 => Atom::Salt,
            2 => Atom::Air,
            3 => Atom::Earth,
            4 => Atom::Fire,
            5 => Atom::Water,
            6 => Atom::Quicksilver,
            7 => Atom::Gold,
            8 => Atom::Silver,
            9 => Atom::Copper,
            10 => Atom::Iron,
            11 => Atom::Tin,
            12 => Atom::Lead,
            13 => Atom::Vitae,
            14 => Atom::Mors,
            15 => Atom::Repeat,
            16 => Atom::Quintessence,
            _ => return None
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum BondType{
    #[default] Normal,
    Triplex{ red: bool, black: bool, yellow: bool }
}

// Parts

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Part{
    pub ty: PartType,
    pub pos: HexIndex,
    pub rotation: i32,
    pub arm_number: i32,
    pub arm_length: i32,
    pub index: i32,
    pub conduit_index: i32,
    pub track_hexes: Vec<HexIndex>,
    pub conduit_hexes: Vec<HexIndex>,
    pub instructions: Vec<(Instruction, i32)>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartType{
    // IO
    Input, Output, PolymerOutput,
    // Mechanisms
    Arm, BiArm, TriArm, HexArm, PistonArm,
    Track, Berlo,
    // Glyphs
    Equilibrium, Bonding, MultiBonding, Debonding, Calcification,
    Projection, Purification,
    Duplication, Animismus,
    Unification, Dispersion,
    TriplexBonding,
    Disposal,
    // Misc
    Conduit
}

impl PartType {
    pub fn from_name(name: &str) -> Option<PartType>{
        Some(match name{
            "input" => PartType::Input,
            "out-std" => PartType::Output,
            "out-rep" => PartType::PolymerOutput,
            "arm1" => PartType::Arm,
            "arm2" => PartType::BiArm,
            "arm3" => PartType::TriArm,
            "arm6" => PartType::HexArm,
            "piston" => PartType::PistonArm,
            "track" => PartType::Track,
            "baron" => PartType::Berlo,
            "glyph-marker" => PartType::Equilibrium,
            "bonder" => PartType::Bonding,
            "bonder-speed" => PartType::MultiBonding,
            "unbonder" => PartType::Debonding,
            "glyph-calcification" => PartType::Calcification,
            "glyph-projection" => PartType::Projection,
            "glyph-purification" => PartType::Purification,
            "glyph-duplication" => PartType::Duplication,
            "glyph-life-and-death" => PartType::Animismus,
            "glyph-unification" => PartType::Unification,
            "glyph-dispersion" => PartType::Dispersion,
            "bonder-prisma" => PartType::TriplexBonding,
            "glyph-disposal" => PartType::Disposal,
            "pipe" => PartType::Conduit,
            _ => return None
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Instruction{
    #[default]
    Blank,
    Grab, Drop,
    RotateClockwise, RotateAnticlockwise,
    Extend, Retract,
    PivotClockwise, PivotAnticlockwise,
    Advance, Retreat,
    PeriodOverride, Reset, Repeat
}

impl Instruction {
    pub fn from_id(id: u8) -> Option<Instruction>{
        Some(match id{
            b' ' => Instruction::Blank,
            b'G' => Instruction::Grab,
            b'g' => Instruction::Drop,
            b'R' => Instruction::RotateClockwise,
            b'r' => Instruction::RotateAnticlockwise,
            b'E' => Instruction::Extend,
            b'e' => Instruction::Retract,
            b'P' => Instruction::PivotClockwise,
            b'p' => Instruction::PivotAnticlockwise,
            b'A' => Instruction::Advance,
            b'a' => Instruction::Retreat,
            b'O' => Instruction::PeriodOverride,
            b'X' => Instruction::Reset, // incredible
            b'C' => Instruction::Repeat,
            _ => return None
        })
    }
}

// Misc

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HexIndex{
    pub p: i32,
    pub q: i32
}
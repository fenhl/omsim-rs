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
    // blehh
    pub index: usize, // arm number, input/output index, conduit id
    pub extra_hexes: Vec<HexIndex>,
    pub instructions: Vec<(Instruction, i32)>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartType{
    // IO
    Input, Output, PolyOutput,
    // Mechanisms
    Arm, BiArm, TriArm, HexArm, PistonArm,
    Track, Berlo,
    // Glyphs
    Equilibrium, Bonding, MultiBonding, Debonding, Calcification,
    Purification, Projection,
    Duplication, Animismus,
    Unification, Dispersion,
    TriplexBonding,
    Disposal,
    // Misc
    Conduit
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Instruction{
    #[default]
    Blank,
    Grab, Drop,
    RotateLeft, RotateRight,
    Extend, Retract,
    PivotLeft, PivotRight,
    Advance, Retreat,
    PeriodOverride, Reset, Repeat
}

// Misc

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HexIndex{
    pub p: i32,
    pub q: i32
}
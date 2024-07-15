use std::collections::{HashMap, HashSet};
use std::convert::Into;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use bitflags::bitflags;

// Puzzle and solution files

/// A puzzle, as parsed from a puzzle file.
/// No attempt is made to check for invalid puzzles. In particular, they may have no inputs or outputs, no enabled parts, or be unsolveable.
#[derive(Debug, Clone)]
pub struct Puzzle{
    /// String ID.
    pub name: String,
    /// Steam ID of the creator of this puzzle.
    pub creator_id: u64,
    /// Input molecules.
    pub reagents: Vec<Molecule>,
    /// Output molecules.
    pub products: Vec<Molecule>,
    /// Multiplier for the number of output molecules required to complete the puzzle.
    pub product_multiplier: i32,
    /// Allowed glyphs and mechanisms.
    pub permissions: Permissions,

    /// If this puzzle is a production puzzle, the layout of chambers and conduits, otherwise None.
    pub production_info: Option<ProductionInfo>
}

/// A solution to a puzzle, as parsed from a solution file.
/// No attempt is made to check for invalid solutions. In particular, parts may have invalid state (like sizes >3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution{
    /// Display name.
    pub name: String,
    /// String ID of the puzzle this solves.
    pub puzzle_name: String,
    /// If solved, the metrics *recorded* in the solution file, otherwise None.
    /// This is unrelated to whether the solution is valid, completes, or actually has these metrics.
    pub metrics: Option<Metrics>,
    /// Placed parts, and their associated instructions.
    pub parts: Vec<Part>
}

/// Metrics that a solved solution may have.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Metrics{
    pub cycles: i32,
    pub cost: i32,
    pub area: i32,
    pub instructions: i32
}

bitflags! {
    /// The set of permission flags that may be enabled on a puzzle, describing enabled glyphs, mechanisms, and instructions.
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Permissions: u64{
        const SIMPLE_ARM = 1;
        const MULTI_ARMS = 2;
        const PISTON_ARM = 4;
        const TRACK = 8;
        const BONDER = 0x00000100;
        const UNBONDER = 0x00000200;
        const MULTI_BONDER = 0x00000400;
        const TRIPLEX_BONDER = 0x00000800;
        const CALCIFICATION = 0x00001000;
        const DUPLICATION = 0x00002000;
        const PROJECTION = 0x00004000;
        const PURIFICATION = 0x00008000;
        const ANIMISMUS = 0x00010000;
        const DISPOSAL = 0x00020000;
        const QUINTESSENCE = 0x00040000;
        const BERLO = 0x10000000;

        const GRAB_TURN_INSTRUCTIONS = 0x00400000;
        const DROP_INSTRUCTION = 0x00800000;
        const RESET_INSTRUCTION = 0x01000000;
        const REPEAT_INSTRUCTION = 0x02000000;
        const PIVOT_INSTRUCTIONS = 0x04000000;

        /// Permissions enabled on newly-created puzzles.
        const DEFAULT_PERMISSIONS
            = Self::SIMPLE_ARM.bits()
            | Self::MULTI_ARMS.bits()
            | Self::PISTON_ARM.bits()
            | Self::TRACK.bits()
            | Self::BONDER.bits()
            | Self::UNBONDER.bits()
            | Self::MULTI_BONDER.bits()
            | Self::CALCIFICATION.bits()
            | Self::GRAB_TURN_INSTRUCTIONS.bits()
            | Self::DROP_INSTRUCTION.bits()
            | Self::RESET_INSTRUCTION.bits()
            | Self::REPEAT_INSTRUCTION.bits()
            | Self::PIVOT_INSTRUCTIONS.bits();

        // mark unknown bits as being potentially used
        const _ = !0;
    }
}

impl Puzzle{

    pub fn clean_solution(&self, solution: &Solution) -> Result<Solution, &'static str>{
        // check puzzle name // don't actually, it's implicit in filenames. check filenames?
        // if self.name != solution.puzzle_name{
        //     return Err("solution is for the wrong puzzle");
        // }

        // check that there are no IOOB inputs/outputs
        for part in &solution.parts{
            if part.ty == PartType::Input || part.ty == PartType::Output || part.ty == PartType::PolymerOutput{
                if part.index < 0 {
                    return Err("solution contains input/output with negative index");
                }
            }
            if part.ty == PartType::Input && (part.index as usize) >= self.reagents.len(){
                return Err("solution contains input with out-of-bounds index");
            }
            if (part.ty == PartType::Output || part.ty == PartType::PolymerOutput) && (part.index as usize) >= self.products.len(){
                return Err("solution contains output with out-of-bounds index");
            }
        }
        // remove forbidden parts
        let cleaned = solution.clone();
        // TODO
        Ok(cleaned)
    }
}

// Production info

/// Information relevant only to production puzzles.
/// Purely visual information, like vial placement, is not stored.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionInfo{
    /// Whether the inputs and outputs must be placed in different chambers.
    pub isolation: bool,
    /// The chambers/cabinets that parts may be placed in.
    pub chambers: Vec<Chamber>,
    /// The conduits defined by the puzzle.
    /// Note that these are only used when creating a new solution to a puzzle; solutions may have any number and layout of conduits.
    /// These are considered illegal in the same sense as overlap.
    pub conduits: Vec<Conduit>
}

/// A chamber/cabinet that parts may be placed within in production puzzles.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chamber{
    /// Position on the board, as an offset from the origin (within u8,u8 range).
    pub pos: HexIndex,
    /// Type/size.
    pub ty: ChamberType
}

/// A conduit defined by a puzzle.
/// Note that these are only used when creating a new solution to a puzzle; solutions may have any number and layout of conduits.
/// Since the game does not allow moving conduits between chambers, conduits store only starting positions and not chamber indices.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Conduit{
    /// Default starting position of one end of the conduit.
    pub pos_a: HexIndex,
    /// Default ending position of the other end of the conduit.
    pub pos_b: HexIndex,
    /// Footprint of the conduit in its default rotation.
    pub hexes: Vec<HexIndex>
}

/// Supported chamber sizes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChamberType{
    Small, SmallWide, SmallWider,
    Medium, MediumWide,
    Large
}

impl ChamberType{
    // it's not worth an extra dependency to autogen this
    pub fn from_name(name: &str) -> Option<ChamberType>{
        Some(match name{
            "Small" => ChamberType::Small,
            "SmallWide" => ChamberType::SmallWide,
            "SmallWider" => ChamberType::SmallWider,
            "Medium" => ChamberType::Medium,
            "MediumWide" => ChamberType::MediumWide,
            "Large" => ChamberType::Large,
            _ => return None
        })
    }
}

// Atoms and molecules

/// A molecule, or collection of bonded atoms that move together.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Molecule{
    /// The atoms in this molecule by relative position.
    pub atoms: HashMap<HexIndex, Atom>,
    /// The bonds between atoms.
    pub bonds: HashSet<Bond>
}

impl Molecule{
    pub fn mapped_positions(&self, f: impl Fn(HexIndex) -> HexIndex) -> Molecule{
        // it's just easier to copy it
        let mut next_atoms = HashMap::with_capacity(self.atoms.len());
        let mut next_bonds = HashSet::with_capacity(self.bonds.len());
        for (pos, atom) in &self.atoms{
            next_atoms.insert(f(*pos), *atom);
        }
        for bond in &self.bonds{
            next_bonds.insert(Bond{ start: f(bond.start), end: f(bond.end), ty: bond.ty });
        }
        Molecule{ atoms: next_atoms, bonds: next_bonds }
    }

    pub fn translated(&self, by: HexIndex) -> Molecule{
        self.mapped_positions(|pos| pos + by)
    }

    pub fn rotated(&self, around: HexIndex, by: HexRotation) -> Molecule{
        self.mapped_positions(|pos| pos.rotated(around, by))
    }

    pub fn contains_pos(&self, pos: HexIndex) -> bool{
        self.atoms.contains_key(&pos)
    }
}

/// A bond between atoms.
/// Note that `start` and `end` may be non-adjacent in the case of quantum bonds.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bond{
    /// One end of the bond.
    pub start: HexIndex,
    /// The other end of the bond.
    pub end: HexIndex,
    /// The type of bond this is (normal or triplex).
    pub ty: BondType
}

/// An atom type, or element.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum Atom{
    #[default] Salt, Air, Earth, Fire, Water,
    Quicksilver, Vitae, Mors,
    Lead, Tin, Iron, Copper, Silver, Gold,
    Quintessence,
    Repeat
}

impl Atom{
    /// Get an atom type by byte ID, or `None` if the ID is invalid.
    pub fn from_id(id: u8) -> Option<Atom>{
        Some(match id{
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

/// A bond type (normal or triplex).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum BondType{
    #[default] Normal,
    Triplex{ red: bool, black: bool, yellow: bool }
}

// Parts

/// A part, as parsed from a solution file.
/// Invalid state, such as arms with sizes >3, or instructions on glyphs, is preserved.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Part{
    pub ty: PartType,
    pub pos: HexIndex,
    pub rotation: i32,
    pub arm_number: i32,
    pub arm_length: i32,
    /// If this is an input or output, index of which input/output this is for.
    pub index: i32,
    /// If this is a conduit, index of which conduit this is an end of.
    /// This is *not* validated against the puzzle's defined conduits.
    pub conduit_index: i32,
    /// If this is a track, the hexes this track covers in placement order.
    pub track_hexes: Vec<HexIndex>,
    /// If this is a conduit, the hexes this conduit occupies.
    pub conduit_hexes: Vec<HexIndex>,
    /// If this is an arm, the instructions this arm has, as `(instruction, index)`.
    /// This is *not* guaranteed to be valid or runnable.
    pub instructions: Vec<(Instruction, i32)>
}

/// A part type, or kind of mechanism or glyph.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PartType{
    // IO
    Input, Output, PolymerOutput,
    // Mechanisms
    Arm, BiArm, TriArm, HexArm, PistonArm,
    Track, Berlo,
    // Glyphs
    Equilibrium, Bonding, MultiBonding, Unbonding, Calcification,
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
            "unbonder" => PartType::Unbonding,
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

/// A type of instruction.
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
    pub const fn from_id(id: u8) -> Option<Instruction>{
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

/// A position or offset on a hex grid.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HexIndex{
    /// Position along the horizontal Q axis (also called X).
    pub q: i32,
    /// Position along the up-right R axis.
    pub r: i32
}

impl HexIndex{
    /// Implicit S coordinate of this coordinate.
    pub const fn s(self) -> i32{
        -self.q - self.r
    }

    pub const fn rotated_cw(self) -> HexIndex{
        HexIndex{ q: -self.r, r: -self.s() }
    }

    pub const fn rotated_ccw(self) -> HexIndex{
        HexIndex{ q: -self.s(), r: -self.q }
    }

    pub fn rotated(self, around: HexIndex, by: HexRotation) -> HexIndex{
        let mut offset = self - around;
        for _ in 0..by.turns(){
            offset = offset.rotated_cw();
        }
        offset + around
    }
}

impl Add for HexIndex{
    type Output = HexIndex;
    fn add(self, rhs: HexIndex) -> HexIndex{
        HexIndex{ q: self.q + rhs.q, r: self.r + rhs.r }
    }
}

impl AddAssign for HexIndex{
    fn add_assign(&mut self, rhs: HexIndex){
        self.q += rhs.q;
        self.r += rhs.r;
    }
}

impl Sub for HexIndex{
    type Output = HexIndex;
    fn sub(self, rhs: HexIndex) -> HexIndex{
        HexIndex{ q: self.q - rhs.q, r: self.r - rhs.r }
    }
}

impl SubAssign for HexIndex{
    fn sub_assign(&mut self, rhs: HexIndex){
        self.q -= rhs.q;
        self.r -= rhs.r;
    }
}

/// A rotation on a hex grid.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct HexRotation{
    turns: u8,
}

impl HexRotation{
    pub const R0: HexRotation = HexRotation{ turns: 0 };
    pub const R60: HexRotation = HexRotation{ turns: 1 };
    pub const R120: HexRotation = HexRotation{ turns: 2 };
    pub const R180: HexRotation = HexRotation{ turns: 3 };
    pub const R240: HexRotation = HexRotation{ turns: 4 };
    pub const R300: HexRotation = HexRotation{ turns: 5 };

    pub fn from_unsigned<T: Into<u64>>(turns: T) -> HexRotation{
        HexRotation{ turns: (turns.into() % 6) as u8 }
    }

    pub fn from_signed<T: Into<i64>>(turns: T) -> HexRotation{
        HexRotation{ turns: (((turns.into() % 6) + 6) % 6) as u8 }
    }

    pub fn turns(self) -> u8{
        self.turns
    }

    pub fn to_radians(self) -> f32{
        return (self.turns as f32 * 60.0) * (std::f32::consts::PI / 180.0);
    }
}

// basically any number can be a hex rotation
// we can't have both Into<usize> and Into<isize>, and we mostly work with u8s and i32s here, so isize wins
impl<T: Into<i64>> From<T> for HexRotation{
    fn from(turns: T) -> HexRotation{
        Self::from_signed(turns.into())
    }
}

impl Add for HexRotation{
    type Output = HexRotation;

    fn add(self, rhs: HexRotation) -> HexRotation{
        HexRotation{ turns: (self.turns + rhs.turns) % 6 }
    }
}

impl AddAssign for HexRotation{
    fn add_assign(&mut self, rhs: HexRotation){
        *self = *self + rhs;
    }
}

impl Sub for HexRotation{
    type Output = HexRotation;

    fn sub(self, rhs: HexRotation) -> HexRotation{
        HexRotation{ turns: ((((self.turns as i8) - (rhs.turns as i8) % 6) + 6) % 6) as u8 }
    }
}

impl SubAssign for HexRotation{
    fn sub_assign(&mut self, rhs: HexRotation) {
        *self = *self - rhs;
    }
}
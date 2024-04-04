use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use crate::data::{HexIndex, HexRotation};

pub const HEX_WIDTH: f32 = 82.0;
pub const HEX_HEIGHT: f32 = 71.0;

/// A 2D vector in ordinary cartesian coordinates.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector2{
    pub x: f32,
    pub y: f32
}

impl Vector2{
    pub fn new(x: f32, y: f32) -> Vector2{
        Vector2{ x, y }
    }

    pub fn from_hex_index(h: HexIndex) -> Vector2{
        Vector2::new((h.q as f32)*HEX_WIDTH + 0.5 * (h.r as f32)*HEX_WIDTH, (h.r as f32)*HEX_HEIGHT)
    }

    pub fn length2(self) -> f32{
        self.x * self.x + self.y + self.y
    }

    pub fn length(self) -> f32{
        self.length2().sqrt()
    }

    pub fn dist(self, other: Vector2) -> f32{
        (self - other).length()
    }
}

impl From<HexIndex> for Vector2{
    fn from(value: HexIndex) -> Vector2{
        Vector2::from_hex_index(value)
    }
}

impl Add for Vector2{
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2{
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2{
    fn add_assign(&mut self, rhs: Vector2){
        *self = *self + rhs;
    }
}

impl Sub for Vector2{
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2{
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<f32> for Vector2{
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2{
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vector2{
    fn mul_assign(&mut self, rhs: f32){
        *self = *self * rhs;
    }
}

/// A collider type. Different colliders have different radii and behaviours.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ColliderType{
    Atom, // radius 29
    ArmBase, // radius 20
    ArmGripper, // radius 20?, only collides with ChamberWall
    ProducedAtom, // radius 15
    ChamberWall, // radius 20?, only collides with Atom and ArmGripper
}

impl ColliderType{
    pub fn radius(self) -> f32{
        match self{
            ColliderType::Atom => 29.0,
            ColliderType::ArmBase => 20.0,
            ColliderType::ArmGripper => 20.0,
            ColliderType::ProducedAtom => 15.0,
            ColliderType::ChamberWall => 20.0
        }
    }

    pub fn radius_with(self, other: ColliderType) -> Option<f32>{
        if self == ColliderType::ArmGripper && other != ColliderType::ChamberWall
        || other == ColliderType::ArmGripper && self != ColliderType::ChamberWall{
            return None;
        }
        Some(self.radius() + other.radius())
    }
}

/// A movement that a collider will make.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Movement{
    Stay{ at: HexIndex },
    Translate{ start: HexIndex, end: HexIndex },
    Rotate{ start: HexIndex, around: HexIndex, rotation: HexRotation }
}

impl Movement{
    pub fn pos_at(self, time: f32) -> Vector2{
        match self{
            Movement::Stay{ at } => Vector2::from_hex_index(at),
            Movement::Translate{ start, end } => {
                let (start, end): (Vector2, Vector2) = (start.into(), end.into());
                start + (end - start)*time
            }
            Movement::Rotate{ start, around, rotation } => {
                let (start, around, r): (Vector2, Vector2, f32) = (start.into(), around.into(), -rotation.to_radians() * time);
                let tr = start - around;
                Vector2::new(f32::cos(r) * tr.x - f32::sin(r) * tr.y, f32::sin(r) * tr.x + f32::cos(r) * tr.y) + around
            }
        }
    }
}

/// A collider on the board, with a position and type.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Collider{
    pub ty: ColliderType,
    pub movement: Movement
}

/// Test whether anything in this list of colliders collides.
pub fn collides(colliders: &Vec<Collider>, steps: u32) -> bool{
    for i in 0..=steps{
        let time: f32 = (i as f32)/(steps as f32);
        // bleh
        let objs: Vec<(Vector2, ColliderType)> = colliders.iter().map(|c| (c.movement.pos_at(time), c.ty)).collect();
        for l in 0..objs.len(){
            for r in 0..objs.len(){
                if l != r{
                    let ((lpos, lty), (rpos, rty)) = (objs[l], objs[r]);
                    if let Some(radius) = lty.radius_with(rty){
                        if lpos.dist2(rpos) < radius*radius{
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}
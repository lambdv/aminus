use crate::model::statable::Statable;
/// Represents a comutation from a statable to an f32 number
/// eg: damage output from an attack/action in a rotation or to compute some buff from a character
pub trait Computable {
    fn compute(&self, stats: &dyn Statable) -> f32;
}


use crate::model::statable::Statable;

/// Represents a computation from a statable to an f32 number
pub type Operation = Box<dyn Fn(&dyn Statable) -> f32>;

//pub type Op = fn(&dyn Statable) -> f32;

// pub trait Operation {
//     fn evaluate(&self, statable: Box<dyn Statable>) -> f32;
// }


pub type UnaryOperation = Box<dyn Fn(&dyn Statable) -> Box<dyn Statable>>;
pub type BinaryOperation = Box<dyn Fn(&dyn Statable, &dyn Statable) -> Box<dyn Statable>>;
pub type UnaryScalarOperation = Box<dyn Fn(&dyn Statable) -> f32>;
pub type BinaryScalarOperation = Box<dyn Fn(&dyn Statable, &dyn Statable) -> f32>;

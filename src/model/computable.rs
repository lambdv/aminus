use crate::model::statable::Statable;
pub trait Computable {
    fn compute(&self, stats: &dyn Statable) -> f32;
}
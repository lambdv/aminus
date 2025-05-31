use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
//use crate::computable::Computable;

/// represents a sequence of action preformed by a character
// #[derive(Debug, Clone, PartialEq, Default)]
pub struct Rotation{        
    inner: std::collections::HashMap<String, Box<dyn Fn(&dyn Statable) -> f32>>,
}

impl Rotation {
    /// Construct an empty Rotation
    pub fn new() -> Self {
        Self { inner: std::collections::HashMap::new() }
    }

    /// Construct a Rotation with pre-defined actions
    pub fn of(actions: Vec<(String, Box<dyn Fn(&dyn Statable) -> f32>)>) -> Self {
        let mut map = std::collections::HashMap::new();
        for (k, v) in actions {
            map.insert(k, v);
        }
        Self { inner: map }
    }

    /// Add an action to the rotaiton
    pub fn add(&mut self, name: String, action: Box<dyn Fn(&dyn Statable) -> f32>) -> & mut Self {
        self.inner.insert(name, action);
        self
    }

    /// Compute all actions using the provided stats and return the sum
    pub fn execute(&self, stats: &StatTable) -> f32 {
        self.inner.iter()
            .map(|x| x.1)
            .map(|x|x(stats))
            .sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::stat::Stat;

    #[test] fn empty_rotation_returns_0() {
        let res = Rotation::new().execute(&StatTable::new());
        debug_assert_eq!(res, 0.0)
    }

    #[test] fn populated_rotation_returns_sum_of_action_results() {
        let r = Rotation::of(vec![
            (String::from("test"), Box::new(|s| s.get(&Stat::FlatATK))),
            (String::from("test2"), Box::new(|s| s.get(&Stat::FlatATK))),
        ]);
        let s = StatTable::of(&[
            (Stat::FlatATK, 1.6),
        ]);
        let res = r.execute(&s);
        debug_assert_eq!(res, 1.6*2.0)
    }
}
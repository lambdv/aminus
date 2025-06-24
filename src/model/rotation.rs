use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::model::operation::Operation;

/// represents a sequence of action performed by a character
// #[derive(Debug, Clone, PartialEq, Default)]
pub struct Rotation {        
    inner: std::collections::HashMap<String, Operation>,
}

impl Rotation {
    /// construct a new empty rotation
    pub fn new() -> Self {
        Self{inner: std::collections::HashMap::new()}
    }
    /// construct a new rotation with default values
    pub fn of(actions: Vec<(String, Operation)>) -> Self {
        let mut map = std::collections::HashMap::new();
        for (k, v) in actions {
            map.insert(k, v);
        }
        Self{inner: map}
    }

    /// add an action to the rotation
    pub fn add(&mut self, name: String, action: Operation) -> &mut Self {
        self.inner.insert(name, action);
        self
    }

    /// compute and return sum of all actions based on a given statable instance
    pub fn evaluate(&self, stats: &StatTable) -> f32 {
        self.inner.iter()
            .map(|x| x.1(stats))
            .sum()
    }

    // compute and return sum of all actions based on a given statable instance
    // pub fn execute_buffed(&self, stats: &StatTable, buffs: &StatTable) -> f32 {
    //     let mut sum = stats.clone().chain(buffs);
    //     self.inner.iter()
    //         .map(|x| x.1)
    //         .map(|x|x(stats.chain(buffs)))
    //         .sum()
    // }
}

#[cfg(test)] mod tests {
    use super::*;
    use crate::model::stat::Stat;

    #[test] fn empty_rotation_returns_0() {
        let res = Rotation::new().evaluate(&StatTable::new());
        debug_assert_eq!(res, 0.0)
    }

    #[test] fn populated_rotation_returns_sum_of_action_results() {
        let r = Rotation::of(vec![
            (String::from("test"), Box::new(|stats| stats.get(&Stat::FlatATK))),
            (String::from("test2"), Box::new(|stats| stats.get(&Stat::FlatATK))),
        ]);
        let s = StatTable::of(&[
            (Stat::FlatATK, 1.6),
        ]);
        let res = r.evaluate(&s);
        debug_assert_eq!(res, 1.6*2.0)
    }
}
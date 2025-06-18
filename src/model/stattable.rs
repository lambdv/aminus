use crate::model::stat::Stat;
use crate::model::statable::*;
use crate::model::statable::ModifiableStatable;

///concrete statable that stores stat->f32 mapping in a hash map
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StatTable { 
    inner: std::collections::HashMap<Stat, f32>, 
}

impl StatTable {
    pub fn new() -> StatTable {
        StatTable { inner: std::collections::HashMap::new() }
    }
    pub fn of(values: &[(Stat, f32)]) -> StatTable {
        let mut map = std::collections::HashMap::new();
        for &(k, v) in values {
            *map.entry(k).or_insert(0.0) += v;
        }
        StatTable { inner: map }
    }

    pub fn from_iter(
        iter: StatableIter
    ) -> StatTable {
        let mut map = std::collections::HashMap::new();
        for (k, v) in iter {
            *map.entry(k).or_insert(0.0) += v;
        }
        StatTable { inner: map }
    }
}

impl Statable for StatTable {
    fn get(&self, stat_type: &Stat) -> f32 {
        *self.inner.get(stat_type).unwrap_or(&0.0)
    }
    fn iter(&self) -> StatableIter {
        Box::new(self.inner.iter().map(|(k, v)| (*k, *v)))
    }
}

impl ModifiableStatable for StatTable {
    fn add(&mut self, stat_type: &Stat, value: f32)-> f32 {
        self.inner
            .insert(*stat_type, self.get(stat_type) + value)
            .unwrap_or(0.0)
    }
}


//type Computable = dyn Fn(&dyn Statable) -> f32;
// pub struct ComputedStatTable {
//     constants: std::collections::HashMap<Stat, f32>,
//     closures: std::collections::HashMap<Stat, Box<Computable>>,
// }


#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn construct_with_intial_values() {
        let s = StatTable::of(&[
            (Stat::ATKPercent, 1.0),
            (Stat::ATKPercent, 5.0),
        ]);
        assert_eq!(s.inner.get(&Stat::ATKPercent), Some(&6.0));
        assert_eq!(s.get(&Stat::ATKPercent), 6.0);
    }

    #[test] fn test_adding_and_getting() {
        let s = Stat::FlatATK;
        let mut table: StatTable = StatTable::new();
        assert_eq!(table.get(&s), 0.0); //starts at 0
        table.add(&s, 10.0);
        assert_eq!(table.get(&s), 10.0); //adding 10 sets it 0
        table.add(&s, 10.0);
        assert_eq!(table.get(&s), 20.0); //adding 10 more accumulates
    }
    #[test] fn test_adding_stattable() {
        let mut t1 = StatTable::new();
        assert_eq!(t1.get(&Stat::CritDMG), 0.0);
        t1.add(&Stat::FlatATK, 2000.0);
        let mut t2 = StatTable::new();
        t2.add(&Stat::CritDMG, 0.5);
        t1.add_table(t2.iter());
        assert_eq!(t1.get(&Stat::CritDMG), 0.5);
    }
}
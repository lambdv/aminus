use crate::model::stat::Stat;
use crate::model::statable::statable::Statable;


/// Collection of stats
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StatTable {
    inner: std::collections::HashMap<Stat, f32>,
}

impl StatTable {
    /// Constructor for StatTable
    pub fn new() -> StatTable {
        StatTable { inner: std::collections::HashMap::new() }
    }

    /// Constructor for StatTable with values
    pub fn of(values: &[(Stat, f32)]) -> StatTable {
        StatTable { 
            inner: std::collections::HashMap::from_iter(values.iter().map(|(k, v)| (*k, *v)))
        }
    }

    ///get the amount coorasponding to stat_type stored in the table
    /// gives 0 if no stat_type is stored
    pub fn get(&self, stat_type: &Stat) -> f32 {
        *self.inner.get(stat_type)
            .unwrap_or(&0.0)
    }

    ///adds amount to coorasponding to stat_type stored in the table and returns the new amount
    pub fn add(&mut self , stat_type: &Stat, value: f32) -> f32 {
        self.inner.insert(*stat_type, self.get(stat_type) + value)
            .unwrap_or(0.0)
    }

    pub fn add_table(&mut self , other: &StatTable) -> &mut Self {
        other.inner.iter()
            .for_each(|(k, v)| { self.add(k, *v); });
        self
    }
}

impl Statable for StatTable {
    fn stats(&self) -> &StatTable {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adding_and_getting() {
        let s = Stat::FlatATK;
        let mut table: StatTable = StatTable::new();
        assert_eq!(table.get(&s), 0.0);

        table.add(&s, 10.0);
        assert_eq!(table.get(&s), 10.0);
    }

    #[test]
    fn test_adding_stattable(){
        let mut t1 = StatTable::new();
        assert_eq!(t1.get(&Stat::CritDMG), 0.0);
        t1.add(&Stat::FlatATK, 2000.0);

        let mut t2 = StatTable::new();
        t2.add(&Stat::CritDMG, 0.5);

        t1.add_table(&t2);

        assert_eq!(t1.get(&Stat::CritDMG), 0.5);
    }
}


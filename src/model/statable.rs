use crate::model::stat::Stat;
use crate::model::stattable::StatTable;
/// StatValue is a tuple of a stat type and a value
pub type StatValue = (Stat, f32);
/// StatableIter is a box of an iterator of stat-value pair tuples
pub type StatableIter<'a> = Box<dyn Iterator<Item=StatValue>+'a>;
/// Top level primative type that lets you get an f32 value mapped to a stat type enum
pub trait Statable {
    ///gets all stats from a statable as an iter with stat-value pair tuples 
    fn iter(&self) -> StatableIter;
    ///return value mapped to stat_type if its stored, else returns 0
    fn get(&self, stat_type: &Stat) -> f32{
        self.iter()
            .filter(|x| x.0 == *stat_type)
            .map(|x| x.1)
            .sum()
    }
    /// turnary operator that sums 2 statables together and returns the results
    fn chain(&self, other: Box<dyn Statable>) -> Box<dyn Statable>{
        let mut res = StatTable::new();
        res.add_table(self.iter());
        res.add_table(other.iter());
        Box::new(res)
    }
}

/// Statable that is directly mutable
pub trait ModifiableStatable: Statable {
    ///add amount to stat
    fn add(&mut self, stat_type: &Stat, value: f32) -> f32;
    ///add all stats from other statable to self, returns self for fluent interface
    fn add_table(&mut self, other: StatableIter) -> &mut Self{
        other.for_each(|(k, v)| {
            self.add(&k, v);
        });
        self
    }
}


#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_chainging_stattables() {
        let s1 = StatTable::of(&vec![(Stat::FlatATK, 100.)]);
        let s2 = StatTable::of(&vec![(Stat::FlatATK, 100.)]);
        let s3 = s1.chain(Box::new(s2));
        assert_eq!(s3.get(&Stat::FlatATK), 200.);
    }
}

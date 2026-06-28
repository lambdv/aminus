use crate::model::stat::Stat;
use crate::model::statable::*;
use crate::model::stattable::StatTable;


/// builder pattern for stat tables.
/// used to lazily build/compute stat tables from many statables and computed values
pub struct StatTableBuilder { 
    stattables: Vec<Box<dyn Statable>>,
    computed_values: std::collections::HashMap<Stat, Vec<Box<dyn Fn(&StatTable) -> f32>>>,
    dependencies: Vec<Box<dyn Statable>>,
}

impl StatTableBuilder {
    /// create a new stat table builder
    pub fn new() -> Self {
        Self { stattables: Vec::new(), computed_values: std::collections::HashMap::new(), dependencies: Vec::new() }
    }

    pub fn build(&self) -> StatTable {
        let mut table = StatTable::new();
        for statable in self.stattables.iter() {
            table.add_table(statable.iter());
        }
        for (stat, ops) in self.computed_values.iter() {
            for op in ops.iter() {
                table.add(stat, op(&table));
            }
        }
        table
    }

    /// add a stat to the builder
    pub fn add_stat(&mut self, stat: Stat, value: f32) -> &mut Self {
        self.stattables.push(Box::new(StatTable::of(&[(stat, value)])));
        self
    }

    /// add a statable to the builder
    pub fn add_table(&mut self, statable: Box<dyn Statable>) -> &mut Self {
        self.stattables.push(statable);
        self
    }

    /// add a computed value to the builder
    pub fn add_computed_value(&mut self, stat: Stat, op: Box<dyn Fn(&StatTable) -> f32>) -> &mut Self {
        self.computed_values.entry(stat).or_insert(Vec::new()).push(op);
        self
    }
}



#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_builder_with_tables() {
        let mut builder = StatTableBuilder::new();
        builder.add_stat(Stat::FlatATK, 100.);
        builder.add_stat(Stat::FlatATK, 100.);
        assert_eq!(builder.build().get(&Stat::FlatATK), 200.);



        builder.add_table(Box::new(StatTable::of(&[(Stat::FlatATK, 100.)])));
        assert_eq!(builder.build().get(&Stat::FlatATK), 300.);

        builder.add_stat(Stat::FlatHP, 1000.);
        assert_eq!(builder.build().get(&Stat::FlatATK), 300.);
        assert_eq!(builder.build().get(&Stat::FlatHP), 1000.);
    }

    // #[test] fn test_builder_with_computed_values() {
    //     let mut builder = StatTableBuilder::new();
    //     builder.add_stat(Stat::FlatATK, 100.);
    //     builder.add_computed_value(Stat::FlatATK, Box::new(|s| s.get(&Stat::FlatATK) * 2.));
    //     let table = builder.build();
    //     assert_eq!(table.get(&Stat::FlatATK), 300.);
    // }

}
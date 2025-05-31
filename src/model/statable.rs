use crate::model::stat::Stat;
pub trait Statable {
    fn iter(&self) -> Box<dyn Iterator<Item = (Stat, f32)> + '_>;
    ///return value mapped to stat_type if its stored, else returns 0
    fn get(&self, stat_type: &Stat) -> f32;
    // {
    //     self.iter()
    //         .filter(|x| x.0 == *stat_type)
    //         .map(|x| x.1)
    //         .sum()
    // }    
}
pub trait ModifiableStatable: Statable {
    //adds amount to coorasponding to stat_type stored in the table and returns the new amount
    fn add(&mut self, stat_type: &Stat, value: f32) -> f32;
    fn add_table(&mut self, other: &dyn Statable) -> &mut Self;
}

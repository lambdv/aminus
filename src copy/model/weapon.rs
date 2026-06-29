use crate::model::stat::Stat;
use crate::model::stattable::StatTable;
use crate::model::statable::*;
use crate::model::statable::ModifiableStatable;

/// abstraction over statable to model an ingame weapon
pub struct Weapon {
    base_stat_value: f32,
    main_stat_value: f32,
    main_stat_type: Stat
}

impl Statable for Weapon{
    fn get(&self, stat_type: &Stat) -> f32{
        match stat_type{
            Stat::BaseATK => self.base_stat_value,
            s if *s == self.main_stat_type  => 0.0,
            _ => 0.0,
        }
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (Stat, f32)> + '_> {
        Box::new(vec![
            (Stat::BaseATK, self.base_stat_value),
            (self.main_stat_type, self.main_stat_value),
        ].into_iter())
    }
}
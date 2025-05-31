use crate::model::stat::Stat;
use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::model::statable::ModifiableStatable;

pub struct Weapon {
    //_level: u8,
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


//composite type
pub struct Character {
    base_stats: StatTable,
    fluid_stats: StatTable,
    weapon: Option<StatTable>,
    flower: Option<StatTable>,
    plume: Option<StatTable>,
    sands: Option<StatTable>,
    goblet: Option<StatTable>,
    circlet: Option<StatTable>,
}

impl Statable for Character{
    fn get(&self, stat_type: &Stat) -> f32{
        let base_value = self.base_stats.get(stat_type);
        let fluid_value = self.fluid_stats.get(stat_type);
        let weapon_value = self.weapon.as_ref().map_or(0.0, |w| w.get(stat_type));
        let flower_value = self.flower.as_ref().map_or(0.0, |f| f.get(stat_type));
        let plume_value = self.plume.as_ref().map_or(0.0, |p| p.get(stat_type));
        let sands_value = self.sands.as_ref().map_or(0.0, |s| s.get(stat_type));
        let goblet_value = self.goblet.as_ref().map_or(0.0, |g| g.get(stat_type));
        let circlet_value = self.circlet.as_ref().map_or(0.0, |c| c.get(stat_type));
        base_value + fluid_value + weapon_value + flower_value + plume_value + sands_value + goblet_value + circlet_value
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (Stat, f32)> + '_> {
        let iter = self
            .base_stats.iter()
            .chain(self.fluid_stats.iter())
            .chain(self.weapon.iter().flat_map(|s| s.iter()))
            .chain(self.flower.iter().flat_map(|s| s.iter()))
            .chain(self.plume.iter().flat_map(|s| s.iter()))
            .chain(self.sands.iter().flat_map(|s| s.iter()))
            .chain(self.goblet.iter().flat_map(|s| s.iter()))
            .chain(self.circlet.iter().flat_map(|s| s.iter()));
        Box::new(iter)
    }
}



pub enum Artifact{
    Flower(),
    Feather(),
    Sands(),
    Goblet(),
    Circlet(),
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_and_getting() {

    }

}

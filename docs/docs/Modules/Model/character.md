# Character

Abstraction over statable to represent an ingame character.

## Description

This module provides the `Character` struct which composes static base stats, mutable fluid stats for buffs, and composites a weapon and artifacts. It implements both `Statable` and `ModifiableStatable` traits.

```rust
use crate::model::stat::Stat;
use crate::model::stattable::StatTable;
use crate::model::statable::*;
use crate::model::statable::ModifiableStatable;

/// abstraction over statable to represent an ingame character.
/// composes static base stats, mutable fluid stats for buffs and composites a weapon and artifacts
pub struct Character {
    base_stats: StatTable,
    fluid_stats: StatTable,
    //dynamic_stats: StatTable,
    weapon: Option<StatTable>,
    flower: Option<StatTable>,
    plume: Option<StatTable>,
    sands: Option<StatTable>,
    goblet: Option<StatTable>,
    circlet: Option<StatTable>,
}

impl Statable for Character{
    fn iter(&self) -> StatableIter {
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
}

impl ModifiableStatable for Character{
    fn add(&mut self, stat_type: &Stat, value: f32)-> f32 {
        self.fluid_stats.add(stat_type, value)
    }
    fn add_table(&mut self, other: StatableIter) -> &mut Self {
        other.for_each(|(k, v)| {
            self.add(&k, v);
        });
        self
    }
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn test_adding_and_getting() {
        
    }
} 
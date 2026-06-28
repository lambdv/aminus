use crate::artifact;
use crate::model::stat::Stat;
use crate::model::stattable::StatTable;
use crate::model::statable::*;
use crate::model::statable::ModifiableStatable;
use std::collections::HashSet;

pub struct ArtifactSpec {
    pub rarity: i8,
    pub level: i8,
    pub stat_type: Stat,
    pub sub_stats: Option<StatTable>,
    pub set: Option<String>
 }

 impl ArtifactSpec {
    pub fn new(rarity: i8, level: i8, stat_type: Stat) -> Self {
        Self {rarity,level,stat_type,sub_stats: None,set: None}
    }
 }


/// Abstraction over statable to model an ingame artifact piece.
pub enum Artifact{
    Flower(ArtifactSpec),
    Feather(ArtifactSpec),
    Sands(ArtifactSpec),
    Goblet(ArtifactSpec),
    Circlet(ArtifactSpec),
}

impl Artifact {
    pub fn new_flower(rarity: i8, level: i8) -> Self {
        Self::Flower(ArtifactSpec {
            rarity: rarity,
            level: level,
            stat_type: Stat::FlatATK,
            sub_stats: Some(StatTable::new()),
            set: Some(String::from("eosf")),
        })
    }
}



// impl Statable for Artifact {
//     fn iter(&self) -> StatableIter {
//         match self {
//             Artifact::Flower(spec) => Box::new(spec.sub_stats.iter()),
//             Artifact::Feather(spec) => Box::new(spec.sub_stats.iter()),
//             Artifact::Sands(spec) => Box::new(spec.sub_stats.iter()),
//             Artifact::Goblet(spec) => Box::new(spec.sub_stats.iter()),
//             Artifact::Circlet(spec) => Box::new(spec.sub_stats.iter()),
//         }
//     }
// }

// pub struct Flower {inner: ArtifactSpec}
// impl Flower {
//     pub fn new(rarity: i8, level: i8, main_stat: &Stat) -> Self {
//         Self{inner: ArtifactSpec {
//             rarity: rarity,
//             level: level,
//             stat_type: *main_stat,
//             sub_stats: StatTable::new(),
//             _set: String::from("eosf"),
//         }}
//     }
// }
// impl Statable for Flower {
//     fn iter(&self) -> StatableIter{
//         let main_stat_value = 0.0;
//         Box::new(
//             vec![(Stat::FlatATK, main_stat_value)].into_iter().chain(self.inner.sub_stats.iter())
//         )
//     }
// }


pub const POSSIBLE_SANDS_STATS: &[Stat] = &[
    Stat::HPPercent,
    Stat::ATKPercent,
    Stat::DEFPercent,
    Stat::ElementalMastery,
    Stat::EnergyRecharge,
];


pub const POSSIBLE_GOBLET_STATS: &[Stat] = &[
    Stat::HPPercent,
    Stat::ATKPercent,
    Stat::DEFPercent,
    Stat::ElementalMastery,
    Stat::PyroDMGBonus, 
    Stat::CryoDMGBonus, 
    Stat::GeoDMGBonus, 
    Stat::DendroDMGBonus, 
    Stat::ElectroDMGBonus, 
    Stat::HydroDMGBonus, 
    Stat::AnemoDMGBonus, 
    Stat::PhysicalDMGBonus,
];

pub const POSSIBLE_CIRCLE_STATS: &[Stat] = &[
    Stat::HPPercent,
    Stat::ATKPercent,
    Stat::DEFPercent,
    Stat::ElementalMastery,
    Stat::CritRate,
    Stat::CritDMG,
    Stat::HealingBonus,
];

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn manual_construction() {}
}

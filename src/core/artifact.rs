use crate::core::types::Stat;
use crate::core::stattable::StatTable;
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

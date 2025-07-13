# Artifact Builder

Builder pattern for making artifact stattables.

## Description

This module provides the `ArtifactBuilder` struct which implements the builder pattern for creating artifact stat tables. It handles main stats, substat rolls, and constraints based on KQMC (KeqingMains Compendium) assumptions.

```rust
use crate::functions::stat_factory::StatFactory;
use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::stat::Stat;
use crate::statable::ModifiableStatable;
use crate::model::artifact::*;
use crate::assert_aprx;

/// Builder pattern for making artifact stattables
pub struct ArtifactBuilder{
    // artifact pieces
    pub flower: Option<ArtifactPiece>,
    pub feather: Option<ArtifactPiece>,
    pub sands: Option<ArtifactPiece>,
    pub goblet: Option<ArtifactPiece>,
    pub circlet: Option<ArtifactPiece>,
    // substat roll data
    pub rolls: std::collections::HashMap<(Stat,RollQuality, i8),i8>, // stattype, quality, rarity : amount of rolls
    pub constraints: std::collections::HashMap<(Stat, i8), i8>, // stattype, rarity : roll limit
    roll_limit: Option<i8>,
}

impl ArtifactBuilder{
    //constructors
    /// constructs a new default artifact builder
    pub fn new(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>,) -> Self {
        assert!(flower.as_ref().map(|x| x.stat_type == Stat::FlatHP).unwrap_or(true));
        assert!(feather.as_ref().map(|x| x.stat_type == Stat::FlatATK).unwrap_or(true));
        assert!(sands.as_ref().map(|x| POSSIBLE_SANDS_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(goblet.as_ref().map(|x| POSSIBLE_GOBLET_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(circlet.as_ref().map(|x| POSSIBLE_CIRCLE_STATS.contains(&x.stat_type)).unwrap_or(true));

        let mut constraints = std::collections::HashMap::new();
        for &stat in POSSIBLE_SUB_STATS {
            [&flower, &feather, &sands, &goblet, &circlet].iter()
                .filter_map(|piece| piece.as_ref())
                .filter(|piece| piece.stat_type != stat)
                .for_each(|piece| {
                    let key = (stat, piece.rarity);
                    let new_value = constraints.get(&key).unwrap_or(&0)+
                        max_rolls_for_given(&piece, &stat, false);
                    constraints.insert(key, new_value);
                });
        }
        ArtifactBuilder{flower, feather, sands, goblet, circlet,rolls: std::collections::HashMap::new(), constraints: constraints, roll_limit: None}
    }

    /// constructs artifact builder for kqmc assumptions
    // https://compendium.keqingmains.com/
    /// total 40 subs
    /// 20 fixed subs distributed across 2 rolls for each substat type
    /// 20 fluid subs to distribute, max 2 rolls for each artifact piece with a main stat that is of a different stat from it
    /// 4-star artifacts have a x0.8 substat value modifer compared to 5-stars and penalty of -2 distributed substats per 4-star artifact
    /// 1 5-star and 4 4-star means the 5 star artifact will have a stat modifer of (1 * 1 + 0.8 * 4) / 5 = 0.84x rather than 1 
    pub fn kqmc(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>) -> Self {
        //invariant checks
        assert!(flower.as_ref().map(|x| x.stat_type == Stat::FlatHP).unwrap_or(true));
        assert!(feather.as_ref().map(|x| x.stat_type == Stat::FlatATK).unwrap_or(true));
        assert!(sands.as_ref().map(|x| POSSIBLE_SANDS_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(goblet.as_ref().map(|x| POSSIBLE_GOBLET_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(circlet.as_ref().map(|x| POSSIBLE_CIRCLE_STATS.contains(&x.stat_type)).unwrap_or(true));
        [flower.as_ref(), feather.as_ref(), sands.as_ref(), goblet.as_ref(), circlet.as_ref()].iter()
            .filter_map(|x| x.as_ref())
            .for_each(|x| {
                assert!(StatFactory::check_correct_level_for_rarity(x.level, x.rarity));
                assert!(x.rarity > 3);
                assert!((x.level == 20 && x.rarity == 5) || (x.level == 16 && x.rarity == 4));
            });

        // Determine the rarity to use for rolling based on the artifacts provided
        // If all artifacts are the same rarity, use that rarity. Otherwise, use the highest rarity.
        let rarities: std::collections::HashSet<i8> = [&flower, &feather, &sands, &goblet, &circlet].iter()
            .filter_map(|x| x.as_ref())
            .map(|a| a.rarity)
            .collect();
        let roll_rarity = if rarities.len() == 1 {
            *rarities.iter().next().unwrap()
        } else {
            *rarities.iter().max().unwrap()
        };

        // calculates max number of rolls for stats colored by rarity given the artifacts provided
        let mut constraints = std::collections::HashMap::new();
        for &stat in POSSIBLE_SUB_STATS {
            [&flower, &feather, &sands, &goblet, &circlet].iter()
                .filter_map(|piece| piece.as_ref())
                .filter(|piece| piece.stat_type != stat)
                .for_each(|piece| {
                    let key = (stat, piece.rarity);
                    let new_value = constraints.get(&key).unwrap_or(&0)+2; //only 2 rolls for each substat per artifact
                    constraints.insert(key, new_value);
                });
        }

        let base = [&flower, &feather, &sands, &goblet, &circlet].iter()
            .filter_map(|piece| piece.as_ref())
            .map(|x| max_rolls_for(&x, false))
            .fold(0, |x,y| x+y) as i8;


        let penalty = [&flower, &feather, &sands, &goblet, &circlet].iter()
            .filter_map(|piece| piece.as_ref())
            .count() as i8;

        let roll_limit = base - penalty;

        let mut bob = ArtifactBuilder{flower, feather, sands, goblet, circlet,
            rolls: std::collections::HashMap::new(),
            constraints: constraints,
            roll_limit: Some(roll_limit)
        };

        POSSIBLE_SUB_STATS.iter()
            .for_each(|&stat| {
                bob.roll(stat, RollQuality::AVG, roll_rarity, 2);
                //increase constraint for each stat by 2
                let old_constraint = bob.constraints.get(&(stat, roll_rarity)).unwrap_or(&0);
                bob.constraints.insert((stat, roll_rarity), old_constraint + 2);
            });
        bob
    }

    //exports

    ///compiles main stats and sub stat rolls into a single stattable
    /// this statable represents all the stats built by the builder
    pub fn build(&self) -> StatTable {
        let mut sum = self.main_stats();
        sum.add_table(Box::new(self.sub_stats().iter()));
        sum
    }

    /// compiles main stats into a stattable
    pub fn main_stats(&self)  -> StatTable{
        let mut res = StatTable::new();
        self.main_pieces().iter().for_each(|spec| {
            let value = StatFactory::get_main_stat_value(spec.rarity, spec.level, &spec.stat_type).unwrap(); //TODO: handle this maybe
            res.add(&spec.stat_type, value);
        });
        res 
    }

    /// compiles sub stats based on rolls allocated
    pub fn sub_stats(&self)  -> StatTable{
        let mut res = StatTable::new();
        for ((stat, quality, rarity), num) in self.rolls.iter() {
            let value = StatFactory::get_sub_stat_value(*rarity, *stat).unwrap();
            let value = value * quality.multiplier() * (*num as f32);
            res.add(stat, value);
        }
        res
    }

    //update methods

    /// rolls a substat
    pub fn roll(&mut self, substat_value: Stat, quality: RollQuality, rarity: i8, num: i8) {
        assert!(is_valid_substat_type(&substat_value));
        assert!(self.current_rolls_for_given(&substat_value, quality.clone(), rarity.clone()) + num <= self.substat_constraint(&substat_value, rarity));

        self.rolls.entry((substat_value.clone(), quality.clone(), rarity))
            .and_modify(|v| *v+=num)
            .or_insert(num);
    }

    /// unrolls a substat
    pub fn unroll(&mut self, substat_value: Stat, quality: RollQuality, rarity: i8, num: i8) {
        assert!(is_valid_substat_type(&substat_value));
        
        let key = (substat_value.clone(), quality.clone(), rarity);
        if let Some(current_rolls) = self.rolls.get_mut(&key) {
            if *current_rolls >= num {
                *current_rolls -= num;
                if *current_rolls == 0 {
                    self.rolls.remove(&key);
                }
            }
        }
    }
}

pub struct ArtifactPiece {
    pub rarity: i8,
    pub level: i8,
    pub stat_type: Stat,
}

pub enum RollQuality{
    MAX,
    HIGH,
    MID,
    LOW,
    AVG //not in game
}

impl RollQuality{
    pub fn multiplier(&self) -> f32 {
        match self {
            RollQuality::MAX => 1.0,
            RollQuality::HIGH => 0.8,
            RollQuality::MID => 0.6,
            RollQuality::LOW => 0.4,
            RollQuality::AVG => 0.7,
        }
    }
}

pub const POSSIBLE_SUB_STATS: &[Stat] = &[
    Stat::FlatHP,
    Stat::HPPercent,
    Stat::FlatATK,
    Stat::ATKPercent,
    Stat::FlatDEF,
    Stat::DEFPercent,
    Stat::ElementalMastery,
    Stat::EnergyRecharge,
    Stat::CritRate,
    Stat::CritDMG,
];

pub fn is_valid_substat_type(stat_type: &Stat) -> bool {
    POSSIBLE_SUB_STATS.contains(stat_type)
}
``` 
use crate::{model::stattable::StatTable, standardize::flatten_str};
use crate::model::statable::Statable;
use crate::model::stat::Stat;
use crate::model::statable::ModifiableStatable;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use serde::Deserialize;
use serde::Serialize;
use std::io::BufReader;
use crate::utils::percentage::*;
use std::fmt;
use std::error::Error;
use anyhow::{Result, anyhow};
use crate::irminsul_data_adaptor::*;

/// factory for creating stattables
pub struct StatTableFactory{}
impl StatTableFactory{

    /// opens a file name in project/data/filename
    fn get_data_file(filename: &str) -> Result<File, std::io::Error> {
        File::open(format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), filename))
    }

    /// reads file system to get character base stats of a given name as a stattable 
    pub fn get_character_base_stats(name: &str) -> Result<StatTable> {
        let file: File = StatTableFactory::get_data_file("characters.json")?;
        let list: CharacterList = serde_json::from_reader(BufReader::new(file))?;
        let stat_list = StatTableFactory::find_match(list.data, name)?;
        
        stat_list.base_stats.last()
            .map(|x| x.to_stattable())
            .ok_or_else(|| anyhow!("failed to get last base stat tuple"))?
    }

    pub fn get_weapon_stats(name: &str) -> Result<StatTable> {
        let file = StatTableFactory::get_data_file("weapons.json")?;
        let list: WeaponList = serde_json::from_reader(BufReader::new(file))?;
        let stat_list: WeaponJSON = StatTableFactory::find_match(list.data, name)?;
        
        stat_list.base_stats.last()
            .map(|x| x.to_stattable())
            .ok_or_else(|| anyhow!("failed to get last base stat tuple"))?
    }



    pub fn get_main_stat_value(rarity: i8, level: i8, stat_type: &Stat) -> Result<f32> {
        if !Self::check_correct_level_for_rarity(level, rarity){
            return Err(anyhow!("invalid level and rarity combo: level={level} rarity={rarity}"));
        }

        let file: File = StatTableFactory::get_data_file("artifactMainStats.json")?;
        let mainstats: AllArtifactMainStatJson = serde_json::from_reader(BufReader::new(file))?;

        let rarity_pool = match rarity {
            5 => mainstats.five_star,
            4 => mainstats.four_star,
            3 => mainstats.three_star,
            2 => mainstats.two_star,
            1 => mainstats.one_star,
            _ => Err(anyhow!("invalid rarity value: {}", rarity))?
        };

        let stat_pool = match stat_type {
            Stat::FlatHP => &rarity_pool.flat_hp,
            Stat::FlatATK => &rarity_pool.flat_atk,
            Stat::HPPercent => &rarity_pool.hp_percent,
            Stat::ATKPercent => &rarity_pool.atk_percent,
            Stat::DEFPercent => &rarity_pool.def_percent,
            Stat::PhysicalDMGBonus => &rarity_pool.physical_dmg_bonus,
            Stat::ElementalMastery => &rarity_pool.elemental_mastery,
            Stat::EnergyRecharge => &rarity_pool.energy_recharge,
            Stat::CritRate => &rarity_pool.crit_rate,
            Stat::CritDMG => &rarity_pool.crit_dmg,
            Stat::HealingBonus => &rarity_pool.healing_bonus,
            s if s.is_elemental_dmg_bonus() => &rarity_pool.elemental_dmg_percent,
            _ => Err(anyhow!("invalid stat_type value: {}", stat_type))?
        };

        stat_pool.get(level as usize)
            .map(|x| *x)
            .ok_or_else(|| anyhow!("invalid level {level}"))
    }

    pub fn get_sub_stat_value(rarity: i32, stat_type: Stat) -> Result<f32> {
        let file: File = StatTableFactory::get_data_file("artifactSubStats.json")?;
        let json: AllArtifactSubStatJson = serde_json::from_reader(BufReader::new(file))?;

        let rarity_pool = match rarity {
            5 => json.five_star,
            4 => json.four_star,
            3 => json.three_star,
            2 => json.two_star,
            1 => json.one_star,
            _ => Err(anyhow!("invalid rarity value: {}", rarity))?
        };

        match stat_type {
            Stat::FlatHP => Ok(rarity_pool.flat_hp),
            Stat::FlatATK => Ok(rarity_pool.flat_atk),
            Stat::FlatDEF => Ok(rarity_pool.flat_def),
            Stat::HPPercent => Ok(rarity_pool.hp_percent),
            Stat::ATKPercent => Ok(rarity_pool.atk_percent),
            Stat::DEFPercent => Ok(rarity_pool.def_percent),
            Stat::ElementalMastery => Ok(rarity_pool.elemental_mastery),
            Stat::EnergyRecharge => Ok(rarity_pool.energy_recharge),
            Stat::CritRate => Ok(rarity_pool.crit_rate),
            Stat::CritDMG => Ok(rarity_pool.crit_dmg),
            _ => Err(anyhow!("invalid stat_type value: {}", stat_type))?
        }
    }

    // pub fn get_sub_stat_value_with_roll(rarity: i32, stat_type: Stat) -> Result<f32> {}

    fn check_correct_level_for_rarity(level: i8, rarity: i8) -> bool {
        match rarity {
            1 => level <= 4,
            2 => level <= 4,
            3 => level <= 12,
            4 => level <= 16,
            5 => level <= 20,
            _ => false,
        }
    }


    fn find_match<T: NamedJSON>(json_list: Vec<T>, name: &str) -> Result<T> {
        let matches = json_list.iter()
            .filter(|c| flatten_str(c.name()) == flatten_str(name) || Self::fuzzy_match(name, c.name()));


        // if matches.clone().count() == 0 {
        //     let matches = json_list.iter()
        //         .filter(|c| StatTableFactory::fuzzy_match(name, c.name()));
        // }

            
        match matches.clone().count() {
            1 => Ok(matches.reduce(|x: &T, y: &T| x).unwrap().clone()),
            0 => Err(anyhow!("No character with name {} found", name)),
            _ => Err(anyhow!("More than 1 character with name {} found", name)),
        }
    }

    pub fn fuzzy_match(needled: &str, haystack: &str) -> bool {
        let needle = flatten_str(needled);
        let haystack = flatten_str(haystack);
        let mut nidx = 0;
        for c in haystack.chars() {
            if c == needle.chars().nth(nidx).unwrap() {
                nidx += 1;
                if nidx == needle.len() {
                    return true;
                }
            }
        }
        false
    }


}


#[cfg(test)] mod tests {
    use super::*;

    #[test] fn get_data_file_works() {
        let file = StatTableFactory::get_data_file("characters.json");
        assert!(file.is_ok())
    }

    #[test] fn get_character_expected() {
        let amber = StatTableFactory::get_character_base_stats("Amber");
        let amber = amber.unwrap();
        assert_eq!(amber.get(&Stat::BaseATK), 223.02);
        assert_eq!(amber.get(&Stat::BaseHP), 9461.18);
        assert_eq!(amber.get(&Stat::BaseDEF), 600.62);
        assert_eq!(amber.get(&Stat::ATKPercent), 0.240);
    }


    #[test] fn fuzzy_match_test() {
        assert!(StatTableFactory::fuzzy_match("ayaka","Kamisato Ayaka"));

    }

    #[test] fn get_chara_fuzzy() {
        let c1 = StatTableFactory::get_character_base_stats("Kamisato Ayaka");
        let c1 = c1.unwrap();
        let c2 = StatTableFactory::get_character_base_stats("ayaka");
        let c2 = c2.unwrap();

        assert_eq!(c1, c2);
    }

    #[test] fn get_weapon_stats_works() {
        let w = StatTableFactory::get_weapon_stats("A Thousand Blazing Suns");
        
        let w = w.unwrap();
        assert_eq!(w.get(&Stat::BaseATK), 741.0);
        assert_eq!(w.get(&Stat::CritRate), 0.11);
    }

    #[test] fn test_get_mainstat_value() {
        assert_eq!(StatTableFactory::get_main_stat_value(5, 20, &Stat::FlatATK).unwrap(), 311.0);
        assert_eq!(StatTableFactory::get_main_stat_value(1, 0, &Stat::FlatATK).unwrap(), 8.0);

        assert_eq!(StatTableFactory::get_main_stat_value(1, 0, &Stat::PyroDMGBonus).unwrap(), 0.031);


        assert_eq!(StatTableFactory::get_main_stat_value(0, 0, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatTableFactory::get_main_stat_value(-1, 0, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatTableFactory::get_main_stat_value(6, 0, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatTableFactory::get_main_stat_value(1, 5, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatTableFactory::get_main_stat_value(5, 21, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatTableFactory::get_main_stat_value(4, 17, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatTableFactory::get_main_stat_value(5, 20, &Stat::BaseATK).is_err(), true);
    }

    #[test] fn test_get_substat_value() {
        assert_eq!(StatTableFactory::get_sub_stat_value(5, Stat::ATKPercent).unwrap(), 0.0583);
        assert_eq!(StatTableFactory::get_sub_stat_value(5, Stat::CritRate).unwrap(), 0.0389);
        assert_eq!(StatTableFactory::get_sub_stat_value(4, Stat::ATKPercent).unwrap(), 0.0466);
        assert_eq!(StatTableFactory::get_sub_stat_value(1, Stat::ATKPercent).unwrap(), 0.0146);

        assert_eq!(StatTableFactory::get_sub_stat_value(0, Stat::BaseATK).is_err(), true);
        assert_eq!(StatTableFactory::get_sub_stat_value(5, Stat::PhysicalDMGBonus).is_err(), true);
    }

}
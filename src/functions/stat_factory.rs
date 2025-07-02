use crate::{model::stattable::StatTable, standardize::flatten_str};
use crate::model::statable::Statable;
use crate::model::stat::Stat;
use crate::model::statable::ModifiableStatable;
use std::str::FromStr;
use serde::Deserialize;
use serde::Serialize;
use crate::utils::percentage::*;
use std::fmt;
use std::error::Error;
use anyhow::{Result, anyhow};
use crate::data::irminsul_adaptor::*;
use once_cell::sync::Lazy;

#[cfg(not(target_arch = "wasm32"))]
use reqwest::Client;

/// factory for creating stattables
pub struct StatFactory{}

// Compile-time cached data using include_str! macro
static CHARACTER_DATA: Lazy<CharacterList> = Lazy::new(|| {
    let json_str = include_str!("../../data/characters.json");
    serde_json::from_str(json_str).expect("Failed to parse characters.json")
});

static WEAPON_DATA: Lazy<WeaponList> = Lazy::new(|| {
    let json_str = include_str!("../../data/weapons.json");
    serde_json::from_str(json_str).expect("Failed to parse weapons.json")
});

static ARTIFACT_MAIN_STAT_DATA: Lazy<AllArtifactMainStatJson> = Lazy::new(|| {
    let json_str = include_str!("../../data/artifactMainStats.json");
    serde_json::from_str(json_str).expect("Failed to parse artifactMainStats.json")
});

static ARTIFACT_SUB_STAT_DATA: Lazy<AllArtifactSubStatJson> = Lazy::new(|| {
    let json_str = include_str!("../../data/artifactSubStats.json");
    serde_json::from_str(json_str).expect("Failed to parse artifactSubStats.json")
});

impl StatFactory{

    /// reads cached character base stats of a given name as a stattable 
    pub fn get_character_base_stats(name: &str) -> Result<StatTable> {
        let stat_list = StatFactory::find_match(CHARACTER_DATA.data.clone(), name)?;
        
        stat_list.base_stats.last()
            .map(|x| x.to_stattable())
            .ok_or_else(|| anyhow!("failed to get last base stat tuple"))?
    }

    /// fetches character base stats from Irminsul API asynchronously
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn fetch_character_base_stats(name: &str) -> Result<StatTable> {
        let client = Client::new();
        let response = client
            .get("https://www.irminsul.moe/api/characters")
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch characters from API: {}", e))?;
        
        let character_list: CharacterList = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse characters JSON: {}", e))?;
        
        let stat_list = StatFactory::find_match(character_list.data, name)?;
        
        stat_list.base_stats.last()
            .map(|x| x.to_stattable())
            .ok_or_else(|| anyhow!("failed to get last base stat tuple"))?
    }

    pub fn get_weapon_base_stats(name: &str) -> Result<StatTable> {
        let stat_list: WeaponJSON = StatFactory::find_match(WEAPON_DATA.data.clone(), name)?;
        
        stat_list.base_stats.last()
            .map(|x| x.to_stattable())
            .ok_or_else(|| anyhow!("failed to get last base stat tuple"))?
    }

    /// fetches weapon stats from Irminsul API asynchronously
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn fetch_weapon_stats(name: &str) -> Result<StatTable> {
        let client = Client::new();
        let response = client
            .get("https://www.irminsul.moe/api/weapons")
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch weapons from API: {}", e))?;
        
        let weapon_list: WeaponList = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse weapons JSON: {}", e))?;
        
        let stat_list: WeaponJSON = StatFactory::find_match(weapon_list.data, name)?;
        
        stat_list.base_stats.last()
            .map(|x| x.to_stattable())
            .ok_or_else(|| anyhow!("failed to get last base stat tuple"))?
    }

    pub fn get_main_stat_value(rarity: i8, level: i8, stat_type: &Stat) -> Result<f32> {
        if !Self::check_correct_level_for_rarity(level, rarity){
            return Err(anyhow!("invalid level and rarity combo: level={level} rarity={rarity}"));
        }

        let rarity_pool = match rarity {
            5 => &ARTIFACT_MAIN_STAT_DATA.five_star,
            4 => &ARTIFACT_MAIN_STAT_DATA.four_star,
            3 => &ARTIFACT_MAIN_STAT_DATA.three_star,
            2 => &ARTIFACT_MAIN_STAT_DATA.two_star,
            1 => &ARTIFACT_MAIN_STAT_DATA.one_star,
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

    pub fn get_sub_stat_value(rarity: i8, stat_type: Stat) -> Result<f32> {
        let rarity_pool = match rarity {
            5 => &ARTIFACT_SUB_STAT_DATA.five_star,
            4 => &ARTIFACT_SUB_STAT_DATA.four_star,
            3 => &ARTIFACT_SUB_STAT_DATA.three_star,
            2 => &ARTIFACT_SUB_STAT_DATA.two_star,
            1 => &ARTIFACT_SUB_STAT_DATA.one_star,
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

    pub fn check_correct_level_for_rarity(level: i8, rarity: i8) -> bool {
        if level < 0 {
            return false;
        }
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
            .filter(|c| 
                flatten_str(c.name()) == flatten_str(name) 
                || Self::fuzzy_match(name, c.name()
            ));

        // if matches.clone().count() == 0 {
        //     let matches = json_list.iter()
        //         .filter(|c| StatFactory::fuzzy_match(name, c.name()));
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

    #[test] fn get_character_expected() {
        let amber = StatFactory::get_character_base_stats("Amber");
        let amber = amber.unwrap();
        assert_eq!(amber.get(&Stat::BaseATK), 223.02);
        assert_eq!(amber.get(&Stat::BaseHP), 9461.18);
        assert_eq!(amber.get(&Stat::BaseDEF), 600.62);
        assert_eq!(amber.get(&Stat::ATKPercent), 0.240);
    }

    #[test] fn fuzzy_match_test() {
        assert!(StatFactory::fuzzy_match("ayaka","Kamisato Ayaka"));
    }

    #[test] fn get_chara_fuzzy() {
        let c1 = StatFactory::get_character_base_stats("Kamisato Ayaka");
        let c1 = c1.unwrap();
        let c2 = StatFactory::get_character_base_stats("ayaka");
        let c2 = c2.unwrap();

        assert_eq!(c1, c2);
    }

    #[test] fn get_weapon_base_stats_works() {
        let w = StatFactory::get_weapon_base_stats("A Thousand Blazing Suns");
        
        let w = w.unwrap();
        assert_eq!(w.get(&Stat::BaseATK), 741.0);
        assert_eq!(w.get(&Stat::CritRate), 0.11);
    }

    #[test] fn test_get_mainstat_value() {
        assert_eq!(StatFactory::get_main_stat_value(5, 20, &Stat::FlatATK).unwrap(), 311.0);
        assert_eq!(StatFactory::get_main_stat_value(1, 0, &Stat::FlatATK).unwrap(), 8.0);

        assert_eq!(StatFactory::get_main_stat_value(1, 0, &Stat::PyroDMGBonus).unwrap(), 0.031);

        assert_eq!(StatFactory::get_main_stat_value(0, 0, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatFactory::get_main_stat_value(-1, 0, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatFactory::get_main_stat_value(6, 0, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatFactory::get_main_stat_value(1, 5, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatFactory::get_main_stat_value(5, 21, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatFactory::get_main_stat_value(4, 17, &Stat::FlatATK).is_err(), true);
        assert_eq!(StatFactory::get_main_stat_value(5, 20, &Stat::BaseATK).is_err(), true);
    }

    #[test] fn test_get_substat_value() {
        assert_eq!(StatFactory::get_sub_stat_value(5, Stat::ATKPercent).unwrap(), 0.0583);
        assert_eq!(StatFactory::get_sub_stat_value(5, Stat::CritRate).unwrap(), 0.0389);
        assert_eq!(StatFactory::get_sub_stat_value(4, Stat::ATKPercent).unwrap(), 0.0466);
        assert_eq!(StatFactory::get_sub_stat_value(1, Stat::ATKPercent).unwrap(), 0.0146);

        assert_eq!(StatFactory::get_sub_stat_value(0, Stat::BaseATK).is_err(), true);
        assert_eq!(StatFactory::get_sub_stat_value(5, Stat::PhysicalDMGBonus).is_err(), true);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[tokio::test]
    async fn test_fetch_character_base_stats() {
        let amber = StatFactory::fetch_character_base_stats("Amber").await;
        let amber = amber.unwrap();
        assert_eq!(amber.get(&Stat::BaseATK), 223.02);
        assert_eq!(amber.get(&Stat::BaseHP), 9461.18);
        assert_eq!(amber.get(&Stat::BaseDEF), 600.62);
        assert_eq!(amber.get(&Stat::ATKPercent), 0.240);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[tokio::test]
    async fn test_fetch_character_fuzzy() {
        let c1 = StatFactory::fetch_character_base_stats("Kamisato Ayaka").await;
        let c1 = c1.unwrap();
        let c2 = StatFactory::fetch_character_base_stats("ayaka").await;
        let c2 = c2.unwrap();

        assert_eq!(c1, c2);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[tokio::test]
    async fn test_fetch_weapon_stats() {
        let w = StatFactory::fetch_weapon_stats("A Thousand Blazing Suns").await;
        let w = w.unwrap();
        assert_eq!(w.get(&Stat::BaseATK), 741.0);
        assert_eq!(w.get(&Stat::CritRate), 0.11);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[tokio::test]
    async fn test_fetch_character_not_found() {
        let result = StatFactory::fetch_character_base_stats("NonExistentCharacter").await;
        assert!(result.is_err());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[tokio::test]
    async fn test_fetch_weapon_not_found() {
        let result = StatFactory::fetch_weapon_stats("NonExistentWeapon").await;
        assert!(result.is_err());
    }
}
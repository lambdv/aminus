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



    pub fn get_main_stat_value(rarity: i32, level: i32, type_: Stat) -> Result<f64> {
        let file: File = StatTableFactory::get_data_file("artifactMainStats.json")?;

        Ok(0.0)
    }

    pub fn get_sub_stat_value(rarity: i32, type_: Stat) -> Result<f64> {
        Ok(0.0)
    }

    fn check_correct_level_for_rarity(level: i32, rarity: i32) -> bool {
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
        let matches = json_list.iter().filter(|c| StatTableFactory::fuzzy_match(name, c.name()));
        match matches.clone().count() {
            1 => Ok(matches.reduce(|x: &T, y: &T| x).unwrap().clone()),
            0 => Err(anyhow!("No character with name {} found", name)),
            _ => Err(anyhow!("More than 1 character with name {} found", name)),
        }
    }

    fn fuzzy_match(needled: &str, haystack: &str) -> bool {
        let needle = flatten_str(needled);
        let haystack = flatten_str(haystack);
        let needle_chars: Vec<char> = needle.chars().collect();
        let mut haystack_chars: Vec<char> = haystack.chars().collect();
        haystack_chars.sort_unstable();
        needle_chars.iter().all(|c| haystack_chars.binary_search(c).is_ok())
    }


}

#[cfg(test)] mod tests {
    use super::*;

    #[test] fn get_data_file_works() {
        let file = StatTableFactory::get_data_file("characters.json");
        assert!(file.is_ok())
    }

    #[test] fn get_amber_local() {
        let amber = StatTableFactory::get_character_base_stats("amber");
        let amber = amber.unwrap();
        assert_eq!(amber.get(&Stat::BaseATK), 223.02);
        assert_eq!(amber.get(&Stat::BaseHP), 9461.18);
        assert_eq!(amber.get(&Stat::BaseDEF), 600.62);
        assert_eq!(amber.get(&Stat::ATKPercent), 0.240);
    }

    #[test] fn get_weapon_stats_works() {
        let w = StatTableFactory::get_weapon_stats("A Thousand Blazing Suns");
        
        let w = w.unwrap();
        assert_eq!(w.get(&Stat::BaseATK), 741.0);
        assert_eq!(w.get(&Stat::CritRate), 0.11);
    }
}
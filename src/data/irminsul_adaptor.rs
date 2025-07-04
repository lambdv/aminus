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

pub trait NamedJSON: Clone {
    fn name(&self) -> &str;
}

#[derive(Debug, Deserialize)] 
pub struct CharacterList {
    pub data: Vec<CharacterJSON>,
}

#[derive(Debug, Deserialize
)] pub struct WeaponList {
    pub data: Vec<WeaponJSON>,
}

#[derive(Debug, Deserialize, Clone)] 
pub struct CharacterJSON {
    //pub id: String,
    //pub index: Option<u32>,
    pub name: String,
    // pub key: String,
    //pub title: String,
    pub rarity: u8,
    pub element: String,
    // pub vision: String,
    pub weapon: String,
    // pub release_date: String,
    // pub release_date_epoch: u64,
    // pub constellation: String,
    // pub birthday: String,
    // pub affiliation: String,
    // pub region: String,
    // pub special_dish: String,
    // pub alternate_title: Option<String>,
    // pub description: String,
    pub ascension_stat: String,
    pub base_stats: Vec<CharacterBaseStatJSON>,
    //pub ascension_costs: Vec<CharacterAscensionCostJSON>,
    //pub talents: Vec<CharacterTalentJSON>,
    //pub passives: Vec<CharacterPassiveJSON>,
    //pub constellations: Vec<CharacterConstellationJSON>,
}

impl NamedJSON for CharacterJSON {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterBaseStatJSON {
    #[serde(rename = "LVL")]
    pub lvl: String,
    #[serde(rename = "BaseHP")]
    pub base_hp: String,
    #[serde(rename = "BaseATK")]
    pub base_atk: String,
    #[serde(rename = "BaseDEF")]
    pub base_def: String,
    #[serde(rename = "AscensionStatType")]
    pub stat_type: String,
    #[serde(rename = "AscensionStatValue")]
    pub stat_value: String,
    #[serde(rename = "AscensionPhase")]
    pub phase: u8,
}

impl CharacterBaseStatJSON {
    pub fn to_stattable(&self) -> Result<StatTable>{
        let substat = Stat::from_str(&self.stat_type.as_str())
            .map_err(|e| anyhow!("failed parse string to stat"))?;

        let substat_value = parse_percentage(self.stat_value.as_str())
            .map_err(|e| anyhow!("failed to parse string to f32: {}",e))?;

        return Ok(StatTable::of(&[
            (Stat::BaseHP, self.base_hp.parse::<f32>().unwrap()),
            (Stat::BaseATK, self.base_atk.parse::<f32>().unwrap()),
            (Stat::BaseDEF, self.base_def.parse::<f32>().unwrap()),
            (Stat::CritRate, 0.05),
            (Stat::CritDMG, 0.5),
            (Stat::EnergyRecharge, 1.0),
            (substat, substat_value),
        ]));
    }
}



#[derive(Debug, Deserialize)]
pub struct CharacterConstellationJSON {
    pub level: u8,
    pub name: String,
    pub description: String,
    pub properties: Vec<serde_json::Value>, // use concrete type if structure known
}

#[derive(Debug, Deserialize)]
pub struct CharacterTalentJSON {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub attributes: Option<Vec<CharacterTalentAttributeJSON>>,
    pub properties: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterTalentAttributeJSON {
    pub hit: String,
    pub values: Vec<serde_json::Value>, // numbers or strings, hence generic
}

#[derive(Debug, Deserialize)]
pub struct CharacterPassiveJSON {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub properties: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterAscensionCostJSON {
    #[serde(rename = "AscensionPhase")]
    pub phase: u8,
    pub materials: Vec<CharacterAscensionMaterialJSON>,
}

#[derive(Debug, Deserialize)]
pub struct CharacterAscensionMaterialJSON {
    pub name: String,
    pub amount: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeaponJSON {
    //pub id: String,
    pub name: String,
    //pub key: String,
    pub rarity: u8,
    //pub description: String,
    pub category: String,
    //pub series: String,
    //pub release_date: String,
    //pub release_date_epoch: i64,
    //pub base_atk_min: f32,
    //pub base_atk_max: f32,
    //pub sub_stat_type: String,
    //pub sub_stat_value_min: String,
    //pub sub_stat_value_max: String,
    //pub refinement_name: String,
    //pub refinements: Vec<String>,
    pub base_stats: Vec<WeaponBaseStatJSON>,
}

impl NamedJSON for WeaponJSON {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WeaponBaseStatJSON {
    pub level: String,
    pub base_atk: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_stat_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_stat_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascension_phase: Option<u8>,
}

impl WeaponBaseStatJSON {
    pub fn to_stattable(&self) -> Result<StatTable>{
        let mut substat_pair = Option::<(Stat, f32)>::None;

        if self.sub_stat_type.as_ref().is_some() && self.sub_stat_value.as_ref().is_some(){
            let substat = Stat::from_str(self.sub_stat_type.as_ref().unwrap().as_str())
                .map_err(|e| anyhow!("failed parse string to stat"))?;
            let substat_value = parse_percentage(self.sub_stat_value.as_ref().unwrap().as_str())
                .map_err(|e| anyhow!("failed to parse string to f32: {}",e))?;
            substat_pair = Some((substat, substat_value));   
        }

        let mut stats = StatTable::new();
        stats.add(&Stat::BaseATK, self.base_atk.parse::<f32>().unwrap());
        
        if substat_pair.is_some() {
            stats.add(&substat_pair.unwrap().0, substat_pair.unwrap().1);
        }
        
        Ok(stats)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllArtifactMainStatJson{
    #[serde(rename = "5star")]
    pub five_star:ArtifactMainStatJson,
    #[serde(rename = "4star")]
    pub four_star:ArtifactMainStatJson,
    #[serde(rename = "3star")]
    pub three_star:ArtifactMainStatJson,
    #[serde(rename = "2star")]
    pub two_star:ArtifactMainStatJson,
    #[serde(rename = "1star")]
    pub one_star:ArtifactMainStatJson,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactMainStatJson{
    // #[serde(rename = "Level")]
    // level: Vec<String>,
    #[serde(rename = "FlatHP")]
    pub flat_hp: Vec<f32>,
    #[serde(rename = "FlatATK")]
    pub flat_atk: Vec<f32>,
    #[serde(rename = "HPPercent")]
    pub hp_percent: Vec<f32>,
    #[serde(rename = "ATKPercent")]
    pub atk_percent: Vec<f32>,
    #[serde(rename = "DEFPercent")]
    pub def_percent: Vec<f32>,
    #[serde(rename = "PhysicalDMGBonus")]
    pub physical_dmg_bonus: Vec<f32>,
    #[serde(rename = "ElementalDMGPercent")]
    pub  elemental_dmg_percent: Vec<f32>,
    #[serde(rename = "ElementalMastery")]
    pub elemental_mastery: Vec<f32>,
    #[serde(rename = "EnergyRecharge")]
    pub energy_recharge: Vec<f32>,
    #[serde(rename = "CritRate")]
    pub crit_rate: Vec<f32>,
    #[serde(rename = "CritDMG")]
    pub crit_dmg: Vec<f32>,
    #[serde(rename = "HealingBonus")]
    pub healing_bonus: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AllArtifactSubStatJson{
    #[serde(rename = "5star")]
    pub five_star:ArtifactSubStatJson,
    #[serde(rename = "4star")]
    pub four_star:ArtifactSubStatJson,
    #[serde(rename = "3star")]
    pub three_star:ArtifactSubStatJson,
    #[serde(rename = "2star")]
    pub two_star:ArtifactSubStatJson,
    #[serde(rename = "1star")]
    pub one_star:ArtifactSubStatJson,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactSubStatJson{
    // #[serde(rename = "Level")]
    // level: Vec<String>,
    #[serde(rename = "FlatHP")]
    pub flat_hp: f32,
    #[serde(rename = "FlatATK")]
    pub flat_atk: f32,
    #[serde(rename = "FlatDEF")]
    pub flat_def: f32,
    #[serde(rename = "HPPercent")]
    pub hp_percent: f32,
    #[serde(rename = "ATKPercent")]
    pub atk_percent: f32,
    #[serde(rename = "DEFPercent")]
    pub def_percent: f32,
    #[serde(rename = "ElementalMastery")]
    pub elemental_mastery: f32,
    #[serde(rename = "EnergyRecharge")]
    pub energy_recharge: f32,
    #[serde(rename = "CritRate")]
    pub crit_rate: f32,
    #[serde(rename = "CritDMG")]
    pub crit_dmg: f32,
}




#[cfg(test)] mod tests {
    use super::*;

    #[test] fn test_to_stattable() {
        let w = WeaponBaseStatJSON {
            level: "1".to_string(),
            base_atk: "510.0".to_string(),
            sub_stat_type: Some("PhysicalDMGBonus".to_string()),
            sub_stat_value: Some("51.7".to_string()),
            ascension_phase: None,
        };
        let w = w.to_stattable().unwrap();
        assert_eq!(w.get(&Stat::BaseATK), 510.0);
        assert_eq!(w.get(&Stat::PhysicalDMGBonus), 51.7);
    }
}
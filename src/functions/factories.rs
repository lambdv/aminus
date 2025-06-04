pub mod factories{
    use crate::{model::stattable::StatTable, standardize::flatten_str};
    use crate::model::statable::Statable;
    use crate::model::stat::Stat;
    use std::fs::File;
    use std::io::prelude::*;
    use std::str::FromStr;
    use serde::Deserialize;
    use std::io::BufReader;
    use crate::factories::jsontypes::*;
    use crate::utils::percentage::*;
    use std::fmt;
    use std::error::Error;
    use anyhow::{Result, anyhow};
    
    fn get_data_file(filename: &str) -> Result<File, std::io::Error> {
        File::open(format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), filename))
    }


    // pub async fn character_base_stats(name: String, _level: String, _ascended: bool) -> StatTable{
    //     let res = reqwest::get(format!("{}characters/{}", API, name)).await
    //         .unwrap_or_else(|e| panic!("Error fetching data from API: {}", e));
    //     if res.status() != 200 { 
    //         panic!("Error fetching data from API");
    //     }
    //     let _data = res.json::<serde_json::Value>().await
    //         .unwrap_or_else(|e| panic!("Error parsing data from API {}", e)); 
    //     StatTable::new()
    // }


    pub fn get_character_base_stats(name: &str) -> Result<StatTable> {
        //load data
        let file = get_data_file("characters.json")?;
        let characters: CharacterList = serde_json::from_reader(BufReader::new(file))?;
        
        //check for 1 match
        let matches = characters.data.iter().filter(|c| flatten_str(c.name.as_str())==flatten_str(name));
        let data = match matches.clone().count() {
            0 => Err(anyhow!("No character with name {} found", name)),
            1 => Ok(matches.reduce(|x: &CharacterJSON, y: &CharacterJSON| x).unwrap()),
            _ => Err(anyhow!("More than 1 character with name {} found", name)),
        }?;

        //parse and return
        let data = data.base_stats.last().ok_or_else(||anyhow!("failed to get last base stat tuple"))?;
        return character_json_to_stattable(data);
    }


    fn character_json_to_stattable(data: &CharacterBaseStatJSON) -> Result<StatTable>{
        let substat = Stat::from_str(&data.AscensionStatType.as_str())
            .map_err(|e| anyhow!("failed parse string to stat"))?;

        let substat_value = parse_percentage(data.AscensionStatValue.as_str())
            .map_err(|e| anyhow!("failed to parse string to f32: {}",e))?;

        return Ok(StatTable::of(&[
            (Stat::BaseHP, data.BaseHP.parse::<f32>().unwrap()),
            (Stat::BaseATK, data.BaseATK.parse::<f32>().unwrap()),
            (Stat::BaseDEF, data.BaseDEF.parse::<f32>().unwrap()),
            (substat, substat_value),
        ]));
    }

    fn weapon_json_to_stattable(data: &CharacterBaseStatJSON) -> Result<StatTable>{
        let substat = Stat::from_str(&data.AscensionStatType.as_str())
            .map_err(|e| anyhow!("failed parse string to stat"))?;

        let substat_value = parse_percentage(data.AscensionStatValue.as_str())
            .map_err(|e| anyhow!("failed to parse string to f32: {}",e))?;

        return Ok(StatTable::of(&[
            (Stat::BaseHP, data.BaseHP.parse::<f32>().unwrap()),
            (Stat::BaseATK, data.BaseATK.parse::<f32>().unwrap()),
            (Stat::BaseDEF, data.BaseDEF.parse::<f32>().unwrap()),
            (substat, substat_value),
        ]));
    }
    

    #[cfg(test)] mod tests {
        use super::*;

        #[test] fn get_data_file_works() {
            let file = get_data_file("characters.json");
            assert!(file.is_ok())
        }

        #[test] fn get_amber_local() {
            let amber = get_character_base_stats("amber");
            let amber = amber.unwrap();
            assert_eq!(amber.get(&Stat::BaseATK), 223.02);
            assert_eq!(amber.get(&Stat::BaseHP), 9461.18);
            assert_eq!(amber.get(&Stat::BaseDEF), 600.62);
            assert_eq!(amber.get(&Stat::ATKPercent), 0.240);

        }
    }
}

mod jsontypes{
    use serde::Deserialize;
    
    #[derive(Debug, Deserialize)] pub struct CharacterList {
        pub data: Vec<CharacterJSON>,
    }

    #[derive(Debug, Deserialize)] pub struct CharacterJSON {
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
    
    #[derive(Debug, Deserialize)]
    pub struct CharacterBaseStatJSON {
        pub LVL: String,
        pub BaseHP: String,
        pub BaseATK: String,
        pub BaseDEF: String,
        pub AscensionStatType: String,
        pub AscensionStatValue: String,
        pub AscensionPhase: u8,
    }
    
    // #[derive(Debug, Deserialize)]
    // pub struct CharacterConstellationJSON {
    //     pub level: u8,
    //     pub name: String,
    //     pub description: String,
    //     pub properties: Vec<serde_json::Value>, // use concrete type if structure known
    // }
    
    // #[derive(Debug, Deserialize)]
    // pub struct CharacterTalentJSON {
    //     pub name: String,
    //     #[serde(rename = "type")]
    //     pub type_: String,
    //     pub description: String,
    //     pub attributes: Option<Vec<CharacterTalentAttributeJSON>>,
    //     pub properties: Vec<serde_json::Value>,
    // }
    
    // #[derive(Debug, Deserialize)]
    // pub struct CharacterTalentAttributeJSON {
    //     pub hit: String,
    //     pub values: Vec<serde_json::Value>, // numbers or strings, hence generic
    // }
    
    // #[derive(Debug, Deserialize)]
    // pub struct CharacterPassiveJSON {
    //     pub name: String,
    //     #[serde(rename = "type")]
    //     pub type_: String,
    //     pub description: String,
    //     pub properties: Option<Vec<serde_json::Value>>,
    // }
    
    // #[derive(Debug, Deserialize)]
    // pub struct CharacterAscensionCostJSON {
    //     pub AscensionPhase: u8,
    //     pub materials: Vec<CharacterAscensionMaterialJSON>,
    // }
    
    // #[derive(Debug, Deserialize)]
    // pub struct CharacterAscensionMaterialJSON {
    //     pub name: String,
    //     pub amount: String,
    // }

}

    
// #[derive(Debug)]
// pub enum CharacterError{
//     FileOpenError(std::io::Error),
//     DeserializeError(serde_json::Error),
//     NotFound(String),
//     Duplicate(String),
//     MissingBaseStat,
//     InvalidStat(String),
//     InvalidPercentage(String),
// }
// impl fmt::Display for CharacterError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CharacterError::FileOpenError(e) => write!(f, "File open error: {}", e),
//             CharacterError::DeserializeError(e) => write!(f, "Deserialization error: {}", e),
//             CharacterError::NotFound(name) => write!(f, "Character '{}' not found", name),
//             CharacterError::Duplicate(name) => write!(f, "More than one character named '{}'", name),
//             CharacterError::MissingBaseStat => write!(f, "Missing base stat entry"),
//             CharacterError::InvalidStat(s) => write!(f, "Invalid stat: {}", s),
//             CharacterError::InvalidPercentage(s) => write!(f, "Invalid percentage: {}", s),
//         }
//     }
// }

// impl Error for CharacterError {}
pub mod factories{
    use crate::model::stattable::StatTable;
    use crate::model::statable::Statable;
    use crate::model::stat::Stat;
    use std::fs::File;
    use std::io::prelude::*;
    use std::str::FromStr;
    use serde::Deserialize;
    use std::io::BufReader;
    use serde_json::Result;
    use crate::factories::jsontypes::*;
    use crate::utils::percentage::*;


    // const DATA_DIR: &str = format!("{}../../data", env!("CARGO_MANIFEST_DIR"));

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
        let file = File::open(format!("{}/data/characters.json", env!("CARGO_MANIFEST_DIR"))).unwrap_or_else(|e| panic!("Opening file fail"));
        let reader= BufReader::new(file);
        let characters: CharacterList = serde_json::from_reader(reader).unwrap_or_else(|e| panic!("serealizaiton failed"));

        for chara in characters.data {
            if chara.name.to_lowercase() == name.to_lowercase() {
                let data: &CharacterBaseStatJSON= chara.base_stats.last().unwrap_or_else(|| panic!("Failed to get last base stat"));
                let substat = Stat::from_str(data.AscensionStatType.as_str()).unwrap_or_else(|e| {
                    panic!("Failed to parse AscensionStatType of type {}", data.AscensionStatType.as_str())
                });
                let substat_value = parse_percentage(data.AscensionStatValue.as_str()).unwrap_or_else(|e| panic!("Failed to parse AscensionStatValue"));

                return Ok(StatTable::of(&[
                    (Stat::BaseHP, data.BaseHP.parse::<f32>().unwrap()),
                    (Stat::BaseATK, data.BaseATK.parse::<f32>().unwrap()),
                    (Stat::BaseDEF, data.BaseDEF.parse::<f32>().unwrap()),
                    (substat, substat_value),
                ]));
            }
        }
        panic!("could not find character")
    }

    #[cfg(test)] mod tests {
        use super::*;
        // #[test] fn test_get_character_base_stats() {
        //     get_character_base_stats("amber");
        // }
        #[test] fn get_amber_local() {
            let amber = get_character_base_stats("amber");
            assert!(amber.is_ok());
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
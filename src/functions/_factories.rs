pub mod factories{
    use crate::model::stattable::StatTable;
    use crate::model::statable::Statable;
    //use crate::model::stat::Stat;
    
    const API: &str = "https://www.irminsul.moe/api/";
    
    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct Chara {
        pub name: String,
        pub key: String,
        pub title: String,
        pub rarity: u8,
        pub element: String,
        pub vision: String,
        pub weapon: String,
        pub release_date: String,
        pub release_date_epoch: u64,
        pub constellation: String,
        pub birthday: String,
        pub affiliation: String,
        pub region: String,
        pub special_dish: String,
        pub alternate_title: String,
        pub description: String,
        pub ascension_stat: String,
        pub base_stats: Vec<BaseStats>,
    }

    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct BaseStats {
        pub lvl: String,
        pub base_hp: String,
        pub base_atk: String,
        pub base_def: String,
        pub ascension_stat_type: String,
        pub ascension_stat_value: String,
        pub ascension_phase: u8,
    }

    pub async fn character_base_stats(name: String, _level: String, _ascended: bool) -> StatTable{
        let res = reqwest::get(format!("{}characters/{}", API, name)).await
            .unwrap_or_else(|e| panic!("Error fetching data from API: {}", e));
        
        if res.status() != 200 { 
            panic!("Error fetching data from API");
        }

        let _data = res.json::<serde_json::Value>().await
            .unwrap_or_else(|e| panic!("Error parsing data from API {}", e)); 

        StatTable::new()
    }

    
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::stat::Stat;
        #[test] fn test_create_character() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let future_character = character_base_stats("amber".to_string(), "90/90".to_string(), true);
            let character = rt.block_on(future_character);
            assert_eq!(character.get(&Stat::FlatATK), 0.0);
        }
    }
}



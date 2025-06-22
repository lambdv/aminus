// Re-export the Stat enum and its functions from aminus
pub use aminus::model::stat::{Stat, get_stat_name, is_elemental_dmg_bonus};

// Re-export the static method too
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "fromString")]
pub fn stat_from_string(name: &str) -> Option<Stat> {
    aminus::model::stat::Stat::from_str_js(name)
}

#[cfg(test)] 
mod tests {
    use super::*;
    
    #[test] 
    fn it_works() {
        let stat = Stat::FlatHP;
        println!("{}", get_stat_name(stat));
    }
}

use wasm_bindgen::prelude::*;
use js_sys;

// ============================================================================
// Frozen stat objects for easier JS usage
// ============================================================================

#[wasm_bindgen]
pub fn create_frozen_stats() -> js_sys::Object {
    let stats = js_sys::Object::new();
    
    // Base stats
    js_sys::Reflect::set(&stats, &"BaseHP".into(), &0u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"FlatHP".into(), &1u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"HPPercent".into(), &2u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"BaseATK".into(), &3u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"FlatATK".into(), &4u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"ATKPercent".into(), &5u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"BaseDEF".into(), &6u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"FlatDEF".into(), &7u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"DEFPercent".into(), &8u32.into()).unwrap();
    
    // Combat stats
    js_sys::Reflect::set(&stats, &"ElementalMastery".into(), &9u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"CritRate".into(), &10u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"CritDMG".into(), &11u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"EnergyRecharge".into(), &12u32.into()).unwrap();
    
    // Damage bonuses
    js_sys::Reflect::set(&stats, &"DMGBonus".into(), &13u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"ElementalDMGBonus".into(), &14u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"PyroDMGBonus".into(), &15u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"CryoDMGBonus".into(), &16u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"GeoDMGBonus".into(), &17u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"DendroDMGBonus".into(), &18u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"ElectroDMGBonus".into(), &19u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"HydroDMGBonus".into(), &20u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"AnemoDMGBonus".into(), &21u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"PhysicalDMGBonus".into(), &22u32.into()).unwrap();
    
    // Attack type bonuses
    js_sys::Reflect::set(&stats, &"NormalATKDMGBonus".into(), &23u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"ChargeATKDMGBonus".into(), &24u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"PlungeATKDMGBonus".into(), &25u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"SkillDMGBonus".into(), &26u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"BurstDMGBonus".into(), &27u32.into()).unwrap();
    
    // Other bonuses
    js_sys::Reflect::set(&stats, &"HealingBonus".into(), &28u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"None".into(), &29u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"ReactionBonus".into(), &30u32.into()).unwrap();
    
    // Defense modifiers
    js_sys::Reflect::set(&stats, &"DefReduction".into(), &31u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"DefIgnore".into(), &32u32.into()).unwrap();
    
    // Resistance reductions
    js_sys::Reflect::set(&stats, &"PyroResistanceReduction".into(), &33u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"HydroResistanceReduction".into(), &34u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"ElectroResistanceReduction".into(), &35u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"CryoResistanceReduction".into(), &36u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"AnemoResistanceReduction".into(), &37u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"GeoResistanceReduction".into(), &38u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"DendroResistanceReduction".into(), &39u32.into()).unwrap();
    js_sys::Reflect::set(&stats, &"PhysicalResistanceReduction".into(), &40u32.into()).unwrap();
    
    // Freeze the object to make it immutable
    js_sys::Object::freeze(&stats);
    stats
}

#[wasm_bindgen]
pub fn create_frozen_elements() -> js_sys::Object {
    let elements = js_sys::Object::new();
    
    js_sys::Reflect::set(&elements, &"Pyro".into(), &0u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Hydro".into(), &1u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Electro".into(), &2u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Cryo".into(), &3u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Anemo".into(), &4u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Geo".into(), &5u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Dendro".into(), &6u32.into()).unwrap();
    js_sys::Reflect::set(&elements, &"Physical".into(), &7u32.into()).unwrap();
    
    js_sys::Object::freeze(&elements);
    elements
}

#[wasm_bindgen]
pub fn create_frozen_damage_types() -> js_sys::Object {
    let damage_types = js_sys::Object::new();
    
    js_sys::Reflect::set(&damage_types, &"Normal".into(), &0u32.into()).unwrap();
    js_sys::Reflect::set(&damage_types, &"Charged".into(), &1u32.into()).unwrap();
    js_sys::Reflect::set(&damage_types, &"Plunging".into(), &2u32.into()).unwrap();
    js_sys::Reflect::set(&damage_types, &"Skill".into(), &3u32.into()).unwrap();
    js_sys::Reflect::set(&damage_types, &"Burst".into(), &4u32.into()).unwrap();
    
    js_sys::Object::freeze(&damage_types);
    damage_types
}

#[wasm_bindgen]
pub fn create_frozen_base_scalings() -> js_sys::Object {
    let scalings = js_sys::Object::new();
    
    js_sys::Reflect::set(&scalings, &"ATK".into(), &0u32.into()).unwrap();
    js_sys::Reflect::set(&scalings, &"DEF".into(), &1u32.into()).unwrap();
    js_sys::Reflect::set(&scalings, &"HP".into(), &2u32.into()).unwrap();
    
    js_sys::Object::freeze(&scalings);
    scalings
}



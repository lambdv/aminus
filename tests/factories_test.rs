use aminus::functions::stat_factory::StatFactory;
use aminus::model::stat::Stat;
use aminus::model::statable::Statable;

#[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching character stats from Irminsul API...");
    
    // Fetch character stats asynchronously
    let amber_stats = StatFactory::fetch_character_base_stats("Amber").await?;
    println!("Amber Base ATK: {}", amber_stats.get(&Stat::BaseATK));
    println!("Amber Base HP: {}", amber_stats.get(&Stat::BaseHP));
    println!("Amber ATK%: {}", amber_stats.get(&Stat::ATKPercent));
    
    // Fetch weapon stats asynchronously
    let weapon_stats = StatFactory::fetch_weapon_stats("A Thousand Blazing Suns").await?;
    println!("Weapon Base ATK: {}", weapon_stats.get(&Stat::BaseATK));
    println!("Weapon Crit Rate: {}", weapon_stats.get(&Stat::CritRate));
    
    // Compare with local data (should be the same)
    let amber_local = StatFactory::get_character_base_stats("Amber")?;
    let weapon_local = StatFactory::get_weapon_base_stats("A Thousand Blazing Suns")?;
    
    println!("Local vs API - Amber Base ATK: {} vs {}", 
             amber_local.get(&Stat::BaseATK), amber_stats.get(&Stat::BaseATK));
    println!("Local vs API - Weapon Base ATK: {} vs {}", 
             weapon_local.get(&Stat::BaseATK), weapon_stats.get(&Stat::BaseATK));
    
    Ok(())
} 
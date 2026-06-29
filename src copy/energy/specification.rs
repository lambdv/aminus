use crate::model::stat::Element;

//character energy data

/**
 * information about character energy.
 * includes datamined and gameplay tested information not found in game
 */
pub struct CharacterEnergySpec{
    character_name: String,
    element: Element,
    skills: [SkillEnergySpec;3],
    burst_cooldown: f32,
    burst_energy: i8,
    burst_discount: i8,
    addtional_er: Option<i8>, //mainly for razor

    //addtional string info
    //help flexable
    //help fixed
    //error flexable
    //error fixed
}

/**
 * energy information about a character elemetnal skill
 */
pub struct SkillEnergySpec{
    label: String,
    particles: f32,
    variance: f32,
    per_second: f32,
    duration: i8,
    cool_down: f32
    
}


//energy constaints
pub const SAME_ELEMENT: i8 = 3;
pub const NO_ELEMENT: i8 = 2; //aka clear or white elemebt
pub const DIFFERENT_ELEMENT: i8 = 1;
pub const ORB_MULTIPLIER: i8 = 6; //orb is just a particle multiplied by 6
pub const OFF_FIELD_4_PARTY_MULTIPLIER: f32 = 0.6;
pub const OFF_FIELD_3_PARTY_MULTIPLIER: f32 = 0.7;
pub const OFF_FIELD_2_PARTY_MULTIPLIER: f32 = 0.8;
//note: onfield is 1



//enemy particle energy

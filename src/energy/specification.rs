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


//energy constents
const RIGHT_ELEMENT: i8 = 3;
const NO_ELEMENT: i8 = 2;
const WRONG_ELEMENT: i8 = 1;
const ORB: i8 = 6;
const OFF_FIELD_MULTIPLIER: f32 = 0.6;
//onfield is 1




//enemy particle energy

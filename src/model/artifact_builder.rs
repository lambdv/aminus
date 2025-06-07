use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::stat::Stat;

//type ArtifactPiece = (f32, f32, Stat);
pub enum RollQuality{
    MAX,
    HIGH,
    MID,
    LOW,
    AVG //not in game
}
impl RollQuality{
    pub fn multiplier(&self) -> f32 {
        match self {
            RollQuality::MAX => 1.0,
            RollQuality::HIGH => 0.9,
            RollQuality::MID => 0.8,
            RollQuality::LOW => 0.7,
            RollQuality::AVG => (1.0+0.9+0.8+0.7)/4.0 //kqm calculation standard
        }
    }
}

pub struct ArtifactPiece {
    pub rarity: i8,
    pub level: i8,
    pub stat_type: Stat,
}

pub struct ArtifactBuilder{
    pub flower: Option<ArtifactPiece>,
    pub feather: Option<ArtifactPiece>,
    pub sands: Option<ArtifactPiece>,
    pub goblet: Option<ArtifactPiece>,
    pub circlet: Option<ArtifactPiece>,

    pub rolls: std::collections::HashMap<
        Stat,
        //[(RollQuality,i8);5], //small map to store number of roles colored by quality
        Vec<RollQuality>
        //std::collections::HashMap<
        //    RollQuality,
        //    i8
        //>
    >

 
}

impl ArtifactBuilder{
    
    pub fn new(
        flower: Option<ArtifactPiece>,
        feather: Option<ArtifactPiece>,
        sands: Option<ArtifactPiece>,
        goblet: Option<ArtifactPiece>,
        circlet: Option<ArtifactPiece>,
    ) -> Self{
        ArtifactBuilder{
            flower, 
            feather, 
            sands, 
            goblet, 
            circlet,
            rolls: std::collections::HashMap::new()
        }
    }

    pub fn build(&self) -> StatTable{
        StatTable::new()
    }
}
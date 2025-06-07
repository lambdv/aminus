use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::stat::Stat;

//type ArtifactPiece = (f32, f32, Stat);


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

    
    //role constraint   
}

impl ArtifactBuilder{
    
    pub fn new(
        flower: Option<ArtifactPiece>,
        feather: Option<ArtifactPiece>,
        sands: Option<ArtifactPiece>,
        goblet: Option<ArtifactPiece>,
        circlet: Option<ArtifactPiece>,
    ) -> Self{
        ArtifactBuilder{flower, feather, sands, goblet, circlet}
    }

    pub fn build(&self) -> StatTable{
        StatTable::new()
    }
}
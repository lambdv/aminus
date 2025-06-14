use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::stat::Stat;

/// Builder pattern for making artifact stattables
pub struct ArtifactBuilder{
    pub flower: Option<ArtifactPiece>,
    pub feather: Option<ArtifactPiece>,
    pub sands: Option<ArtifactPiece>,
    pub goblet: Option<ArtifactPiece>,
    pub circlet: Option<ArtifactPiece>,

    pub rolls: std::collections::HashMap<(Stat,RollQuality),i8>,
    pub constraints: std::collections::HashMap<Stat, i8>,
}

impl ArtifactBuilder{
    //constructors

    /// constructs a new default artifact builder
    pub fn new(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>,) -> Self {
        let constraints =std::collections::HashMap::from_iter(
            POSSIBLE_SUB_STATS.iter()
                .map(|x| (x.clone(), ([&flower, &feather, &sands, &goblet, &circlet].iter()
                    .filter(|y| y.is_some())
                    .map(|y| y.as_ref().unwrap())
                    .filter(|y| y.stat_type != *x)
                    .map(|y| max_rolls_for_given(y, &x, false))
                    .fold(0, |a,b: i8| a+b)) as i8
                ))
        );
        ArtifactBuilder{flower, feather, sands, goblet, circlet,
            rolls: std::collections::HashMap::new(),
            constraints: constraints
        }
    }

    /// constructs artifact builder for kqmc assumptions
    // https://compendium.keqingmains.com/
    /// total 40 subs
    /// 20 fixed subs distributed across 2 rolls for each substat type
    /// 20 fluid subs to distribute, max 2 rolls for each artifact piece with a main stat that is of a different stat from it
    /// 4-star artifacts have a x0.8 substat value modifer compared to 5-stars and penalty of -2 distributed substats per 4-star artifact
    /// 1 5-star and 4 4-star means the 5 star artifact will have a stat modifer of (1 * 1 + 0.8 * 4) / 5 = 0.84x rather than 1 
    pub fn kqm(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>) -> Self {
        let constraints =std::collections::HashMap::from_iter(
            POSSIBLE_SUB_STATS.iter()
                .map(|x| (x.clone(), ([&flower, &feather, &sands, &goblet, &circlet].iter()
                        .filter(|y| y.is_some())
                        .map(|y| y.as_ref().unwrap().stat_type)
                        .filter(|y| y != x)
                        .count()*2) as i8
                ))
        );
        let mut bob = ArtifactBuilder{flower, feather, sands, goblet, circlet,
            rolls: std::collections::HashMap::new(),
            constraints: constraints
        };
        POSSIBLE_SUB_STATS.iter().for_each(|x| bob.roll(*x, RollQuality::AVG, 2));
        bob
    }


    //exports
    
    pub fn build(&self) -> StatTable{
        StatTable::new()
    }

    pub fn main_stats(&self)  -> StatTable{
        StatTable::new()
    }

    pub fn sub_stats(&self)  -> StatTable{
        
        
        StatTable::new()
    }



    //update methods
    pub fn roll(&mut self, substat_value: Stat, quality: RollQuality, num: i8) {
        assert!(valid_substat_type(&substat_value));
        assert!(num <= self.substat_constraint(&substat_value));

        self.rolls.entry((substat_value.clone(), quality.clone()))
            .and_modify(|v| *v+=num)
            .or_insert(num);
    }

    pub fn unroll(substat_value: Stat, quality: RollQuality, num: i8) {

    }


    //compute values

    pub fn current_rolls(&self) -> i8 {
        self.rolls.values()
            .fold(0, |x,y|x+y)
    }

    pub fn current_rolls_for(&self, stat_type: &Stat)-> i8 {
        self.rolls.iter()
            .filter(|x| x.0.0 == *(stat_type))
            .map(|x| x.1)
            .fold(0, |x,y|x+y)
    }

    pub fn roll_distribution(&self, stat_type: Stat) -> i8 {
        0
    }

    pub fn max_rolls(&self) -> i8 {
        [&self.flower, &self.feather, &self.sands, &self.goblet, &self.circlet].iter()
            .filter(|y| y.is_some())
            .map(|x| max_rolls_for(&(x.as_ref().unwrap()), false))
            .fold(0, |x,y| x+y) as i8
    }


    pub fn substat_constraint(&self, stat_type: &Stat) -> i8 {
        self.constraints.get(stat_type).unwrap().clone()
    }

    pub fn rolls_left(&self) -> i8 {
        self.max_rolls() - self.current_rolls()
    }

    //helpers

}


///specifies an artifact
#[derive(Clone)]
pub struct ArtifactPiece {
    pub rarity: i8,
    pub level: i8,
    pub stat_type: Stat,
}


//type ArtifactPiece = (f32, f32, Stat);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
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

 



/// total max number of rolls possible for a given artifact (assuming artifact starts with max number of substats to start with. for worse case senario -1)
pub fn max_rolls_for(artifact: &ArtifactPiece, worse_case: bool) -> i8 {
    let base_substats = artifact.rarity - 1;
    let upgrades = artifact.level / 4;
    (base_substats + upgrades) as i8
}

/// max number of rolls possible given a substat on an artifact
pub fn max_rolls_for_given(artifact: &ArtifactPiece, substat_type: &Stat, worse_case: bool) -> i8 {
    if artifact.stat_type == *substat_type {
        return 0;    
    }
    let upgrades = artifact.level / 4;
    let max_rolls = if worse_case {upgrades} else {upgrades+1};
    max_rolls
}

// pub fn max_rolls_for_given_stat(artifact: &ArtifactPiece, substat_type: &Stat, worse_case: bool) -> i8 {
//     max_rolls_for(artifact,worse_case) - if substat_type == artifact.stat_type {}
// }

/// list of possible stats substats can be 
const POSSIBLE_SUB_STATS: &[Stat] = &[
    Stat::HPPercent,
    Stat::FlatHP,
    Stat::ATKPercent,
    Stat::FlatATK,
    Stat::DEFPercent,
    Stat::FlatDEF,
    Stat::ElementalMastery,
    Stat::CritRate,
    Stat::CritDMG,
    Stat::EnergyRecharge,

];

pub fn valid_substat_type(stat_type: &Stat) -> bool {
    POSSIBLE_SUB_STATS.contains(&stat_type)
}


#[cfg(test)] mod tests {
    use super::*;

    #[test] fn default_artifact_builder() {
        let bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent})
        );
        //println!("{:?}", bob.constraints)

        //builder with no artifacts with main stat of type x has constraint of 30 
        assert_eq!(bob.substat_constraint(&Stat::FlatDEF), 30);
        assert_eq!(bob.substat_constraint(&Stat::CritDMG), 30);
        assert_eq!(bob.substat_constraint(&Stat::CritRate), 30);
        //builder with 1 artifacts with main stat of type x has constraint of 24 
        assert_eq!(bob.substat_constraint(&Stat::EnergyRecharge), 24);
        assert_eq!(bob.substat_constraint(&Stat::FlatATK), 24);
        //builder with 2 artifacts with main stat of type x has constraint of 18 
        assert_eq!(bob.substat_constraint(&Stat::ATKPercent), 18);

    }

    #[test] fn kqm_artifact_builder() {
        let bob = ArtifactBuilder::kqm(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent})
        );
        //println!("{:?}", bob.constraints);

        //builder with no artifacts with main stat of type x has constraint of 30 
        assert_eq!(bob.substat_constraint(&Stat::FlatDEF), 10);
        assert_eq!(bob.substat_constraint(&Stat::CritDMG), 10);
        assert_eq!(bob.substat_constraint(&Stat::CritRate), 10);
        //builder with 1 artifacts with main stat of type x has constraint of 24 
        assert_eq!(bob.substat_constraint(&Stat::EnergyRecharge), (10-2));
        assert_eq!(bob.substat_constraint(&Stat::FlatATK), 10-2);
        //builder with 2 artifacts with main stat of type x has constraint of 18 
        assert_eq!(bob.substat_constraint(&Stat::ATKPercent), 10-2-2);
        
    }

    #[test] fn roll_updates_rolls_map() {
        let mut bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate})
        );
        assert!(bob.rolls.len() == 0);
        bob.roll(Stat::FlatATK, RollQuality::AVG, 1);
        assert!(bob.rolls.len() == 1);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::AVG)));
        assert!(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG)).is_some());
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG)).unwrap()) == 1);


        bob.roll(Stat::FlatATK, RollQuality::AVG, 1);
        assert!(bob.rolls.len() == 1);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::AVG)));
        assert!(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG)).is_some());
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG)).unwrap()) == 2);


        bob.roll(Stat::FlatATK, RollQuality::HIGH, 1);
        assert!(bob.rolls.len() == 2);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::AVG)));
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG)).unwrap()) == 2);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::HIGH)));
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::HIGH)).unwrap()) == 1);



        // assert!(bob.rolls.len() == 1);
        // assert!(bob.rolls.contains_key(&Stat::FlatATK)));
        // assert!(bob.rolls.get(&Stat::FlatATK).is_some());
        // assert!(bob.rolls.get(&Stat::FlatATK).unwrap().contains_key(&RollQuality::AVG));
        // assert!(bob.rolls.get(&Stat::FlatATK).unwrap().len()==1);
        // assert!(*(bob.rolls.get(&Stat::FlatATK).unwrap().get(&RollQuality::AVG).unwrap())==1); 


        // bob.roll(Stat::FlatATK, RollQuality::AVG, 1);
        // assert!(bob.rolls.len() == 1);
        // assert!(bob.rolls.contains_key(&Stat::FlatATK));
        // assert!(bob.rolls.get(&Stat::FlatATK).is_some());
        // assert!(bob.rolls.get(&Stat::FlatATK).unwrap().contains_key(&RollQuality::AVG));
        // assert!(bob.rolls.get(&Stat::FlatATK).unwrap().len()==1);
        // assert!(*(bob.rolls.get(&Stat::FlatATK).unwrap().get(&RollQuality::AVG).unwrap())==2); //only thing that changes is the number of rolls for said stat's roll quality

    }


    #[test] fn test_max_rolls_for_method() {
        let test_cases = vec![
            (1, 0, 0),
            (1, 4, 1),
            (2, 0, 1),
            (2, 4, 2),
            (3, 0, 2),
            (3, 4, 3),
            (3, 8, 4),
            (3, 12, 5),
            (4, 0, 3),
            (4, 4, 4),
            (4, 8, 5),
            (4, 12, 6),
            (4, 16, 7),
            (5, 0, 4),
            (5, 4, 5),
            (5, 8, 6),
            (5, 12, 7),
            (5, 16, 8),
            (5, 20, 9),
        ];

        for (rarity, level, expected) in test_cases {
            let artifact_piece = ArtifactPiece {
                rarity,
                level,
                stat_type: Stat::FlatHP, // Assuming FlatHP for simplicity
            };
            let actual = max_rolls_for(&artifact_piece, false);
            //println!("{}{}{}", rarity, level, expected);
            assert_eq!(actual, expected);
        }
    }


#[test]
fn test_artifact_builder() {
    let five_star_artifacts = ArtifactBuilder::new(
        Some(ArtifactPiece {
            rarity: 5,
            level: 20,
            stat_type: Stat::FlatHP,
        }),
        Some(ArtifactPiece {
            rarity: 5,
            level: 20,
            stat_type: Stat::FlatATK,
        }),
        Some(ArtifactPiece {
            rarity: 5,
            level: 20,
            stat_type: Stat::ATKPercent,
        }),
        Some(ArtifactPiece {
            rarity: 5,
            level: 20,
            stat_type: Stat::PyroDMGBonus,
        }),
        Some(ArtifactPiece {
            rarity: 5,
            level: 20,
            stat_type: Stat::CritRate,
        }),
    );

    assert_eq!(five_star_artifacts.current_rolls(), 0);
    assert_eq!(five_star_artifacts.max_rolls(), 45);
    assert_eq!(five_star_artifacts.rolls_left(), 45);

    let four_star_artifacts = ArtifactBuilder::new(
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::FlatHP,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::FlatATK,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::ATKPercent,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::PyroDMGBonus,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::CritRate,
        }),
    );

    assert_eq!(four_star_artifacts.current_rolls(), 0);
    assert_eq!(four_star_artifacts.max_rolls(), 35);

    let mixed_star_artifacts = ArtifactBuilder::new(
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::FlatHP,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::FlatATK,
        }),
        Some(ArtifactPiece {
            rarity: 5,
            level: 20,
            stat_type: Stat::EnergyRecharge,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::ElementalMastery,
        }),
        Some(ArtifactPiece {
            rarity: 4,
            level: 16,
            stat_type: Stat::CritRate,
        }),
    );

    assert_eq!(mixed_star_artifacts.current_rolls(), 0);
    assert_eq!(mixed_star_artifacts.max_rolls(), 37);
}

}
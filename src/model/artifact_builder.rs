use crate::factories::*;
use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::stat::Stat;
use crate::statable::ModifiableStatable;
use crate::model::artifact::*;

/// Builder pattern for making artifact stattables
pub struct ArtifactBuilder{
    // artifact pieces
    pub flower: Option<ArtifactPiece>,
    pub feather: Option<ArtifactPiece>,
    pub sands: Option<ArtifactPiece>,
    pub goblet: Option<ArtifactPiece>,
    pub circlet: Option<ArtifactPiece>,
    // substat roll data
    pub rolls: std::collections::HashMap<(Stat,RollQuality, i8),i8>, // stattype, quality, rarity : amount of rolls
    pub constraints: std::collections::HashMap<(Stat, i8), i8>, // stattype, rarity : roll limit
}

impl ArtifactBuilder{
    //constructors
    /// constructs a new default artifact builder
    pub fn new(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>,) -> Self {
        assert!(flower.as_ref().map(|x| x.stat_type == Stat::FlatHP).unwrap_or(true));
        assert!(feather.as_ref().map(|x| x.stat_type == Stat::FlatATK).unwrap_or(true));
        assert!(sands.as_ref().map(|x| POSSIBLE_SANDS_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(goblet.as_ref().map(|x| POSSIBLE_GOBLET_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(circlet.as_ref().map(|x| POSSIBLE_CIRCLE_STATS.contains(&x.stat_type)).unwrap_or(true));

        let mut constraints = std::collections::HashMap::new();
        for &stat in POSSIBLE_SUB_STATS {
            [&flower, &feather, &sands, &goblet, &circlet].iter()
                .filter_map(|piece| piece.as_ref())
                .filter(|piece| piece.stat_type != stat)
                .for_each(|piece| {
                    let key = (stat, piece.rarity);
                    let new_value = constraints.get(&key).unwrap_or(&0)+
                        max_rolls_for_given(&piece, &stat, false);
                    constraints.insert(key, new_value);
                });
        }
        ArtifactBuilder{flower, feather, sands, goblet, circlet,rolls: std::collections::HashMap::new(), constraints: constraints}
    }

    /// constructs artifact builder for kqmc assumptions
    // https://compendium.keqingmains.com/
    /// total 40 subs
    /// 20 fixed subs distributed across 2 rolls for each substat type
    /// 20 fluid subs to distribute, max 2 rolls for each artifact piece with a main stat that is of a different stat from it
    /// 4-star artifacts have a x0.8 substat value modifer compared to 5-stars and penalty of -2 distributed substats per 4-star artifact
    /// 1 5-star and 4 4-star means the 5 star artifact will have a stat modifer of (1 * 1 + 0.8 * 4) / 5 = 0.84x rather than 1 
    pub fn kqm(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>) -> Self {
        assert!(flower.as_ref().map(|x| x.stat_type == Stat::FlatHP && StatTableFactory::check_correct_level_for_rarity(x.level, x.rarity)).unwrap_or(true));
        assert!(feather.as_ref().map(|x| x.stat_type == Stat::FlatATK && StatTableFactory::check_correct_level_for_rarity(x.level, x.rarity)).unwrap_or(true));
        assert!(sands.as_ref().map(|x| POSSIBLE_SANDS_STATS.contains(&x.stat_type) && StatTableFactory::check_correct_level_for_rarity(x.level, x.rarity)).unwrap_or(true));
        assert!(goblet.as_ref().map(|x| POSSIBLE_GOBLET_STATS.contains(&x.stat_type) && StatTableFactory::check_correct_level_for_rarity(x.level, x.rarity)).unwrap_or(true));
        assert!(circlet.as_ref().map(|x| POSSIBLE_CIRCLE_STATS.contains(&x.stat_type) && StatTableFactory::check_correct_level_for_rarity(x.level, x.rarity)).unwrap_or(true));
        // calculates max number of rolls for stats colored by rarity given the artifacts provided
        let mut constraints = std::collections::HashMap::new();
        for &stat in POSSIBLE_SUB_STATS {
            [&flower, &feather, &sands, &goblet, &circlet].iter()
                .filter_map(|piece| piece.as_ref())
                .filter(|piece| piece.stat_type != stat)
                .for_each(|piece| {
                    let key = (stat, piece.rarity);
                    let new_value = constraints.get(&key).unwrap_or(&0)+2; //only 2 rolls for each substat per artifact
                    constraints.insert(key, new_value);
                });
        }
        let mut bob = ArtifactBuilder{flower, feather, sands, goblet, circlet,
            rolls: std::collections::HashMap::new(),
            constraints: constraints
        };
        //2 fixed rolls for each substat
        //TODO: currently rolling 5 star rather than based on the artifacts given rarity
        POSSIBLE_SUB_STATS.iter()
            .for_each(|&x|bob.roll(x, RollQuality::AVG, 5, 2));
        bob
    }

    //exports

    ///compiles main stats and sub stat rolls into a single stattable
    /// this statable represents all the stats built by the builder
    pub fn build(&self) -> StatTable {
        let mut sum = self.main_stats();
        sum.add_table(Box::new(self.sub_stats().iter()));
        sum
    }

    /// compiles main stats into a stattable
    pub fn main_stats(&self)  -> StatTable{
        let mut res = StatTable::new();
        self.main_pieces().iter().for_each(|spec| {
            let value = StatTableFactory::get_main_stat_value(spec.rarity, spec.level, &spec.stat_type).unwrap(); //TODO: handle this maybe
            res.add(&spec.stat_type, value);
        });
        res 
    }

    /// compiles sub stats based on rolls allocated
    pub fn sub_stats(&self)  -> StatTable{
        let mut res = StatTable::new();
        for ((stat, quality, rarity), num) in self.rolls.iter() {
            let value = StatTableFactory::get_sub_stat_value(*rarity, *stat).unwrap();
            let value = value * quality.multiplier();
            res.add(stat, value);
        }
        res
    }


    // getters
    /// returns a vector of main pieces
    pub fn main_pieces(&self) -> Vec<&ArtifactPiece> {
        [&self.flower, &self.feather, &self.sands, &self.goblet, &self.circlet].iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .collect::<Vec<_>>()
    }

    //update methods

    /// rolls a substat
    pub fn roll(&mut self, substat_value: Stat, quality: RollQuality, rarity: i8, num: i8) {
        assert!(is_valid_substat_type(&substat_value));
        assert!(num <= self.substat_constraint(&substat_value, rarity));

        self.rolls.entry((substat_value.clone(), quality.clone(), rarity))
            .and_modify(|v| *v+=num)
            .or_insert(num);
    }

    /// unrolls a substat
    pub fn unroll(&mut self, substat_value: Stat, quality: RollQuality, rarity: i8, num: i8) {
        assert!(is_valid_substat_type(&substat_value));
        
        let key = (substat_value.clone(), quality.clone(), rarity);
        if let Some(current_rolls) = self.rolls.get_mut(&key) {
            if *current_rolls >= num {
                *current_rolls -= num;
                if *current_rolls == 0 {
                    self.rolls.remove(&key);
                }
            }
        }
    }

    //compute values

    /// returns the total number of rolls
    pub fn current_rolls(&self) -> i8 {
        self.rolls.values()
            .fold(0, |x,y|x+y)
    }
    /// returns the total number of rolls for a given stat
    pub fn current_rolls_for(&self, stat_type: &Stat)-> i8 {
        self.rolls.iter()
            .filter(|x| x.0.0 == *(stat_type))
            .map(|x| x.1)
            .fold(0, |x,y|x+y)
    }

    /// returns the total number of rolls for a given stat, quality, and rarity
    pub fn current_rolls_for_given(&self, stat_type: &Stat, quality: RollQuality, rarity: i8)-> i8 {
        self.rolls.iter()
            .filter(|x| x.0.0 == *(stat_type) && x.0.1 == quality && x.0.2 == rarity)
            .map(|x| x.1)
            .fold(0, |x,y|x+y)
    }



    /// returns the total number of rolls possible
    pub fn max_rolls(&self) -> i8 {
        [&self.flower, &self.feather, &self.sands, &self.goblet, &self.circlet].iter()
            .filter(|y| y.is_some())
            .map(|x| max_rolls_for(&(x.as_ref().unwrap()), false))
            .fold(0, |x,y| x+y) as i8
    }

    /// returns the total number of rolls possible for a given stat
    pub fn substat_constraint(&self, stat_type: &Stat, rarity: i8) -> i8 {
        self.constraints.get(&(stat_type.clone(), rarity)).unwrap().clone()
    }

    /// returns the total number of rolls left
    pub fn rolls_left(&self) -> i8 {
        self.max_rolls() - self.current_rolls()
    }

}


///specifies an artifact
#[derive(Clone, Debug)]
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
    if artifact.stat_type == *substat_type { return 0; }
    let upgrades = artifact.level / 4;
    let max_rolls = if worse_case {upgrades} else {upgrades + 1};
    max_rolls
}

pub fn max_rolls_for_given_stat(artifact: &ArtifactPiece, substat_type: &Stat, worse_case: bool) -> i8 {
    max_rolls_for(artifact,worse_case) - if substat_type == &artifact.stat_type {1} else {0}
}

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

pub fn is_valid_substat_type(stat_type: &Stat) -> bool {
    POSSIBLE_SUB_STATS.contains(&stat_type)
}


#[cfg(test)] mod tests {
    use super::*;

    #[test] fn default_artifact_builder() {
        let mut bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent})
        );
        //println!("{:?}", bob.constraints)

        //builder with no artifacts with main stat of type x has constraint of 30 
        assert_eq!(bob.substat_constraint(&Stat::FlatDEF, 5), 30);
        assert_eq!(bob.substat_constraint(&Stat::CritDMG, 5), 30);
        assert_eq!(bob.substat_constraint(&Stat::CritRate, 5), 30);
        //builder with 1 artifacts with main stat of type x has constraint of 24 
        assert_eq!(bob.substat_constraint(&Stat::EnergyRecharge, 5), 24);
        assert_eq!(bob.substat_constraint(&Stat::FlatATK, 5), 24);
        //builder with 2 artifacts with main stat of type x has constraint of 18 
        assert_eq!(bob.substat_constraint(&Stat::ATKPercent, 5), 18);


        //test main stats
        let expected = StatTable::of(&[
            (Stat::FlatHP, 4780.0),
            (Stat::FlatATK, 311.0),
            (Stat::EnergyRecharge,0.518),
            (Stat::ATKPercent, 0.466),
            (Stat::ATKPercent, 0.466),
        ]); 

        let actual = bob.main_stats();
        assert_eq!(expected, actual);

        //test sub stats
        let expected = StatTable::new();
        let actual = bob.sub_stats();
        assert_eq!(expected, actual);

        //lets now roll
        bob.roll(Stat::CritRate, RollQuality::AVG, 5, 1);
        let expected = StatTableFactory::get_sub_stat_value(5, Stat::CritRate).unwrap() * RollQuality::AVG.multiplier();
        let actual = bob.sub_stats().get(&Stat::CritRate);
        assert_eq!(expected, actual);
        bob.roll(Stat::CritRate, RollQuality::MAX, 5, 1);
        let expected = expected + StatTableFactory::get_sub_stat_value(5, Stat::CritRate).unwrap() * RollQuality::MAX.multiplier();
        let actual = bob.sub_stats().get(&Stat::CritRate);
        assert_eq!(expected, actual);

        
        //test build
        let expected = StatTable::of(&[
            (Stat::FlatHP, 4780.0),
            (Stat::FlatATK, 311.0),
            (Stat::EnergyRecharge,0.518),
            (Stat::ATKPercent, 0.466),
            (Stat::ATKPercent, 0.466),
            (Stat::CritRate, expected),
        ]); 
        let actual = bob.build();
        assert_eq!(expected, actual);

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
        assert_eq!(bob.substat_constraint(&Stat::FlatDEF, 5), 10);
        assert_eq!(bob.substat_constraint(&Stat::CritDMG, 5), 10);
        assert_eq!(bob.substat_constraint(&Stat::CritRate, 5), 10);
        //builder with 1 artifacts with main stat of type x has constraint of 24 
        assert_eq!(bob.substat_constraint(&Stat::EnergyRecharge, 5), (10-2));
        assert_eq!(bob.substat_constraint(&Stat::FlatATK, 5), 10-2);
        //builder with 2 artifacts with main stat of type x has constraint of 18 
        assert_eq!(bob.substat_constraint(&Stat::ATKPercent, 5), 10-2-2);
        
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
        bob.roll(Stat::FlatATK, RollQuality::AVG, 5, 1);
        assert!(bob.rolls.len() == 1);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::AVG, 5)));
        assert!(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG, 5)).is_some());
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG, 5)).unwrap()) == 1);

        bob.roll(Stat::FlatATK, RollQuality::AVG, 5, 1);
        assert!(bob.rolls.len() == 1);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::AVG, 5)));
        assert!(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG, 5)).is_some());
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG, 5)).unwrap()) == 2);

        bob.roll(Stat::FlatATK, RollQuality::HIGH, 5, 1);
        assert!(bob.rolls.len() == 2);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::AVG, 5)));
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::AVG, 5)).unwrap()) == 2);
        assert!(bob.rolls.contains_key(&(Stat::FlatATK, RollQuality::HIGH, 5)));
        assert!(*(bob.rolls.get(&(Stat::FlatATK, RollQuality::HIGH, 5)).unwrap()) == 1);



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


    #[test] fn test_artifact_builder() {
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

    #[test] fn main_stats_are_correct() {
        let bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent})
        );

        
        let expected = StatTable::of(&[
            (Stat::FlatHP, 4780.0),
            (Stat::FlatATK, 311.0),
            (Stat::EnergyRecharge,0.518),
            (Stat::ATKPercent, 0.466),
            (Stat::ATKPercent, 0.466),
        ]); 
        let actual = bob.main_stats();

        assert_eq!(expected, actual);
    }

    #[test] #[should_panic] fn invalid_main_stat_values_are_caught() {
        let _bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatDEF}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge}), //among us
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent})
        );
    }

    #[test] fn load_artifact_sub_stat_resource_as_json() {
        // Test loading artifact substat values from JSON
        let flat_hp = StatTableFactory::get_sub_stat_value(5, Stat::FlatHP);
        assert!(flat_hp.is_ok());
        assert!((flat_hp.unwrap() - 298.75).abs() < 0.001);

        let flat_atk = StatTableFactory::get_sub_stat_value(5, Stat::FlatATK);
        assert!(flat_atk.is_ok());
        assert!((flat_atk.unwrap() - 19.45).abs() < 0.001);

        let flat_def = StatTableFactory::get_sub_stat_value(5, Stat::FlatDEF);
        assert!(flat_def.is_ok());
        assert!((flat_def.unwrap() - 23.15).abs() < 0.001);

        let hp_percent = StatTableFactory::get_sub_stat_value(5, Stat::HPPercent);
        assert!(hp_percent.is_ok());
        assert!((hp_percent.unwrap() - 0.0583).abs() < 0.001);

        let atk_percent = StatTableFactory::get_sub_stat_value(5, Stat::ATKPercent);
        assert!(atk_percent.is_ok());
        assert!((atk_percent.unwrap() - 0.0583).abs() < 0.001);

        let def_percent = StatTableFactory::get_sub_stat_value(5, Stat::DEFPercent);
        assert!(def_percent.is_ok());
        assert!((def_percent.unwrap() - 0.0729).abs() < 0.001);

        let elemental_mastery = StatTableFactory::get_sub_stat_value(5, Stat::ElementalMastery);
        assert!(elemental_mastery.is_ok());
        assert!((elemental_mastery.unwrap() - 23.31).abs() < 0.001);

        let energy_recharge = StatTableFactory::get_sub_stat_value(5, Stat::EnergyRecharge);
        assert!(energy_recharge.is_ok());
        assert!((energy_recharge.unwrap() - 0.0648).abs() < 0.001);

        let crit_rate = StatTableFactory::get_sub_stat_value(5, Stat::CritRate);
        assert!(crit_rate.is_ok());
        assert!((crit_rate.unwrap() - 0.0389).abs() < 0.001);

        let crit_dmg = StatTableFactory::get_sub_stat_value(5, Stat::CritDMG);
        assert!(crit_dmg.is_ok());
        assert!((crit_dmg.unwrap() - 0.0777).abs() < 0.001);
    }

    #[test] fn load_artifact_main_stat_resource_as_json() {
        // Test loading artifact main stat values from JSON
        let flat_hp_level_0 = StatTableFactory::get_main_stat_value(5, 0, &Stat::FlatHP);
        assert!(flat_hp_level_0.is_ok());
        assert!((flat_hp_level_0.unwrap() - 717.0).abs() < 0.01);

        let flat_hp_level_20 = StatTableFactory::get_main_stat_value(5, 20, &Stat::FlatHP);
        assert!(flat_hp_level_20.is_ok());
        assert!((flat_hp_level_20.unwrap() - 4780.0).abs() < 0.01);

        let crit_dmg_level_20 = StatTableFactory::get_main_stat_value(5, 20, &Stat::CritDMG);
        assert!(crit_dmg_level_20.is_ok());
        assert!((crit_dmg_level_20.unwrap() - 0.622).abs() < 0.01);
    }

    #[test] fn artifact_utils_class_get_main_and_substat_base_values() {
        // Test utility functions for getting main and substat values
        assert!((StatTableFactory::get_main_stat_value(5, 0, &Stat::FlatHP).unwrap() - 717.0).abs() < 0.01);
        assert!((StatTableFactory::get_main_stat_value(5, 20, &Stat::FlatHP).unwrap() - 4780.0).abs() < 0.01);
        assert!((StatTableFactory::get_main_stat_value(5, 20, &Stat::CritDMG).unwrap() - 0.622).abs() < 0.01);
        
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::FlatHP).unwrap() - 298.75).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::FlatATK).unwrap() - 19.45).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::FlatDEF).unwrap() - 23.15).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::HPPercent).unwrap() - 0.0583).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::ATKPercent).unwrap() - 0.0583).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::DEFPercent).unwrap() - 0.0729).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::ElementalMastery).unwrap() - 23.31).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::EnergyRecharge).unwrap() - 0.0648).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::CritRate).unwrap() - 0.0389).abs() < 0.01);
        assert!((StatTableFactory::get_sub_stat_value(5, Stat::CritDMG).unwrap() - 0.0777).abs() < 0.01);
    }

    #[test] fn kqm_artifact_builder_detailed_test() {
        let bob = ArtifactBuilder::kqm(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate})
        );

        assert_eq!(bob.current_rolls(), 20); // 2 rolls for each of 10 substats
        assert_eq!(bob.max_rolls(), 40); // 20 fixed + 20 fluid in KQM
        assert_eq!(bob.rolls_left(), 20);

        // Test substat constraints
        assert_eq!(bob.substat_constraint(&Stat::HPPercent, 5), 10);
        assert_eq!(bob.substat_constraint(&Stat::ATKPercent, 5), 8); // -2 because ATKPercent is main stat
        assert_eq!(bob.substat_constraint(&Stat::CritRate, 5), 8); // -2 because CritRate is main stat
    }

    #[test] fn artifact_builder_rolling_substats() {
        let mut bob = ArtifactBuilder::kqm(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate})
        );

        // Initially HPPercent should have 10 rolls left
        let initial_hp_constraint = bob.substat_constraint(&Stat::HPPercent, 5);
        assert_eq!(initial_hp_constraint, 10);

        let initial_rolls = bob.current_rolls();
        assert_eq!(initial_rolls, 20);

        // Roll HPPercent 10 times
        for i in 1..=10 {
            bob.roll(Stat::HPPercent, RollQuality::AVG, 5, 1);
            assert_eq!(bob.current_rolls_for(&Stat::HPPercent), 2 + i); // 2 initial + i additional
        }

        // Should not be able to roll anymore HPPercent
        let hp_rolls_after = bob.current_rolls_for(&Stat::HPPercent);
        assert_eq!(hp_rolls_after, 12);

        // Test that substats are calculated correctly
        let substats = bob.sub_stats();
        let hp_percent_value = substats.get(&Stat::HPPercent);
        let expected_hp_percent: f32 = StatTableFactory::get_sub_stat_value(5, Stat::HPPercent).unwrap() 
            * RollQuality::AVG.multiplier() * 12.0;
        assert!((hp_percent_value - expected_hp_percent).abs() < 0.1);
    }

    #[test] fn artifact_builder_rolling_substats_for_4star() {
        let bob = ArtifactBuilder::kqm(
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::CritRate})
        );

        // 4-star artifacts should have less max rolls
        assert_eq!(bob.max_rolls(), 35); // Should be less than 5-star
    }

    #[test] fn test_rolling_constraints() {
        let mut bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate})
        );

        // Test that we can roll up to the constraint
        let constraint = bob.substat_constraint(&Stat::HPPercent, 5);
        for _ in 0..constraint {
            bob.roll(Stat::HPPercent, RollQuality::AVG, 5, 1);
        }

        // Test that we've reached the constraint
        assert_eq!(bob.current_rolls_for(&Stat::HPPercent), constraint);
    }

    #[test] fn test_roll_quality_multipliers() {
        assert!((RollQuality::MAX.multiplier() - 1.0).abs() < 0.001);
        assert!((RollQuality::HIGH.multiplier() - 0.9).abs() < 0.001);
        assert!((RollQuality::MID.multiplier() - 0.8).abs() < 0.001);
        assert!((RollQuality::LOW.multiplier() - 0.7).abs() < 0.001);
        assert!((RollQuality::AVG.multiplier() - 0.85).abs() < 0.001); // (1.0+0.9+0.8+0.7)/4.0
    }

    #[test] fn test_substat_value_calculation() {
        let mut bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate})
        );

        // Roll some substats
        bob.roll(Stat::CritDMG, RollQuality::MAX, 5, 3);
        bob.roll(Stat::CritDMG, RollQuality::HIGH, 5, 2);

        let substats = bob.sub_stats();
        let crit_dmg_value = substats.get(&Stat::CritDMG);
        
        let base_value = StatTableFactory::get_sub_stat_value(5, Stat::CritDMG).unwrap();
        let expected = base_value * (RollQuality::MAX.multiplier() * 3.0 + RollQuality::HIGH.multiplier() * 2.0);
        
        assert!((crit_dmg_value - expected).abs() < 0.001);
    }

    #[test] fn test_build_combines_main_and_sub_stats() {
        let mut bob = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate})
        );

        // Add some substat rolls
        bob.roll(Stat::CritDMG, RollQuality::AVG, 5, 5);
        bob.roll(Stat::ATKPercent, RollQuality::AVG, 5, 3);

        let combined = bob.build();
        let main_only = bob.main_stats();  
        let sub_only = bob.sub_stats();

        // Check that build combines both main and sub stats
        assert!(combined.get(&Stat::FlatHP) > 0.0); // From main stats
        assert!(combined.get(&Stat::CritDMG) > 0.0); // From sub stats
        
        // Check that ATKPercent is combined (main + sub)
        let combined_atk_percent = combined.get(&Stat::ATKPercent);
        let main_atk_percent = main_only.get(&Stat::ATKPercent);
        let sub_atk_percent = sub_only.get(&Stat::ATKPercent);
        
        assert!((combined_atk_percent - (main_atk_percent + sub_atk_percent)).abs() < 0.001);
    }

    #[test] fn test_valid_substat_types() {
        assert!(is_valid_substat_type(&Stat::HPPercent));
        assert!(is_valid_substat_type(&Stat::FlatHP));
        assert!(is_valid_substat_type(&Stat::ATKPercent));
        assert!(is_valid_substat_type(&Stat::FlatATK));
        assert!(is_valid_substat_type(&Stat::DEFPercent));
        assert!(is_valid_substat_type(&Stat::FlatDEF));
        assert!(is_valid_substat_type(&Stat::ElementalMastery));
        assert!(is_valid_substat_type(&Stat::CritRate));
        assert!(is_valid_substat_type(&Stat::CritDMG));
        assert!(is_valid_substat_type(&Stat::EnergyRecharge));

        // These should not be valid substats
        assert!(!is_valid_substat_type(&Stat::PyroDMGBonus));
        assert!(!is_valid_substat_type(&Stat::BaseHP));
        assert!(!is_valid_substat_type(&Stat::BaseATK));
    }
}

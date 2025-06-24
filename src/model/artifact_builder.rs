use crate::functions::stat_factory::StatFactory;
use crate::model::stattable::StatTable;
use crate::model::statable::Statable;
use crate::stat::Stat;
use crate::statable::ModifiableStatable;
use crate::model::artifact::*;
use crate::assert_aprx;


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
    roll_limit: Option<i8>,
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
        ArtifactBuilder{flower, feather, sands, goblet, circlet,rolls: std::collections::HashMap::new(), constraints: constraints, roll_limit: None}
    }

    /// constructs artifact builder for kqmc assumptions
    // https://compendium.keqingmains.com/
    /// total 40 subs
    /// 20 fixed subs distributed across 2 rolls for each substat type
    /// 20 fluid subs to distribute, max 2 rolls for each artifact piece with a main stat that is of a different stat from it
    /// 4-star artifacts have a x0.8 substat value modifer compared to 5-stars and penalty of -2 distributed substats per 4-star artifact
    /// 1 5-star and 4 4-star means the 5 star artifact will have a stat modifer of (1 * 1 + 0.8 * 4) / 5 = 0.84x rather than 1 
    pub fn kqm(flower: Option<ArtifactPiece>, feather: Option<ArtifactPiece>, sands: Option<ArtifactPiece>, goblet: Option<ArtifactPiece>, circlet: Option<ArtifactPiece>) -> Self {
        //invariant checks
        assert!(flower.as_ref().map(|x| x.stat_type == Stat::FlatHP).unwrap_or(true));
        assert!(feather.as_ref().map(|x| x.stat_type == Stat::FlatATK).unwrap_or(true));
        assert!(sands.as_ref().map(|x| POSSIBLE_SANDS_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(goblet.as_ref().map(|x| POSSIBLE_GOBLET_STATS.contains(&x.stat_type)).unwrap_or(true));
        assert!(circlet.as_ref().map(|x| POSSIBLE_CIRCLE_STATS.contains(&x.stat_type)).unwrap_or(true));
        [flower.as_ref(), feather.as_ref(), sands.as_ref(), goblet.as_ref(), circlet.as_ref()].iter()
            .filter_map(|x| x.as_ref())
            .for_each(|x| {
                assert!(StatFactory::check_correct_level_for_rarity(x.level, x.rarity));
                assert!(x.rarity > 3);
                assert!((x.level == 20 && x.rarity == 5) || (x.level == 16 && x.rarity == 4));
            });

        // Determine the rarity to use for rolling based on the artifacts provided
        // If all artifacts are the same rarity, use that rarity. Otherwise, use the highest rarity.
        let rarities: std::collections::HashSet<i8> = [&flower, &feather, &sands, &goblet, &circlet].iter()
            .filter_map(|x| x.as_ref())
            .map(|a| a.rarity)
            .collect();
        let roll_rarity = if rarities.len() == 1 {
            *rarities.iter().next().unwrap()
        } else {
            *rarities.iter().max().unwrap()
        };

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

        let base = [&flower, &feather, &sands, &goblet, &circlet].iter()
            .filter_map(|piece| piece.as_ref())
            .map(|x| max_rolls_for(&x, false))
            .fold(0, |x,y| x+y) as i8;

        println!("base: {}", base);

        let penalty = [&flower, &feather, &sands, &goblet, &circlet].iter()
            .filter_map(|piece| piece.as_ref())
            .count() as i8;

        let roll_limit = base - penalty;

        let mut bob = ArtifactBuilder{flower, feather, sands, goblet, circlet,
            rolls: std::collections::HashMap::new(),
            constraints: constraints,
            roll_limit: Some(roll_limit)
        };

        POSSIBLE_SUB_STATS.iter()
            .for_each(|&stat| {
                bob.roll(stat, RollQuality::AVG, roll_rarity, 2);
            });
        bob
    }


    /// constructs artifact builder for kqmc assumptions with all 5 star artifacts
    pub fn kqm_all_5_star(sands_main: Stat, goblet_main: Stat, circlet_main: Stat) -> Self {
       ArtifactBuilder::kqm(
        Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
        Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
        Some(ArtifactPiece{rarity:5, level:20, stat_type: sands_main}),
        Some(ArtifactPiece{rarity:5, level:20, stat_type: goblet_main}),
        Some(ArtifactPiece{rarity:5, level:20, stat_type: circlet_main}),
        )
    }

    /// constructs artifact builder for kqmc assumptions with all 4 star artifacts
    pub fn kqm_all_4_star(sands_main: Stat, goblet_main: Stat, circlet_main: Stat) -> Self {
        let mut bob = ArtifactBuilder::kqm(
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: sands_main}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: goblet_main}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: circlet_main}),
        );

        POSSIBLE_SUB_STATS.iter()
            .for_each(|&stat| {
                bob.unroll(stat, RollQuality::AVG, 5, 2);
                bob.roll(stat, RollQuality::AVG, 4, 2);
            });
        bob
    }

    /// constructs artifact builder for kqmc assumptions with all 4 star artifacts and one 5 star artifact
    /// index 0 is sands, 1 is goblet, 2 is circlet
    pub fn kqm_all_4_star_with_5_star(sands_main: Stat, goblet_main: Stat, circlet_main: Stat, five_star_index: usize) -> Self {
        assert!(five_star_index < 3);
        
        let sands_piece = ArtifactPiece{rarity:4, level:16, stat_type: sands_main};
        let goblet_piece = ArtifactPiece{rarity:4, level:16, stat_type: goblet_main};
        let circlet_piece = ArtifactPiece{rarity:4, level:16, stat_type: circlet_main};

        match five_star_index {
            0 => {let sands_piece = ArtifactPiece{rarity:5, level:20, stat_type: sands_main};},
            1 => {let goblet_piece = ArtifactPiece{rarity:5, level:20, stat_type: goblet_main};},
            2 => {let circlet_piece = ArtifactPiece{rarity:5, level:20, stat_type: circlet_main};},
            _ => panic!("Invalid five star index"),
        };

        let mut bob = ArtifactBuilder::kqm(
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatATK}),
            Some(sands_piece),
            Some(goblet_piece),
            Some(circlet_piece)
        );
        POSSIBLE_SUB_STATS.iter()
        .for_each(|&stat| {
            bob.unroll(stat, RollQuality::AVG, 5, 2);
            bob.roll(stat, RollQuality::AVG, 4, 2);
        });
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
            let value = StatFactory::get_main_stat_value(spec.rarity, spec.level, &spec.stat_type).unwrap(); //TODO: handle this maybe
            res.add(&spec.stat_type, value);
        });
        res 
    }

    /// compiles sub stats based on rolls allocated
    pub fn sub_stats(&self)  -> StatTable{
        let mut res = StatTable::new();
        for ((stat, quality, rarity), num) in self.rolls.iter() {
            let value = StatFactory::get_sub_stat_value(*rarity, *stat).unwrap();
            let value = value * quality.multiplier() * (*num as f32);
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
        self.rolls.values().fold(0, |x,y|x+y)
    }
    /// returns the total number of rolls for a given stat
    // pub fn current_rolls_for(&self, stat_type: &Stat)-> i8 {
    //     self.rolls.iter()
    //         .filter(|x| x.0.0 == *(stat_type))
    //         .map(|x| x.1)
    //         .fold(0, |x,y|x+y)
    // }

    /// returns the total number of rolls for a stat, given quality and rarity
    pub fn current_rolls_for_given(&self, stat_type: &Stat, quality: RollQuality, rarity: i8)-> i8 {
        self.rolls.iter()
            .filter(|x| x.0.0 == *(stat_type) && x.0.1 == quality && x.0.2 == rarity)
            .map(|x| x.1)
            .fold(0, |x,y|x+y)
    }

    /// returns the total number of rolls possible
    pub fn max_rolls(&self) -> i8 {
        if self.roll_limit.is_some() {
            return self.roll_limit.unwrap();
        }

        self.artifacts_iter()
            .map(|x| max_rolls_for(&x, false))
            .fold(0, |x,y| x+y) as i8
    }

    /// returns the total number of rolls possible for a given stat
    pub fn substat_constraint(&self, stat_type: &Stat, rarity: i8) -> i8 {
        self.constraints.get(&(stat_type.clone(), rarity)).unwrap_or(&0).clone()
    }

    /// returns the total number of rolls left
    pub fn rolls_left(&self) -> i8 {
        self.max_rolls() - self.current_rolls()
    }

    ///helpers
    
    fn artifacts_iter(&self) -> std::vec::IntoIter<&ArtifactPiece> {
        [&self.flower, &self.feather, &self.sands, &self.goblet, &self.circlet].iter()
            .filter(|y| y.is_some())
            .map(|x| x.as_ref().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
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
    base_substats + upgrades
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
pub const POSSIBLE_SUB_STATS: &[Stat] = &[
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
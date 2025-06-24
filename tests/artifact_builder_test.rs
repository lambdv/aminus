use aminus::model::artifact_builder::*;
use aminus::factories::*;
use aminus::model::stattable::StatTable;
use aminus::model::statable::Statable;
use aminus::stat::Stat;
use aminus::assert_aprx;


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
    let expected = StatFactory::get_sub_stat_value(5, Stat::CritRate).unwrap() * RollQuality::AVG.multiplier();
    let actual = bob.sub_stats().get(&Stat::CritRate);
    assert_eq!(expected, actual);
    bob.roll(Stat::CritRate, RollQuality::MAX, 5, 1);
    let expected = expected + StatFactory::get_sub_stat_value(5, Stat::CritRate).unwrap() * RollQuality::MAX.multiplier();
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

#[test] fn default_artifact_builder_4_star_artifacts() {
    let mut bob = ArtifactBuilder::new(
        Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatHP}),
        Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::FlatATK}),
        Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::EnergyRecharge}),
        Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::ATKPercent}),
        Some(ArtifactPiece{rarity:4, level:16, stat_type: Stat::ATKPercent})
    );
    //println!("{:?}", bob.constraints)

    assert_eq!(bob.max_rolls(), 35);
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
    let flat_hp = StatFactory::get_sub_stat_value(5, Stat::FlatHP);
    assert!(flat_hp.is_ok());
    assert!((flat_hp.unwrap() - 298.75).abs() < 0.001);

    let flat_atk = StatFactory::get_sub_stat_value(5, Stat::FlatATK);
    assert!(flat_atk.is_ok());
    assert!((flat_atk.unwrap() - 19.45).abs() < 0.001);

    let flat_def = StatFactory::get_sub_stat_value(5, Stat::FlatDEF);
    assert!(flat_def.is_ok());
    assert!((flat_def.unwrap() - 23.15).abs() < 0.001);

    let hp_percent = StatFactory::get_sub_stat_value(5, Stat::HPPercent);
    assert!(hp_percent.is_ok());
    assert!((hp_percent.unwrap() - 0.0583).abs() < 0.001);

    let atk_percent = StatFactory::get_sub_stat_value(5, Stat::ATKPercent);
    assert!(atk_percent.is_ok());
    assert!((atk_percent.unwrap() - 0.0583).abs() < 0.001);

    let def_percent = StatFactory::get_sub_stat_value(5, Stat::DEFPercent);
    assert!(def_percent.is_ok());
    assert!((def_percent.unwrap() - 0.0729).abs() < 0.001);

    let elemental_mastery = StatFactory::get_sub_stat_value(5, Stat::ElementalMastery);
    assert!(elemental_mastery.is_ok());
    assert!((elemental_mastery.unwrap() - 23.31).abs() < 0.001);

    let energy_recharge = StatFactory::get_sub_stat_value(5, Stat::EnergyRecharge);
    assert!(energy_recharge.is_ok());
    assert!((energy_recharge.unwrap() - 0.0648).abs() < 0.001);

    let crit_rate = StatFactory::get_sub_stat_value(5, Stat::CritRate);
    assert!(crit_rate.is_ok());
    assert!((crit_rate.unwrap() - 0.0389).abs() < 0.001);

    let crit_dmg = StatFactory::get_sub_stat_value(5, Stat::CritDMG);
    assert!(crit_dmg.is_ok());
    assert!((crit_dmg.unwrap() - 0.0777).abs() < 0.001);
}

#[test] fn load_artifact_main_stat_resource_as_json() {
    // Test loading artifact main stat values from JSON
    let flat_hp_level_0 = StatFactory::get_main_stat_value(5, 0, &Stat::FlatHP);
    assert!(flat_hp_level_0.is_ok());
    assert!((flat_hp_level_0.unwrap() - 717.0).abs() < 0.01);

    let flat_hp_level_20 = StatFactory::get_main_stat_value(5, 20, &Stat::FlatHP);
    assert!(flat_hp_level_20.is_ok());
    assert!((flat_hp_level_20.unwrap() - 4780.0).abs() < 0.01);

    let crit_dmg_level_20 = StatFactory::get_main_stat_value(5, 20, &Stat::CritDMG);
    assert!(crit_dmg_level_20.is_ok());
    assert!((crit_dmg_level_20.unwrap() - 0.622).abs() < 0.01);
}

#[test] fn artifact_utils_class_get_main_and_substat_base_values() {
    // Test utility functions for getting main and substat values
    assert!((StatFactory::get_main_stat_value(5, 0, &Stat::FlatHP).unwrap() - 717.0).abs() < 0.01);
    assert!((StatFactory::get_main_stat_value(5, 20, &Stat::FlatHP).unwrap() - 4780.0).abs() < 0.01);
    assert!((StatFactory::get_main_stat_value(5, 20, &Stat::CritDMG).unwrap() - 0.622).abs() < 0.01);
    
    assert!((StatFactory::get_sub_stat_value(5, Stat::FlatHP).unwrap() - 298.75).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::FlatATK).unwrap() - 19.45).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::FlatDEF).unwrap() - 23.15).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::HPPercent).unwrap() - 0.0583).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::ATKPercent).unwrap() - 0.0583).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::DEFPercent).unwrap() - 0.0729).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::ElementalMastery).unwrap() - 23.31).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::EnergyRecharge).unwrap() - 0.0648).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::CritRate).unwrap() - 0.0389).abs() < 0.01);
    assert!((StatFactory::get_sub_stat_value(5, Stat::CritDMG).unwrap() - 0.0777).abs() < 0.01);
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

    assert_eq!(bob.max_rolls(), 40);
    assert_eq!(bob.rolls_left(), 20);

    // Initially HPPercent should have 10 rolls left
    let initial_hp_constraint = bob.substat_constraint(&Stat::HPPercent, 5);
    assert_eq!(initial_hp_constraint, 10);

    let initial_rolls = bob.current_rolls();
    assert_eq!(initial_rolls, 20);

    // Roll HPPercent 10 times
    for i in 1..=10 {
        bob.roll(Stat::HPPercent, RollQuality::AVG, 5, 1);
        assert_eq!(bob.current_rolls_for_given(&Stat::HPPercent, RollQuality::AVG, 5), 2 + i); // 2 initial + i additional
    }

    // Should not be able to roll anymore HPPercent
    let hp_rolls_after = bob.current_rolls_for_given(&Stat::HPPercent, RollQuality::AVG, 5);
    assert_eq!(hp_rolls_after, 12);

    // Test that substats are calculated correctly
    let substats = bob.sub_stats();
    let hp_percent_value = substats.get(&Stat::HPPercent);
    let expected_hp_percent: f32 = StatFactory::get_sub_stat_value(5, Stat::HPPercent).unwrap() 
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

    assert_eq!(bob.max_rolls(), 30);
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
    assert_eq!(bob.current_rolls_for_given(&Stat::HPPercent, RollQuality::AVG, 5), constraint);
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

    assert_eq!(bob.current_rolls_for_given(&Stat::CritDMG, RollQuality::MAX, 5), 3);
    assert_eq!(bob.current_rolls_for_given(&Stat::CritDMG, RollQuality::HIGH, 5), 2);
    assert_eq!(bob.current_rolls(), 5);
    assert_eq!(bob.rolls_left(), 45-5);
    assert_eq!(bob.current_rolls_for_given(&Stat::CritDMG, RollQuality::MAX, 5), 3);
    assert_eq!(bob.current_rolls_for_given(&Stat::CritDMG, RollQuality::HIGH, 5), 2);



    let substats = bob.sub_stats();
    let crit_dmg_value = substats.get(&Stat::CritDMG);
    
    let base_value = StatFactory::get_sub_stat_value(5, Stat::CritDMG).unwrap();
    let expected = (base_value*(RollQuality::MAX.multiplier()*3.0)) + (base_value * (RollQuality::HIGH.multiplier()*2.0));
    
    assert_aprx!(crit_dmg_value, expected, 0.001);
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

    assert_aprx!(combined_atk_percent, (main_atk_percent + sub_atk_percent), 0.1);
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
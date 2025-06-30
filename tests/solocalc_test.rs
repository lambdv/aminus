use aminus::factories::StatFactory;
use aminus::functions::formulas::formulas::*;
use aminus::model::stattable::*;
use aminus::model::statable::*;
use aminus::model::rotation::Rotation;
// use aminus::formulas::formulas::*;
use aminus::model::stat::*;
use aminus::functions::dmg_function::*;


#[test] fn primative_character_damage_calculation() {
    let mut diluc = StatTable::of(&[
        (Stat::BaseATK, 334.85),
        (Stat::CritRate, 0.192 + 0.05),
        (Stat::CritDMG, 0.5),
        (Stat::EnergyRecharge, 1.0),
    ]);
    let weapon = StatTable::of(&[
        (Stat::BaseATK, 510.0),
        (Stat::ElementalMastery, 165.0),
    ]);
    let artifacts = StatTable::of(&[
        //mainstats
        (Stat::FlatHP, 4780.0),
        (Stat::FlatATK, 311.0),
        (Stat::ATKPercent, 0.466),
        (Stat::PyroDMGBonus, 0.466),
        (Stat::CritRate, 0.311),
        //substats
        (Stat::ATKPercent, 0.0992),
        (Stat::FlatATK, 33.08),
        (Stat::ElementalMastery, 39.6400),
        (Stat::CritRate, 0.0662),
        (Stat::CritDMG, 0.1324),
        (Stat::EnergyRecharge, 0.1102),
    ]);
    diluc.add_table(weapon.iter());
    diluc.add_table(artifacts.iter());
    
    assert_aprx!(diluc.get(&Stat::BaseATK), 844.85, 0.1);
    assert_aprx!(diluc.get(&Stat::ATKPercent), 0.5652, 0.1);
    assert_aprx!(diluc.get(&Stat::FlatATK), 344.08, 0.1);
    assert_aprx!(diluc.get(&Stat::CritRate), 0.6192  , 0.01);
    assert_aprx!(diluc.get(&Stat::CritDMG), 0.6324  , 0.01);
    assert_aprx!(diluc.get(&Stat::ElementalMastery), 204.64, 1.0);
    assert_aprx!(diluc.get(&Stat::EnergyRecharge), 1.11, 0.1);
    assert_aprx!(diluc.get(&Stat::PyroDMGBonus), 0.466, 0.1);
    
    let t= total_atk(&diluc);
    assert_aprx!(t, 1667.0, 10.0);

    let skill_formula = Box::new(|s: &dyn Statable| {
        
        let mut stat_table = StatTable::new();
        stat_table.add_table(s.iter());

        DMGFunction::calculate_damage(
            Element::Pyro, 
            DamageType::Skill, 
            BaseScaling::ATK, 
            Amplifier::None, 
            1.0, 
            1.0, 
            Box::new(&stat_table), 
            None  
        )
    });
    let multip = skill_formula(&diluc);
    assert_aprx!(multip, 1490.609, 0.1);
    

    let r = Rotation::of(vec![
        (String::from("skill vape"), skill_formula),
    ]);

    let res = r.evaluate(&diluc);
    let expected = 1490.609;
    assert_aprx!(res, expected, 1.0);
}




#[test] fn kqm_damage_calculation() {
    let diluc = StatFactory::get_character_base_stats("diluc").unwrap()
        .chain(Box::new(StatFactory::get_weapon_stats("rainslasher").unwrap()))
        .chain(Box::new(StatTable::of(&[
            (Stat::ATKPercent, 0.2),
            (Stat::ATKPercent, 0.48),
            (Stat::FlatATK, 191.16+565.*1.32),
            (Stat::ElementalMastery, 50.+(679.92*0.2)),
            (Stat::PyroDMGBonus, 0.36+0.15),
            (Stat::PyroResistanceReduction, 0.4),
            (Stat::ReactionBonus, 0.15),
        ])));

    let rotation = Rotation::of(vec![
        (String::from("N1_1"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.65, 1.0, Box::new(s), None))),
        (String::from("E_1"),  Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 1.60, 1.0, Box::new(s), None))),
        (String::from("N1_2"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.65, 1.0, Box::new(s), None))),
        (String::from("N2_1"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.61, 1.0, Box::new(s), None))),
        (String::from("N3_1"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.8154, 1.0, Box::new(s), None))),
        (String::from("N4_1"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 2.4616, 1.0, Box::new(s), None))),
        (String::from("E_2"),  Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 1.6529, 1.0, Box::new(s), None))),
        (String::from("N1_2"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.65, 1.0, Box::new(s), None))),
        (String::from("N2_2"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.61, 1.0, Box::new(s), None))),
        (String::from("N3_2"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.8154, 1.0, Box::new(s), None))),
        (String::from("N4_2"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 2.4616, 1.0, Box::new(s), None))),
        (String::from("E_3"),  Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 2.1865, 1.0, Box::new(s), None))),
        (String::from("N1_3"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.65, 1.0, Box::new(s), None))),
        (String::from("N2_3"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.61, 1.0, Box::new(s), None))),
        (String::from("N3_3"), Box::new(|s| DMGFunction::calculate_damage(Element::Pyro, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 1.8154, 1.0, Box::new(s), None))),
    ]);

    
}










#[macro_export]
macro_rules! assert_aprx {
    ($left:expr, $right:expr, $epsilon:expr $(,)?) => {{
        let (left_val, right_val, epsilon_val) = ($left, $right, $epsilon);
        if (left_val - right_val).abs() > epsilon_val {
            panic!(
                "assertion failed: `(left ≈ right)` \
                 (left: `{}`, right: `{}`, epsilon: `{}`)",
                left_val, right_val, epsilon_val
            );
        }
    }};
}
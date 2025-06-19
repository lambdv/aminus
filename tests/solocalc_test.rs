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

    let res = r.execute(&diluc);
    let expected = 1490.609;
    assert_aprx!(res, expected, 1.0);
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
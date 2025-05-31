use aminus::functions::formulas::formulas::*;
use aminus::model::stattable::*;
use aminus::model::statable::*;
use aminus::model::rotation::Rotation;
// use aminus::formulas::formulas::*;
use aminus::model::stat::*;
macro_rules! assert_aprx {
    ($left:expr, $right:expr, $epsilon:expr) => {
        assert!(($left - $right).abs() < $epsilon);
    };
}

#[test] fn primative_solution_result() {
    let mut diluc = StatTable::of(&[
        //(Stat::BaseHP, 12980.67),
        (Stat::BaseATK, 334.85),
        (Stat::CritRate, 0.192),
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
        (Stat::DEFPercent, 0.08),
        (Stat::FlatDEF, 12.4),
        (Stat::ElementalMastery, 393.0),
        (Stat::CritRate, 0.06),
        (Stat::CritDMG, 3.96),
        (Stat::EnergyRecharge, 0.64),
    ]);
    diluc.add_table(&weapon);
    diluc.add_table(&artifacts);
    
    assert_aprx!(diluc.get(&Stat::BaseATK), 844.85, 0.1);
    assert_aprx!(diluc.get(&Stat::ATKPercent), 0.5652, 0.1);
    assert_aprx!(diluc.get(&Stat::FlatATK), 344.08, 0.1);


    // assert_eq!(total_atk(&diluc) , 1667.0);


    let skill_formula = Box::new(|s: &dyn Statable| calculate_damage(
        Element::Pyro, 
        DamageType::Skill, 
        BaseScaling::ATK, 
        Amplifier::Reverse, 
        1.0, 
        1.0, 
        s, 
        None  
    ));

    let r = Rotation::of(vec![
        (String::from("skill vape"), skill_formula),
    ]);


    let res = r.execute(&diluc);
    println!("{:?}", res);


}

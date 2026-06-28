use aminus::core::stattable::*;
use aminus::core::rotation::Rotation;
use aminus::core::types::*;
use aminus::functions::stat_factory::StatFactory;
use aminus::functions::formulas::formulas::*;
use aminus::functions::dmg_function::DMGFunction;
use aminus::functions::optimizers::*;
use aminus::stats;
use aminus::rotation;

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

    let skill_formula = Box::new(|s: &StatTable| {
        DMGFunction::calculate_damage(
            Element::Pyro, 
            DamageType::Skill, 
            BaseScaling::ATK, 
            Amplifier::None, 
            1.0, 
            1.0, 
            s, 
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

fn default_cryo_na_formula(name: &str, x: &StatTable, multi: f32, num: i8, option: Option<f32>) -> (String, Box<dyn Fn(&StatTable) -> f32 + 'static>) {
    let num = num;
    let multi = multi;
    (String::from(name), Box::new(move |s| DMGFunction::calculate_damage(Element::Cryo, DamageType::Normal, 
        BaseScaling::ATK, Amplifier::None, num as f32, multi, s, None)))
}

fn default_cryo_e_formula(name: &str, x: &StatTable, multi: f32, num: i8, option: Option<f32>) -> (String, Box<dyn Fn(&StatTable) -> f32 + 'static>) {
    let num = num;
    let multi = multi;
    (String::from(name), Box::new(move |s| DMGFunction::calculate_damage(Element::Cryo, DamageType::Skill, 
        BaseScaling::ATK, Amplifier::None, num as f32, multi, s, None)))
}

fn default_cryo_q_formula(name: &str, x: &StatTable, multi: f32, num: i8, option: Option<f32>) -> (String, Box<dyn Fn(&StatTable) -> f32 + 'static>) {
    let num = num;
    let multi = multi;
    (String::from(name), Box::new(move |s| DMGFunction::calculate_damage(Element::Cryo, DamageType::Burst, 
        BaseScaling::ATK, Amplifier::None, num as f32, multi, s, None)))
}


#[test] fn kqm_damage_calculation() {
    let ayaka = StatFactory::get_character_base_stats("ayaka", 90).unwrap()
        .chain(StatFactory::get_weapon_base_stats("mistsplitter", 90).unwrap())
        .chain(StatTable::of(&[
            (Stat::ATKPercent, 0.88),
            (Stat::CritRate, 0.55),
            (Stat::CryoDMGBonus, 0.73),
            (Stat::NormalATKDMGBonus, 0.3),
            (Stat::ChargeATKDMGBonus, 0.3),
            (Stat::CryoResistanceReduction, 0.4),
        ]));
    let rotation = Rotation::of(vec![
        default_cryo_na_formula("n1", &ayaka, 0.84, 3, None),
        default_cryo_na_formula("n2", &ayaka, 0.894, 2, None),
        default_cryo_na_formula("ca", &ayaka, 3.039, 2, None),
        default_cryo_e_formula("skill", &ayaka, 4.07, 2, None),
        default_cryo_q_formula("burstcuts", &ayaka, 1.91, 19, None),
        default_cryo_q_formula("burstexplosion", &ayaka, 2.86, 1, None),
    ]);
    let ayaka = optimizers::optimal_kqmc_5_artifacts_stats(&ayaka, &rotation, 1.30);
    let dps = rotation.evaluate(&ayaka)/21.;
    
    println!("dps: {}", dps);
}


#[test] fn kqm_damage_calculation_with_macros() {
    let ayaka = StatFactory::get_character_base_stats("ayaka", 90).unwrap()
        .chain(StatFactory::get_weapon_base_stats("mistsplitter", 90).unwrap())
        .chain(stats! {
            Stat::ATKPercent: 0.88,
            Stat::CritRate: 0.55,
            Stat::CryoDMGBonus: 0.73,
            Stat::NormalATKDMGBonus: 0.3,
            Stat::ChargeATKDMGBonus: 0.3,
            Stat::CryoResistanceReduction: 0.4,
        });
    let rotation = rotation! {
        ("n1", Element::Cryo, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 0.84, 3.0, None),
        ("n2", Element::Cryo, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 0.894, 2.0, None),
        ("ca", Element::Cryo, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 3.039, 2.0, None),
        ("skill", Element::Cryo, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 4.07, 2.0, None),
        ("burstcuts", Element::Cryo, DamageType::Burst, BaseScaling::ATK, Amplifier::None, 1.91, 19.0, None),
        ("burstexplosion", Element::Cryo, DamageType::Burst, BaseScaling::ATK, Amplifier::None, 2.86, 1.0, None),
    };
    let ayaka = optimizers::optimal_kqmc_5_artifacts_stats(&ayaka, &rotation, 1.30);
    let dps = rotation.evaluate(&ayaka)/21.;
    
    println!("dps: {}", dps);
}

// pub fn swirl_damage(
//     character_level: i8,
//     elemental_mastery: f32,
//     reaction_bonus: f32,
//     res_multiplier: f32,
//     instances: f32,
// ) -> f32 {
//     let level_multiplier = match character_level {
//         90 => 1446.85,
//         _ => todo!("add level multiplier table"),
//     };

//     transformative_reaction_damage(
//         level_multiplier,
//         0.6,
//         elemental_mastery,
//         reaction_bonus,
//         res_multiplier,
//         instances,
//     )
// }

// #[test]
// fn diluc_furina_bennett_xianyun_sheet_calc() {
//     let rotation_len = 22.0;

//     let diluc = stats! {
//         Stat::BaseATK: 845.0,
//         Stat::ATKPercent: 1.02,
//         Stat::FlatATK: 1342.0,
//         Stat::ElementalMastery: 79.0,
//         Stat::CritRate: 0.96,
//         Stat::CritDMG: 1.78,
//         Stat::PyroDMGBonus: 1.60,
//         Stat::PyroResistanceReduction: 0.40,
//         Stat::ReactionBonus: 0.15,
//     };

//     let diluc_rotation = rotation! {
//         // vv + 4pcNO + 0 cw stack low plunge Vape
//         ("low_plunge_vape_no_vv", Element::Pyro, DamageType::Plunging, BaseScaling::ATK, Amplifier::Forward, 3.29, 4.0, None),

//         // vv + 4pcNO + 0 cw stack collision plunge
//         ("collision_plunge_no_vv", Element::Pyro, DamageType::Plunging, BaseScaling::ATK, Amplifier::None, 1.6444, 2.0, None),

//         // vv + 0 cw stack low plunge Vape
//         ("low_plunge_vape_vv", Element::Pyro, DamageType::Plunging, BaseScaling::ATK, Amplifier::Forward, 3.29, 4.0, None),

//         // vv + 0 cw stack collision plunge
//         ("collision_plunge_vv", Element::Pyro, DamageType::Plunging, BaseScaling::ATK, Amplifier::None, 1.6444, 2.0, None),

//         // Xianyun A4 flat damage. This may need special handling if your DMGFunction
//         // only supports talent multiplier damage.
//         ("xianyun_a4_flat", Element::Pyro, DamageType::Plunging, BaseScaling::FlatDamage, Amplifier::None, 49837.0, 8.0, None),
//     };

//     let furina = stats! {
//         Stat::BaseHP: 15307.0,
//         Stat::HPPercent: 0.66,
//         Stat::FlatHP: 5288.0,
//         Stat::BaseATK: 698.0,
//         Stat::ATKPercent: 0.10,
//         Stat::FlatATK: 344.0,
//         Stat::CritRate: 0.69,
//         Stat::CritDMG: 1.78,
//         Stat::HydroDMGBonus: 0.46,
//         Stat::SkillDMGBonus: 0.70,
//         Stat::HydroResistanceReduction: 0.18,
//     };

//     let furina_rotation = rotation! {
//         ("ousia_bubble_cast", Element::Hydro, DamageType::Skill, BaseScaling::HP, Amplifier::None, 0.13, 1.0, None),
//         ("gentilhomme_usher", Element::Hydro, DamageType::Skill, BaseScaling::HP, Amplifier::None, 0.14, 6.0, None),
//         ("surintendante_chevalmarin", Element::Hydro, DamageType::Skill, BaseScaling::HP, Amplifier::None, 0.14, 12.0, None),
//         ("mademoiselle_crabaletta", Element::Hydro, DamageType::Skill, BaseScaling::HP, Amplifier::None, 0.14, 4.0, None),
//         ("let_the_people_rejoice", Element::Hydro, DamageType::Burst, BaseScaling::HP, Amplifier::None, 0.19, 1.0, None),
//     };

//     let bennett = stats! {
//         Stat::BaseATK: 756.0,
//         Stat::ATKPercent: 0.55,
//         Stat::FlatATK: 1342.0,
//         Stat::CritRate: 0.69,
//         Stat::CritDMG: 1.29,
//         Stat::PyroDMGBonus: 0.47,
//         Stat::BurstDMGBonus: 0.20,
//         Stat::ElementalMastery: 40.0,
//     };

//     let bennett_rotation = rotation! {
//         ("buffed_vape_e", Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::Forward, 2.75, 1.0, None),
//         ("unbuffed_unreacted_e", Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 2.75, 2.0, None),
//         ("vape_q", Element::Pyro, DamageType::Burst, BaseScaling::ATK, Amplifier::Forward, 4.66, 1.0, None),
//     };

//     let xianyun = stats! {
//         Stat::BaseATK: 845.0,
//         Stat::ATKPercent: 2.43,
//         Stat::FlatATK: 1342.0,
//         Stat::CritRate: 0.31,
//         Stat::CritDMG: 0.63,
//         Stat::AnemoDMGBonus: 0.15,
//         Stat::ElementalMastery: 40.0,
//         Stat::PhysicalResistanceReduction: 0.40,
//         Stat::ReactionBonus: 0.60,
//     };

//     let xianyun_rotation = Rotation::of(vec![
//     (
//         String::from("infused_na"),
//         Box::new(move |s| {
//             DMGFunction::calculate_damage(
//                 Element::Pyro,
//                 DamageType::Normal,
//                 BaseScaling::ATK,
//                 Amplifier::None,
//                 1.0,
//                 0.69,
//                 s,
//                 None,
//             )
//         }),
//     ),
//     (
//         String::from("skill_cast"),
//         Box::new(move |s| {
//             DMGFunction::calculate_damage(
//                 Element::Anemo,
//                 DamageType::Skill,
//                 BaseScaling::ATK,
//                 Amplifier::None,
//                 1.0,
//                 0.42,
//                 s,
//                 None,
//             )
//         }),
//     ),
//     (
//         String::from("burst_cast"),
//         Box::new(move |s| {
//             DMGFunction::calculate_damage(
//                 Element::Anemo,
//                 DamageType::Burst,
//                 BaseScaling::ATK,
//                 Amplifier::None,
//                 1.0,
//                 1.84,
//                 s,
//                 None,
//             )
//         }),
//     ),
//     (
//         String::from("burst_tick"),
//         Box::new(move |s| {
//             DMGFunction::calculate_damage(
//                 Element::Anemo,
//                 DamageType::Burst,
//                 BaseScaling::ATK,
//                 Amplifier::None,
//                 8.0,
//                 0.666,
//                 s,
//                 None,
//             )
//         }),
//     ),
//     (
//         String::from("swirl"),
//         Box::new(move |_| {
//             // 12 swirls = 22,893 total damage from the spreadsheet
//             22_893.0
//         }),
//     ),
// ]);

//     let diluc_dpr = diluc_rotation.evaluate(&diluc);
//     let furina_dpr = furina_rotation.evaluate(&furina);
//     let bennett_dpr = bennett_rotation.evaluate(&bennett);
//     let xianyun_dpr = xianyun_rotation.evaluate(&xianyun);

//     let total_dpr = diluc_dpr + furina_dpr + bennett_dpr + xianyun_dpr;
//     let total_dps = total_dpr / rotation_len;

//     println!("Diluc DPR: {diluc_dpr}");
//     println!("Furina DPR: {furina_dpr}");
//     println!("Bennett DPR: {bennett_dpr}");
//     println!("Xianyun DPR: {xianyun_dpr}");
//     println!("Total DPR: {total_dpr}");
//     println!("Total DPS: {total_dps}");

//     // assert!((total_dpr - 1_452_779.0).abs() < 10_000.0);
//     // assert!((total_dps - 66_035.0).abs() < 500.0);
// }




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
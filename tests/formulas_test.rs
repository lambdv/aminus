// use aminus::stat::Stat;
// use aminus::functions::formulas::formulas::*;
// use aminus::model::stattable::*;

// #[test]
// fn test_atk_function() {
//     let base = 42.0;
//     let flat = 100.0;
//     let percentage = 1.2;

//     let expected: f32 = base * (1.0+percentage) + flat;

//     let actual: f32 = total_atk(
//         &StatTable::of(&[
//             (Stat::BaseATK, base),
//             (Stat::ATKPercent,percentage),
//             (Stat::FlatATK, flat),
//         ])
//     );

//     debug_assert_eq!(expected, actual);
// }

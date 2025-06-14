// use crate::model::stat::Stat;
// use crate::model::stattable::StatTable;
// use crate::model::statable::*;
// use crate::model::statable::ModifiableStatable;

// pub struct ArtifactSpec {
//     rarity: i8,
//     level: i8,
//     stat_type: Stat,
//     sub_stats: StatTable,
//     _set: String
// }
// pub enum Artifact{
//     Flower(ArtifactSpec),
//     Feather(ArtifactSpec),
//     Sands(ArtifactSpec),
//     Goblet(ArtifactSpec),
//     Circlet(ArtifactSpec),
// }

// impl Artifact {
//     // pub fn new_flower() -> Self {
//     //     let spec=  ArtifactSpec {
//     //         rarity: 20,
//     //         level: 20,
//     //         stat_type: Stat::None,

//     //     };
//     //     Artifact::Flower(spec)
//     // }
// }

// // impl Statable for Artifact {
// //     fn iter(&self) -> StatableIter {
// //         match self {
// //             Artifact::Flower(spec) => Box::new(spec.sub_stats.iter()),
// //             Artifact::Feather(spec) => Box::new(spec.sub_stats.iter()),
// //             Artifact::Sands(spec) => Box::new(spec.sub_stats.iter()),
// //             Artifact::Goblet(spec) => Box::new(spec.sub_stats.iter()),
// //             Artifact::Circlet(spec) => Box::new(spec.sub_stats.iter()),
// //         }
// //     }
// // }

// pub struct Flower {inner: ArtifactSpec}
// impl Statable for Flower {
//     fn iter(&self) -> StatableIter{
//         let main_stat_value = 0.0;
//         Box::new(
//             vec![(Stat::FlatATK, main_stat_value)].into_iter().chain(self.inner.sub_stats.iter())
//         )
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test] fn manual_construction() {
//     }
// }

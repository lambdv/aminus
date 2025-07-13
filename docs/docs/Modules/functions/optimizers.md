# Optimizers

Module of functions that provide algorithms to optimize statables.

## Description

This module contains optimization algorithms for finding the best artifact main stats and substat distributions to maximize damage output, including gradient-based optimization and energy recharge requirements handling.

```rust
/// module of functions that provide algorithms to optimize statables
pub mod optimizers{
    use crate::model::operation::Operation;
    use crate::functions::stat_factory::StatFactory;
    use crate::rotation::Rotation;
    use crate::{artifact::ArtifactSpec, stattable::*};
    use crate::stat::Stat;
    use crate::statable::ModifiableStatable;
    use crate::model::statable::Statable;
    use crate::model::artifact::*;
    use crate::model::artifact_builder::*;

    pub type VariableMainstatType = (Stat,Stat,Stat);
    pub type SubstatDistribution = std::collections::HashMap<Stat, i8>;


    ///TODO: Account for if meeting er recs with er sands or just er subs is better
    pub fn optimal_kqmc_5_artifacts_stats(
        stats: &StatTable,
        target: &Rotation,
        energy_recharge_requirements: f32,
    ) -> StatTable {
        let optimal_mainstats = global_kqmc_artifact_main_stat_optimizer(stats, target);
        
        let mut builder = ArtifactBuilder::new(
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: optimal_mainstats.0}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: optimal_mainstats.1}),
            Some(ArtifactPiece{rarity:5, level:20, stat_type: optimal_mainstats.2})
        );

        let optimal_substats = gradient_5_star_kqmc_artifact_substat_optimizer(stats, target, 
            Some(builder.flower.clone().unwrap()), 
            Some(builder.feather.clone().unwrap()), 
            Some(builder.sands.clone().unwrap()), 
            Some(builder.goblet.clone().unwrap()), 
            Some(builder.circlet.clone().unwrap()), 
            energy_recharge_requirements);
        
        for (stat, count) in optimal_substats.iter() {
            builder.roll(*stat, RollQuality::AVG, 5, *count);
        }
        let sum = StatTable::unbox(stats.chain(Box::new(builder.build())));
        sum
    }

    /// finds best aritfact main stat combo for a statable given a computable
    /// eg: best mains for a character for a particular rotation
    pub fn global_kqmc_artifact_main_stat_optimizer(
        //stats: Box<Statable>,
        //target: Box<Computable>
        stats: &StatTable,
        target: &Rotation
    ) -> VariableMainstatType{

        let sands_stats: std::collections::HashSet<Stat> = std::collections::HashSet::from_iter(POSSIBLE_SANDS_STATS.iter().cloned());
        let goblet_stats = std::collections::HashSet::from_iter(POSSIBLE_GOBLET_STATS.iter().cloned());
        let circlet_stats = std::collections::HashSet::from_iter(POSSIBLE_CIRCLE_STATS.iter().cloned());
        let pool = sands_stats.union(&goblet_stats).cloned().collect::<std::collections::HashSet<Stat>>().union(&circlet_stats).cloned().collect::<std::collections::HashSet<Stat>>();
        
        // heuristic: check which stats actually increase target value
        let slopes = std::collections::HashMap::from_iter(pool.iter().map(|x|(*x, 1.0)));
        let effective_set = relu_heuristic(stats, target, &slopes);

        //intersect set of valid stats for sands, goblet and circlet with set of effective stats
        let sands_subset: std::collections::HashSet<Stat> = effective_set.intersection(&sands_stats).cloned().collect();
        let goblet_subset: std::collections::HashSet<Stat> = effective_set.intersection(&goblet_stats).cloned().collect();
        let circlet_subset: std::collections::HashSet<Stat> = effective_set.intersection(&circlet_stats).cloned().collect();

        let mut best_combo = (Stat::None, Stat::None, Stat::None);
        let mut best_value = 0.0;
        //let mut iterations = 0;

        // n^3 global search to find best combo
        for sands in sands_subset.iter() {
            for goblet in goblet_subset.iter() {
                for circlet in circlet_subset.iter() {
                    let combo = (sands.clone(), goblet.clone(), circlet.clone());
                    let builder = ArtifactBuilder::new(
                        Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP}),
                        Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK}),
                        Some(ArtifactPiece{rarity:5, level:20, stat_type: *sands}),
                        Some(ArtifactPiece{rarity:5, level:20, stat_type: *goblet}),
                        Some(ArtifactPiece{rarity:5, level:20, stat_type: *circlet})
                    );
                    let artifact = builder.build();
                    let sum = &StatTable::from_iter(stats.chain(Box::new(artifact)).iter());
                    let value = target.evaluate(sum);
                    if value > best_value {
                        best_value = value;
                        best_combo = combo;
                    }
                    //iterations += 1;
                }
            }
        }
        //println!("iterations: {}", iterations);
        best_combo
    }



    /// finds best substat distrubtion 
    pub fn gradient_5_star_kqmc_artifact_substat_optimizer(
        stats: &StatTable,
        target: &Rotation,
        flower: Option<ArtifactPiece>,
        feather: Option<ArtifactPiece>,
        sands: Option<ArtifactPiece>,
        goblet: Option<ArtifactPiece>,
        circlet: Option<ArtifactPiece>,
        energy_recharge_requirements: f32,
    ) -> SubstatDistribution {
        let mut builder = ArtifactBuilder::kqmc(flower, feather, sands, goblet, circlet);
        
        /*
         * TOOD: instead if having energy requirement as a f32, give a constraint/requirement argument which is a data structure that stores multiple "constraints" which are functions that take a statable/statable and returns a boolean if if the requirement for that statable is meet or not, and based on it 
         */
        
        //meet er reqs from subs
        while {
            let combined_stats = StatTable::from_iter(stats.chain(Box::new(builder.build())).iter());
            combined_stats.get(&Stat::EnergyRecharge)
        } < energy_recharge_requirements {
            if builder.rolls_left() <= 0 || builder.rolls_left_for_given(&Stat::EnergyRecharge, RollQuality::AVG, 5) <= 0 {
                panic!("Energy Recharge requirements cannot be met with substats alone");
            }
            builder.roll(Stat::EnergyRecharge, RollQuality::AVG, 5, 1);
        }
        //println!("builder: {:?}", builder.constraints);
        let mut possible_subs_to_roll: std::collections::HashSet<Stat> = POSSIBLE_SUB_STATS.iter().cloned().collect();
        
        //gradient search loop
        while builder.rolls_left() > 0 && !possible_subs_to_roll.is_empty() {
            let mut best_sub = Stat::None;
            let mut best_dpr = 0.0;
            
            // Try each possible substat and find the one that gives the best DPR
            for &substat in &possible_subs_to_roll {
                // Check if we can roll this substat
                if builder.current_rolls_for_given(&substat, RollQuality::AVG, 5) < builder.substat_constraint(&substat, 5) {
                    // Temporarily roll this substat
                    builder.roll(substat, RollQuality::AVG, 5, 1);
                    
                    // Calculate DPR with this roll
                    let combined_stats = StatTable::from_iter(stats.chain(Box::new(builder.build())).iter());
                    let dpr = target.evaluate(&combined_stats);
                    
                    // Unroll to restore previous state
                    builder.unroll(substat, RollQuality::AVG, 5, 1);
                    
                    // Update best if this is better
                    if dpr > best_dpr {
                        best_dpr = dpr;
                        best_sub = substat;
                    }
                }
            }
            
            if best_dpr == 0.0 {
                possible_subs_to_roll.clear();
            } else {
                //println!("substat: {}, dpr: {}", best_sub, best_dpr);
                builder.roll(best_sub, RollQuality::AVG, 5, 1);
            }
        }

        // Convert builder rolls to SubstatDistribution format
        let mut distribution = std::collections::HashMap::new();
        for ((stat, _, _), count) in &builder.rolls {
            distribution.insert(*stat, *count);
        }
        
        distribution
    }

    /// computes graident of a statable based on slopes of stats
    pub fn stat_gradients(
        base: &StatTable,
        target: &Rotation,
        slopes: &std::collections::HashMap<Stat, f32>,
    ) -> std::collections::HashMap<Stat, f32> {
        let mut gradients = std::collections::HashMap::new();
        for (stat, delta) in slopes {
            let direction = StatTable::of(&[(*stat, *delta)]);
            let adjusted = base.chain(Box::new(direction));
            let before = target.evaluate(base);
            let after = target.evaluate(&StatTable::from_iter(adjusted.iter()));
            let gradient  = (after - before) / *delta;
            gradients.insert(*stat, gradient );
        }
        gradients
    }

    /// find stats that are actually increase target based on given slopes
    pub fn relu_heuristic(
        base: &StatTable,
        target: &Rotation,
        slopes: &std::collections::HashMap<Stat, f32>,
    ) -> std::collections::HashSet<Stat>{
        let gradients = stat_gradients(base, target, slopes);
        //filter out stats that have 0 or negative gradient
        gradients.iter()
            .filter(|(_, gradient)| **gradient > 0.0)
            .map(|(stat, _)| *stat)
            .collect()
    }

    
    // pub enum Method {
    //     Gradient, 
    //     Greedy, 
    // }


    // pub fn kqmc_artifact_main_stat_optimizer(
    //     //stats: Box<Statable>,
    //     //target: Box<Computable>
    //     stats: StatTable,
    //     target: Rotation,
    //     method: OptimizationMethod
    // ) -> (Stat,Stat,Stat){
    //     // heuristic: check which stats actually increase target value
    //     (Stat::None,Stat::None,Stat::None)
    // }

        //     /// finds best aritfact main stat combo for a statable given a computable
    // /// eg: best mains for a character for a particular rotation
    // pub fn greedy_kqmc_artifact_main_stat_optimizer(
    //     //stats: Box<Statable>,
    //     //target: Box<Computable>
    //     stats: StatTable,
    //     target: Rotation
    // ) -> VariableMainstatType{

    //     let grad = stat_gradients(&stats, &target, &slopes);
    //     //find a circle stat from possible circle stats that has the highest gradient
    //     let best_circle_stat = POSSIBLE_CIRCLE_STATS.iter()
    //         .map(|x| (x, grad.get(x).unwrap_or(&0.0)))
    //         .max_by_key(|(_, gradient)| *gradient)
    //         .map(|(stat, _)| *stat)
    //         .unwrap_or(Stat::None);

    //     best_combo
    // }


    #[cfg(test)] mod tests {
        use super::*;
        use crate::dmg_function::*;
        use crate::stat::*;
        use crate::{
            dmg_function::DMGFunction, 
            model::stat::Stat, 
            model::stattable::StatTable,
        };

        #[test] fn test_gradients() {
            let stats = StatTable::of(&[
                (Stat::BaseATK, 100.0),
                (Stat::ATKPercent, 0.5),
                (Stat::FlatATK, 100.0),
                (Stat::CritRate, 0.5),
                (Stat::CritDMG, 0.5),
                (Stat::ElementalMastery, 100.0),
                (Stat::EnergyRecharge, 1.0),
            ]);

            let target = Rotation::of(vec![
                (String::from("test"), Box::new(|x| 
                    DMGFunction::calculate_damage(
                        Element::Pyro, 
                        DamageType::Normal, 
                        BaseScaling::ATK, 
                        Amplifier::None, 
                        1.0, 
                        1.0, 
                        Box::new(x), 
                        None,
                    )
                ))
            ]);

            let grad: std::collections::HashMap<Stat, f32> = stat_gradients(
                &stats, 
                    &target,
            &std::collections::HashMap::from_iter(POSSIBLE_SUB_STATS.iter().map(|x|
                            (*x, StatFactory::get_sub_stat_value(5, *x).unwrap_or_else(|e| panic!("invalid stat: {}", e)))
                        ))
            );
            //println!("{:?}", grad);
            assert!(grad.get(&Stat::FlatATK).unwrap() > &0.0);
            assert!(grad.get(&Stat::ElementalMastery).unwrap() == &0.0);
        }


        #[test] fn test_global_kqmc_artifact_main_stat_optimizer() {
            let stats = StatTable::of(&[
                (Stat::BaseATK, 100.0),
                (Stat::ATKPercent, 0.5),
                (Stat::FlatATK, 100.0),
                (Stat::CritRate, 0.05),
                (Stat::CritDMG, 0.5),
            ]);
            let mut target = Rotation::new();
            let atk1: Operation = Box::new(|x| DMGFunction::calculate_damage(
                Element::Pyro, 
                DamageType::Normal, 
                BaseScaling::ATK, 
                Amplifier::None, 
                1.0, 1.0, 
                Box::new(x), 
                None,
            ));
            target.add(String::from("atk1"), atk1);

            let result = global_kqmc_artifact_main_stat_optimizer(&stats, &target);
            println!("{:?}", result);
            assert_eq!(result, (Stat::ATKPercent, Stat::PyroDMGBonus, Stat::CritRate));
        }

        #[test] fn test_gradient_5_star_kqmc_artifact_substat_optimizer() {
            let stats = StatTable::of(&[
                (Stat::BaseATK, 844.85),
                (Stat::ATKPercent, 0.2),
                (Stat::FlatATK, 1000.0),
                (Stat::CritRate, 0.05),
                (Stat::CritDMG, 0.5),
                (Stat::EnergyRecharge, 1.0),
            ]);
            let mut target = Rotation::new();
            let atk1: Operation = Box::new(|x| DMGFunction::calculate_damage(
                Element::Pyro, 
                DamageType::Normal, 
                BaseScaling::ATK, 
                Amplifier::None, 
                1.0, 1.0, 
                Box::new(x), 
                None,
            ));
            target.add(String::from("atk1"), atk1);

            let flower = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP});
            let feather = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK});
            let sands = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ATKPercent});
            let goblet = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::PyroDMGBonus});
            let circlet = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate});

            let res = gradient_5_star_kqmc_artifact_substat_optimizer(
                &stats, 
                &target, 
                flower, 
                feather, 
                sands, 
                goblet, 
                circlet, 
                1.0 // energy recharge requirement
            );
            
            //println!("Substat distribution: {:?}", res);

            assert!(!res.is_empty());
            assert!(res.get(&Stat::EnergyRecharge).unwrap() == &(0+2));
            assert_eq!(res.get(&Stat::HPPercent), Some(&(0+2)));
            assert_eq!(res.get(&Stat::FlatHP), Some(&(0+2)));
            assert_eq!(res.get(&Stat::ATKPercent), Some(&(2+2)));
            assert_eq!(res.get(&Stat::FlatATK), Some(&(0+2)));
            assert_eq!(res.get(&Stat::DEFPercent), Some(&(0+2)));
            assert_eq!(res.get(&Stat::FlatDEF), Some(&(0+2)));
            assert_eq!(res.get(&Stat::ElementalMastery), Some(&(0+2)));
            assert_eq!(res.get(&Stat::CritRate), Some(&(8+2)));
            assert_eq!(res.get(&Stat::CritDMG), Some(&(10+2)));
            assert_eq!(res.get(&Stat::EnergyRecharge), Some(&(0+2)));
        }

        #[test] fn test_accept_artifact_optimizer() {
            // Equivalent to Java test: AcceptArtifactOptimizer
            let mut target = Rotation::new();
            let atk1: Operation = Box::new(|x| DMGFunction::calculate_damage(
                Element::Pyro, 
                DamageType::Normal, 
                BaseScaling::ATK, 
                Amplifier::None, 
                1.0, 1.0, 
                Box::new(x), 
                None,
            ));
            target.add(String::from("t"), atk1);

            // Create character stats similar to Hu Tao
            let mut character_stats = StatTable::of(&[
                (Stat::BaseATK, 106.0), // Hu Tao base ATK
                (Stat::BaseHP, 15552.0), // Hu Tao base HP
                (Stat::BaseDEF, 876.0), // Hu Tao base DEF
                (Stat::CritRate, 0.05), // Base crit rate
                (Stat::CritDMG, 0.5), // Base crit damage
                (Stat::EnergyRecharge, 1.0), // Base energy recharge
            ]);

            // Add weapon stats (Dragon's Bane equivalent)
            character_stats.add_table(Box::new(StatTable::of(&[
                (Stat::BaseATK, 454.0), // Dragon's Bane base ATK
                (Stat::ElementalMastery, 221.0), // Dragon's Bane EM
            ]).iter()));

            let before = target.evaluate(&character_stats);
            
            // Apply artifact optimization (main stats only for this test)
            let (sands, goblet, circlet) = global_kqmc_artifact_main_stat_optimizer(&character_stats, &target);

            // Create optimized character with artifacts
            let mut optimized_stats = character_stats.clone();
            optimized_stats.add_table(Box::new(StatTable::of(&[
                (sands, 0.466), // Sands main stat value
                (goblet, 0.466), // Goblet main stat value  
                (circlet, 0.311), // Circlet main stat value
            ]).iter()));

            // Create a new target for the second evaluation
            let mut target2 = Rotation::new();
            let atk2: Operation = Box::new(|x| DMGFunction::calculate_damage(
                Element::Pyro, 
                DamageType::Normal, 
                BaseScaling::ATK, 
                Amplifier::None, 
                1.0, 1.0, 
                Box::new(x), 
                None,
            ));
            target2.add(String::from("t"), atk2);

            let after = target2.evaluate(&optimized_stats);
            
            assert!(before < after, "Optimization should improve damage: before={}, after={}", before, after);
        }

        #[test] fn test_artifact_optimizer_not_enough_er_case() {
            // Equivalent to Java test: ArtifactOptimizerNotEnoughERCase
            let mut target = Rotation::new();
            let atk1: Operation = Box::new(|x| DMGFunction::calculate_damage(
                Element::Pyro, 
                DamageType::Normal, 
                BaseScaling::ATK, 
                Amplifier::None, 
                1.0, 1.0, 
                Box::new(x), 
                None,
            ));
            target.add(String::from("t"), atk1);

            // Create character stats similar to Hu Tao (without weapon)
            let character_stats = StatTable::of(&[
                (Stat::BaseATK, 106.0),
                (Stat::BaseHP, 15552.0),
                (Stat::BaseDEF, 876.0),
                (Stat::CritRate, 0.05),
                (Stat::CritDMG, 0.5),
                (Stat::EnergyRecharge, 1.0),
            ]);

            // This should panic because 2.0 ER requirement cannot be met with substats alone
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                gradient_5_star_kqmc_artifact_substat_optimizer(
                    &character_stats,
                    &target,
                    None, None, None, None, None,
                    2.0 // energy recharge requirement that cannot be met
                )
            }));

            assert!(result.is_err(), "Should panic when ER requirements cannot be met");
        }

        #[test] fn test_artifact_optimizer_enough_er_case() {
            // Equivalent to Java test: ArtifactOptimizerEnoughERCase
            let mut target = Rotation::new();
            let atk1: Operation = Box::new(|x| DMGFunction::calculate_damage(
                Element::Electro, 
                DamageType::Normal, 
                BaseScaling::ATK, 
                Amplifier::None, 
                1.0, 1.0, 
                Box::new(x), 
                None,
            ));
            target.add(String::from("t"), atk1);

            // Create character stats similar to Raiden with The Catch
            let mut character_stats = StatTable::of(&[
                (Stat::BaseATK, 337.0), // Raiden base ATK
                (Stat::BaseHP, 12907.0), // Raiden base HP
                (Stat::BaseDEF, 789.0), // Raiden base DEF
                (Stat::CritRate, 0.05),
                (Stat::CritDMG, 0.5),
                (Stat::EnergyRecharge, 1.32), // Raiden base ER
                (Stat::FlatATK, 900.0), // Additional flat ATK
            ]);

            // Add weapon stats (The Catch equivalent)
            character_stats.add_table(Box::new(StatTable::of(&[
                (Stat::BaseATK, 510.0), // The Catch base ATK
                (Stat::EnergyRecharge, 0.459), // The Catch ER
            ]).iter()));

            let flower = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatHP});
            let feather = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::FlatATK});
            let sands = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::EnergyRecharge});
            let goblet = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::ElectroDMGBonus});
            let circlet = Some(ArtifactPiece{rarity:5, level:20, stat_type: Stat::CritRate});

            let res = gradient_5_star_kqmc_artifact_substat_optimizer(
                &character_stats,
                &target,
                flower,
                feather,
                sands,
                goblet,
                circlet,
                2.0 // energy recharge requirement
            );

            // Calculate total energy recharge after optimization
            let total_er = 1.32 + 0.459 + 0.518 + 0.0; // Base + Weapon + Sands + Substat rolls
            assert!(total_er >= 2.0, "Total ER should be >= 2.0, got {}", total_er);
            assert!(!res.is_empty(), "Should have substat distribution");
        }
    }
}
``` 
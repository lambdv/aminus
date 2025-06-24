/// module of functions that provide algorithms to optimize statables
pub mod optimizers{
    use crate::model::operation::Operation;
    use crate::functions::stat_factory::StatFactory;
    use crate::rotation::Rotation;
    use crate::{artifact::ArtifactSpec, stattable::*};
    use crate::stat::Stat;
    use crate::statable::ModifiableStatable;
    use crate::model::statable::Statable;
    use crate::model::artifact_builder::*;

    pub type VariableMainstatType = (Stat,Stat,Stat);

    /// finds best aritfact main stat combo for a statable given a computable
    /// eg: best mains for a character for a particular rotation
    pub fn global_kqmc_artifact_main_stat_optimizer(
        //stats: Box<Statable>,
        //target: Box<Computable>
        stats: StatTable,
        target: Rotation
    ) -> VariableMainstatType{
        // heuristic: check which stats actually increase target value
        let effective_set: std::collections::HashMap<Stat, f32> = stat_gradients(
        &stats, 
            &target,
    &std::collections::HashMap::from_iter(POSSIBLE_SUB_STATS.iter().map(|x|
                    (*x, StatFactory::get_sub_stat_value(5, *x).unwrap())
                ))
        );

        //intersect set of valid stats for sands, goblet and circlet with set of effective stats
        let effective_sands: std::collections::HashMap<String, String>;
        let effective_goblet: std::collections::HashMap<String, String>;
        let effective_circlet: std::collections::HashMap<String, String>;

        let best_combo = (Stat::None,Stat::None,Stat::None);


        //match method to use
            // n^3 global search to find best combo
            //greedy, use the stats that have the best values from heuristic test

        best_combo
    }

    pub type SubstatDistribution = std::collections::HashMap<Stat, i8>;
    /// finds best substat distrubtion 
    pub fn gradient_kqmc_artifact_substat_optimizer(
        stats: StatTable,
        target: Rotation 
    ) -> SubstatDistribution {
        let best_substat_distrubtion: SubstatDistribution = std::collections::HashMap::new();

        // let bulder = ArtifactBuilder::kqmc(
        //     Some
        // );

        best_substat_distrubtion
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
            let slope = (after - before) / *delta;
            gradients.insert(*stat, slope);
        }
        gradients
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

#[cfg(test)] mod tests {
    use super::*;
    use crate::dmg_function::*;
    use crate::stat::*;

    use crate::{
        dmg_function::DMGFunction, model::stat::Stat, stattable::StatTable,
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
}

}
/// module of functions that provide algorithms to optimize statables
pub mod optimizers{
    use crate::computable::Computable;
    use crate::factories::StatTableFactory;
    use crate::rotation::Rotation;
    use crate::{artifact::ArtifactSpec, stattable::*};
    use crate::stat::Stat;
    use crate::statable::ModifiableStatable;
    use crate::model::statable::Statable;
    use crate::model::artifact_builder::*;

    // pub enum OptimizationMethod {
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
    
    /// finds best aritfact main stat combo for a statable given a computable
    /// eg: best mains for a character for a particular rotation
    pub fn global_kqmc_artifact_main_stat_optimizer(
        //stats: Box<Statable>,
        //target: Box<Computable>
        stats: StatTable,
        target: Rotation
    ) -> (Stat,Stat,Stat){
        // heuristic: check which stats actually increase target value
        let effective_set: std::collections::HashMap<Stat, f32> = stat_gradients(
        &stats, 
            &target,
    &std::collections::HashMap::from_iter(POSSIBLE_SUB_STATS.iter().map(|x|
                    (*x, StatTableFactory::get_sub_stat_value(5, *x).unwrap())
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

    /// finds best substat distrubtion 
    pub fn gradient_kqmc_artifact_substat_optimizer(
        stats: StatTable,
        target: Rotation 
    ) -> std::collections::HashMap<Stat, i8> {
        let best_substat_distrubtion: std::collections::HashMap<Stat, i8> = std::collections::HashMap::new();

        best_substat_distrubtion
    }

    /// computes graident of a statable based on slopes of stats
    pub fn stat_gradients(
        base: &StatTable,
        target: &Rotation,
        slopes: &std::collections::HashMap<Stat, f32>,
    ) -> std::collections::HashMap<Stat, f32> {
        let mut graidents = std::collections::HashMap::new();
        //let s = std::collections::HashMap::into_iter();
        for (stat, delta) in slopes {
            let grad = StatTable::of(&[(*stat, *delta)]);
            let mut adjusted = base.clone();
            adjusted.add_table(Box::new(grad.iter()));
            let slope = target.execute(&adjusted);
            graidents.insert(*stat, slope);
        }
        graidents
    }


#[cfg(test)] mod tests {
    use super::*;
    use crate::{model::stat::Stat, stattable::StatTable};

    #[test] fn test_gradients() {
        let stats = StatTable::new();
        let target = Rotation::of(vec![
            (String::from("test"), Box::new(|x| 0.1))
        ]);

        let grad: std::collections::HashMap<Stat, f32> = stat_gradients(
            &stats, 
                &target,
        &std::collections::HashMap::from_iter(POSSIBLE_SUB_STATS.iter().map(|x|
                        (*x, StatTableFactory::get_sub_stat_value(5, *x).unwrap())
                    ))
        );

        println!("{:?}", grad);
    }
}

}
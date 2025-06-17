/// module of functions that provide algorithms to optimize statables
pub mod optimizers{
    use crate::computable::Computable;
    use crate::rotation::Rotation;
    use crate::{artifact::ArtifactSpec, stattable::*};
    use crate::stat::Stat;

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
        let effective_set: std::collections::HashMap<String, String>;

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
}

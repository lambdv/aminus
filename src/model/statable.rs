pub mod statable{
    use crate::model::stattable::StatTable;
    //use crate::model::stat::Stat;
    pub trait Statable {
        fn stats(&self) -> &StatTable;
    }
}
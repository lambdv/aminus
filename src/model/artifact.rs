pub mod artifacts{
    // use crate::model::statable::statable::Statable;
    // use crate::model::stattable::StatTable;
    pub enum Artifact{
        Flower,
    }

    // impl Statable for Artifact {
    //     fn stats(&self) -> &StatTable {
    //         // We need to store the StatTable as a field in the struct
    //         // or return a static reference instead of a temporary one
    //         // For now, returning a default empty table from a static reference
    //         static EMPTY_STAT_TABLE: once_cell::sync::Lazy<StatTable> = once_cell::sync::Lazy::new(|| StatTable::new());
    //         &EMPTY_STAT_TABLE
    //     }
    // }
}
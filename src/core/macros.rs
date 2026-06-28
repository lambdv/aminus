use crate::artifact;
use crate::core::types::Stat;
use crate::core::stattable::StatTable;
use std::collections::HashSet;

macro_rules! StatTable {
    () => {
        StatTable::of($kv)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let _ = StatTable!(&[(Stat::BaseATK, 42.0)]);
    }
}
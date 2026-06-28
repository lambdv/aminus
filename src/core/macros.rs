use crate::artifact;
use crate::core::types::Stat;
use crate::core::stattable::StatTable;
use std::collections::HashSet;

macro_rules! hello {
    () => {
        println!("Hello, world!");
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        hello!();
    }
}
pub mod rotation{
    use crate::model::stattable::StatTable;
    use crate::model::stat::Stat;

    // pub trait ComputableDamage {
    //     fn execute(s: &StatTable) -> f32;
    // }

    /// Data structure that represents a character's rotation or sequence of action configuration.
    /// 
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Rotation{        
        inner: std::collections::HashMap<String, fn(&StatTable)->f32>,
    }

    impl Rotation {
        pub fn new() -> Self {
            Self { inner: std::collections::HashMap::new() }
        }

        pub fn of(actions: &[(String, fn(&StatTable) -> f32)]) -> Self {
            Self { 
                inner: std::collections::HashMap::from_iter(
                    actions.iter().map(|(name, action)| (name.clone(), *action))
                )
            }
        }

        pub fn add(&mut self, name: String, action: fn(&StatTable)->f32) -> & mut Self {
            self.inner.insert(name, action);
            self
        }

        pub fn execute(&self, stats: &StatTable) -> f32 {
            self.inner.iter()
                .map(|x| x.1)
                .map(|x|x(stats))
                .sum()
        }
    }

    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_execute_empty() {
            let r = Rotation::new();
            let s = StatTable::new();

            let res = r.execute(&s);
            debug_assert_eq!(res, 0.0)
        }

        #[test]
        fn test_execute_populated() {
            let r = Rotation::of(&[
                (String::from("test"), |s| s.get(&Stat::FlatATK)),
                (String::from("test2"), |s| s.get(&Stat::FlatATK)),
            ]);

            let s = StatTable::of(&[
                (Stat::FlatATK, 1.6),
            ]);

            let res = r.execute(&s);
            
            debug_assert_eq!(res, 1.6*2.0)
        }
    }
}


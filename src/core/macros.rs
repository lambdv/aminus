
#[macro_export] macro_rules! stats {
    ($($statType:path : $statValue:expr),* $(,)?) => {{
        let mut res = $crate::core::stattable::StatTable::new();
        $(
            res.add(&$statType, $statValue as f32);
        )*
        res
    }};
}


#[macro_export] macro_rules! rotation {
    (
        $(
            ($name:literal, $element:expr, $damage_t:expr, $base_scaling:expr, $amplifier:expr, $multiplier:expr, $instances:expr, $buffs:expr)
        ),* $(,)?
    ) => {{
        let mut res = $crate::core::rotation::Rotation::new();

        $(
            let damage_function = Box::new(move |s: &$crate::core::stattable::StatTable| $crate::functions::dmg_function::DMGFunction::calculate_damage($element, $damage_t, $base_scaling, $amplifier, $instances, $multiplier, s, $buffs));
            res.add(String::from($name), damage_function);
        )*

        res
    }};
}



#[cfg(test)]
mod tests {
    use crate::artifact;
    use crate::core::types::*;
    use crate::core::stattable::StatTable;
    use crate::functions::dmg_function::DMGFunction;
    
    #[test]
    fn one_stat_stattable() {
        assert_eq!({
            let mut expected = StatTable::new();
            expected.add(&Stat::BaseATK, 41.0);
            expected
        }, stats!(Stat::BaseATK: 41.0))
    }

        #[test]
    fn multiple_same_stat_stattable() {
        assert_eq!({
            let mut expected = StatTable::new();
            expected.add(&Stat::BaseATK, 41.0 * 2.);
            expected
        }, stats! {
            Stat::BaseATK: 41.0, 
            Stat::BaseATK: 41.0,
        })
    }

    #[test]
    fn rotation_macro_single_entry() {
        let stats = StatTable::of(&[
            (Stat::BaseATK, 844.85),
            (Stat::ATKPercent, 0.5652),
            (Stat::FlatATK, 344.08),
            (Stat::CritRate, 0.6192),
            (Stat::CritDMG, 0.6324),
            (Stat::PyroDMGBonus, 0.466),
        ]);

        let r = rotation! {
            ("skill", Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 1.0, 1.0, None),
        };

        let dps = r.evaluate(&stats);
        assert!(dps > 0.0);
    }

    #[test]
    fn rotation_macro_multiple_entries() {
        let stats = StatTable::of(&[
            (Stat::BaseATK, 844.85),
            (Stat::ATKPercent, 0.5652),
            (Stat::FlatATK, 344.08),
            (Stat::CritRate, 0.6192),
            (Stat::CritDMG, 0.6324),
            (Stat::PyroDMGBonus, 0.466),
        ]);

        let r = rotation! {
            ("normal", Element::Physical, DamageType::Normal, BaseScaling::ATK, Amplifier::None, 0.84, 3.0, None),
            ("skill", Element::Pyro, DamageType::Skill, BaseScaling::ATK, Amplifier::None, 1.0, 1.0, None),
        };

        let dps = r.evaluate(&stats);
        assert!(dps > 0.0);
        assert_ne!(r.evaluate(&stats), 0.0);
    }
}


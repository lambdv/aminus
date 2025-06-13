/// module that provides function implementation of formulas to compute f32s from statables
pub mod formulas{
    use crate::model::stattable::*;
    use crate::model::stat::*;
    use crate::model::statable::*;
    
    pub fn total_atk(stats: &impl Statable) -> f32 {
        let base_atk = stats.get(&Stat::BaseATK);
        let atk_percent = stats.get(&Stat::ATKPercent);
        let flat_atk = stats.get(&Stat::FlatATK);
        base_atk * (1.0+atk_percent) + flat_atk
    }

    pub fn total_def(stats: &impl Statable) -> f32 {
        let flat_def = stats.get(&Stat::FlatDEF);
        let def_percent = stats.get(&Stat::DEFPercent);
        let base_def = stats.get(&Stat::BaseDEF);
        base_def * (1.0+def_percent) + flat_def
    }

    pub fn total_hp(stats: &impl Statable) -> f32 { 
        let flat_hp = stats.get(&Stat::FlatHP);
        let hp_percent = stats.get(&Stat::HPPercent);
        let base_hp = stats.get(&Stat::BaseHP);
        base_hp * (1.0+hp_percent) + flat_hp
    }

    pub fn avg_crit_multiplier(stats: &impl Statable) -> f32 {
        let cr = stats.get(&Stat::CritRate);
        let cr = if cr > 1.0 { 1.0 } else { cr };
        let cr = if cr < 0.0 { 0.0 } else { cr };

        let cd = stats.get(&Stat::CritDMG);
        1.0+(cr*cd)
    }

    pub fn def_multiplier(character_level: i8, enemy_level: i8, def_reduction: f32, def_ignore: f32,) -> f32{
        assert!(character_level >= 1);
        assert!(character_level <= 90);
        assert!(enemy_level >= 1);
        (character_level as f32 + 100.0) / (
            (character_level as f32 + 100.0) 
            + (enemy_level as f32 + 100.0)
            * (1.0 - f32::min(def_reduction, 0.9)) 
            * (1.0 - def_ignore)
        )
    }

    pub fn res_multiplier(enemy_base_resistance: f32, resistance_reduction: f32) -> f32 {
        let resistance = enemy_base_resistance - resistance_reduction;
        if resistance < 0.0 {
            1.0 - (resistance / 2.0)
        } else if resistance < 0.75 {
            1.0 - resistance
        } else { //resistance >= 0.75
            1.0 / (4.0 * resistance + 1.0)
        }
    }

    pub fn amplifier_multiplier(amplifier: f32, elemental_mastery: f32, reaction_bonus: f32) -> f32 {
        amplifier * (1.0 + (2.78 * elemental_mastery) / (1400.0 + elemental_mastery) + reaction_bonus)
    }

    pub fn full_damage_formula(
        instances: f32,
        total_scaling_stat: f32,
        motion_value: f32,
        base_dmg_multiplier: f32,
        additive_base_dmg_bonus: f32,
        avg_crit_multiplier: f32,
        total_dmg_bonus: f32,
        dmg_reduction_target: f32,
        def_multiplier: f32,
        res_multiplier: f32,
        amplifier_multiplier: f32
    ) -> f32 {
        (((total_scaling_stat * motion_value) * base_dmg_multiplier) + additive_base_dmg_bonus)
            * avg_crit_multiplier
            * (1.0 + total_dmg_bonus - dmg_reduction_target)
            * def_multiplier
            * res_multiplier
            * amplifier_multiplier
            * instances
    }

    pub fn calculate_damage(
        element: Element,
        damage_type: DamageType,
        scaling: BaseScaling,
        amplifier: Amplifier,
        instances: f32,
        motion_value: f32,
        character: &dyn Statable,
        buffs: Option<&StatTable>
    ) -> f32 {
        if amplifier == Amplifier::Forward || amplifier == Amplifier::Reverse {
            assert!(element == Element::Pyro || element == Element::Hydro || element == Element::Cryo || element == Element::Anemo);
        }

        let mut total = StatTable::new();
        total.add_table(character.iter());
        if let Some(buffs) = buffs {
            total.add_table(buffs.iter());
        }
        
        let total_base_scaling_stat = match scaling {
            BaseScaling::ATK => total_atk(&total),
            BaseScaling::DEF => total_def(&total),
            BaseScaling::HP => total_hp(&total),
        };

        let amplifier_multiplier = match amplifier {
            Amplifier::Forward => amplifier_multiplier(2.0, total.get(&Stat::ElementalMastery), total.get(&Stat::ReactionBonus)),
            Amplifier::Reverse => amplifier_multiplier(1.5, total.get(&Stat::ElementalMastery), total.get(&Stat::ReactionBonus)),
            Amplifier::None => 1.0,
        };

        let element_dmg_bonus = match element {
            Element::Pyro => total.get(&Stat::PyroDMGBonus),
            Element::Hydro => total.get(&Stat::HydroDMGBonus),
            Element::Electro => total.get(&Stat::ElectroDMGBonus),
            Element::Anemo => total.get(&Stat::AnemoDMGBonus),
            Element::Geo => total.get(&Stat::GeoDMGBonus),
            Element::Dendro => total.get(&Stat::DendroDMGBonus),
            Element::Cryo => total.get(&Stat::CryoDMGBonus),
            Element::Physical => total.get(&Stat::PhysicalDMGBonus),
            Element::None => 0.0,
        };

        let attack_type_dmg_bonus = match damage_type {
            DamageType::Normal => total.get(&Stat::NormalATKDMGBonus),
            DamageType::Charged => total.get(&Stat::ChargeATKDMGBonus),
            DamageType::Plunging => total.get(&Stat::PlungeATKDMGBonus),
            DamageType::Skill => total.get(&Stat::SkillDMGBonus),
            DamageType::Burst => total.get(&Stat::BurstDMGBonus),
            DamageType::None => 0.0,
        };

        let total_dmg_bonus = total.get(&Stat::DMGBonus) 
            + total.get(&Stat::ElementalDMGBonus)
            + element_dmg_bonus
            + attack_type_dmg_bonus;

        let def_reduction = total.get(&Stat::DefReduction);
        let def_ignore = total.get(&Stat::DefIgnore);
        
        let resistance_reduction = match element {
            Element::Pyro => total.get(&Stat::PyroResistanceReduction),
            Element::Hydro => total.get(&Stat::HydroResistanceReduction),
            Element::Electro => total.get(&Stat::ElectroResistanceReduction),
            Element::Anemo => total.get(&Stat::AnemoResistanceReduction),
            Element::Geo => total.get(&Stat::GeoResistanceReduction),
            Element::Dendro => total.get(&Stat::DendroResistanceReduction),
            Element::Cryo => total.get(&Stat::CryoResistanceReduction),
            Element::Physical => total.get(&Stat::PhysicalResistanceReduction),
            Element::None => 0.0,
        };

        full_damage_formula(
            instances,
            total_base_scaling_stat,
            motion_value,
            1.0,
            0.0,
            avg_crit_multiplier(&total),
            total_dmg_bonus,
            0.0,
            def_multiplier(90, 100, def_reduction, def_ignore),
            res_multiplier(0.1, resistance_reduction), // Assuming KQMC enemy with 10% base resistance
            amplifier_multiplier
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::model::stattable::StatTable;
        use crate::utils::testingutils::*;
        use crate::utils::testingutils::*;
        #[test] fn atk_fomrula() {
            let base = 42.0;
            let flat = 100.0;
            let percentage = 1.2;
            let expected: f32 = base * (1.0+percentage) + flat;
            let actual: f32 = total_atk(
                &StatTable::of(&[
                    (Stat::BaseATK, base),
                    (Stat::ATKPercent,percentage),
                    (Stat::FlatATK, flat),
                ])
            );
            debug_assert_eq!(expected, actual);
        }

        #[test]
        fn test_avg_crit_multiplier() {
            //edge cases
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, 0.0), (Stat::CritDMG, 0.0)])), 1.0); //0 cr, cd does not matter
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, 0.0), (Stat::CritDMG, 1.0)])), 1.0); //0 cr with some cd
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, -1.2), (Stat::CritDMG, 1.0)])), 1.0); //negative cr does not matter
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, 1.2), (Stat::CritDMG, 1.0)])), 2.0); //negative cr does not matter
    
            //normal cases
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, 1.0), (Stat::CritDMG, 1.0)])), 2.0);
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, 0.5), (Stat::CritDMG, 1.0)])), 1.5);
            assert_eq!(avg_crit_multiplier(&StatTable::of(&[(Stat::CritRate, 0.7), (Stat::CritDMG, 1.2)])), 1.84);
        }
    
        #[test]
        fn test_def_multiplier() {
            let enemy_level = 100;
            let def_reduction = 0.0;
            let def_ignore = 0.0;
            assert!((def_multiplier(90, enemy_level, def_reduction, def_ignore) - 0.487179487179487).abs() < 0.0001);
            assert!((def_multiplier(90, enemy_level, 0.5, 0.0) - 0.655172413793103).abs() < 0.0001);
            assert!((def_multiplier(90, enemy_level, 0.5, 0.5) - 0.791666667).abs() < 0.0001);
            assert!((def_multiplier(90, enemy_level, 0.9, 0.0) - 0.904761905).abs() < 0.0001);
            assert!((def_multiplier(90, enemy_level, 100000.0, 0.0) - 0.904761905).abs() < 0.0001);
        }
    
        #[test]
        fn test_res_multiplier() {
            let enemy_base_resistance = 0.1;
            assert!((res_multiplier(enemy_base_resistance, 0.0) - 0.9).abs() < 0.0001);
            assert!((res_multiplier(enemy_base_resistance, 0.2) - 1.05).abs() < 0.0001);
            assert!((res_multiplier(enemy_base_resistance, 0.4) - 1.15).abs() < 0.0001);
        }
    }
}



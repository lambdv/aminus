use crate::model::stattable::*;
use crate::model::stat::*;
use crate::model::statable::*;
use crate::functions::formulas::*;

/// higher level functions to compute damage from statables
pub struct DMGFunction {}
impl DMGFunction {

    /// higher level function that applys the full damage formula to a statable
    pub fn calculate_damage(
        element: Element,
        damage_type: DamageType,
        scaling: BaseScaling,
        amplifier: Amplifier,
        instances: f32,
        motion_value: f32,
        character: Box<&dyn Statable>,
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
            BaseScaling::ATK => formulas::total_atk(&total),
            BaseScaling::DEF => formulas::total_def(&total),
            BaseScaling::HP => formulas::total_hp(&total),
        };

        let amplifier_multiplier = match amplifier {
            Amplifier::Forward => formulas::amplifier_multiplier(2.0, total.get(&Stat::ElementalMastery), total.get(&Stat::ReactionBonus)),
            Amplifier::Reverse => formulas::amplifier_multiplier(1.5, total.get(&Stat::ElementalMastery), total.get(&Stat::ReactionBonus)),
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

        formulas::full_damage_formula(
            instances,
            total_base_scaling_stat,
            motion_value,
            1.0,
            0.0,
            formulas::avg_crit_multiplier(&total),
            total_dmg_bonus,
            0.0,
            formulas::def_multiplier(90, 100, def_reduction, def_ignore),
            formulas::res_multiplier(0.1, resistance_reduction), // Assuming KQMC enemy with 10% base resistance
            amplifier_multiplier
        )
    }

}

use crate::energy::specification::*;

/// most basic abstraction to calculate total energy stored by a character from particles and orbs
pub fn calculate_energy(
    num_party_memebers: i8,
    energy_recharge: f32,

    //speciify energy and orbs pool consumed by character
    same_particals_caught: f32,
    different_particals_caught: f32,
    white_particals_caught: f32,

    same_particals_not_caught: f32,
    different_particals_not_caught: f32,
    white_particals_not_caught: f32,

    same_orbs_caught: f32,
    different_orbs_caught: f32,
    white_orbs_caught: f32,

    same_orbs_not_caught: f32,
    different_orbs_not_caught: f32,
    white_orbs_not_caught: f32

) -> f32 {
    assert!(num_party_memebers > 0 || num_party_memebers <= 4);

    let off_field_multiplier = match num_party_memebers {
        4 => OFF_FIELD_4_PARTY_MULTIPLIER,
        3 => OFF_FIELD_3_PARTY_MULTIPLIER,
        2 => OFF_FIELD_2_PARTY_MULTIPLIER,
        _ => 0.,
    };

    let mut total_energy = 0.;
    total_energy += same_particals_caught * SAME_ELEMENT as f32;
    total_energy += different_particals_caught * DIFFERENT_ELEMENT as f32;
    total_energy += white_particals_caught * NO_ELEMENT as f32;

    total_energy += same_particals_not_caught * SAME_ELEMENT as f32 * off_field_multiplier;
    total_energy += different_particals_not_caught * DIFFERENT_ELEMENT as f32 * off_field_multiplier;
    total_energy += white_particals_not_caught * NO_ELEMENT as f32 * off_field_multiplier;

    total_energy += same_orbs_caught * SAME_ELEMENT as f32 * ORB_MULTIPLIER as f32;
    total_energy += different_orbs_caught * DIFFERENT_ELEMENT as f32 * ORB_MULTIPLIER as f32;
    total_energy += white_orbs_caught * NO_ELEMENT as f32 * ORB_MULTIPLIER as f32;

    total_energy += same_orbs_not_caught * SAME_ELEMENT as f32 * off_field_multiplier * ORB_MULTIPLIER as f32;
    total_energy += different_orbs_not_caught * DIFFERENT_ELEMENT as f32 * off_field_multiplier * ORB_MULTIPLIER as f32;
    total_energy += white_orbs_not_caught * NO_ELEMENT as f32 * off_field_multiplier * ORB_MULTIPLIER as f32;

    total_energy *= energy_recharge;
    
    total_energy
}


#[cfg(test)] mod tests {
    use super::*;

    #[test] fn test_calculate_energy_no_particles_or_orbs() {
        let energy = calculate_energy(
            4, 1.0,
            0., 0., 0.,
            0., 0., 0.,
            0., 0., 0.,
            0., 0., 0.,
        );
        //println!("{:?}", energy)
        assert!(energy==0.)
    }

    #[test] fn calculate_energy_recharge_requirements_from_calculate_energy_function() {
        let energy = calculate_energy(
            4, 1.0,
            (3. + 3.), 0., 0.,
            3., (6.+4.+4.), 4.,
            //ignore orbs, these come from enemies
            0., 0., 0.,
            0., 0., 0.,
        );

        let burst_cost = 80.;
        //let energy_needed_to_burst = energy - burst_cost;
        let energy_recharge_requirements = burst_cost/energy ;
        println!("{:?}", energy_recharge_requirements)
        //assert!(energy==0.)
    }
}
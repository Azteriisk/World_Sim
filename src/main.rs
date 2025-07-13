mod earth {
    pub mod inner_core;
    pub mod outer_core;
    pub mod mantle;
    pub mod crust;
}

use earth::inner_core::InnerCore;
use earth::outer_core::OuterCore;
use earth::mantle::{Mantle, Plate, HotSpot};

fn main() {
    let mut core = InnerCore::new();
    let mut outer = OuterCore::new(&core);

    let mut mantle = Mantle::new();

    mantle.plates.push(Plate {
        area_km2: 100_000_000.0,
        age_myr: 50.0,
        velocity_cm_per_year: 5.0,
        motion_direction_deg: 90.0,
        is_subducting: false,
        is_transform_boundary: true,
        shear_stress_mpa: 0.0,
        has_back_arc_spreading: true,
        back_arc_spreading_rate_cm_per_year: 2.0,
        volcanic_activity_factor: 0.7,
    });

    mantle.plates.push(Plate {
        area_km2: 60_000_000.0,
        age_myr: 10.0,
        velocity_cm_per_year: 3.0,
        motion_direction_deg: 45.0,
        is_subducting: false,
        is_transform_boundary: false,
        shear_stress_mpa: 0.0,
        has_back_arc_spreading: false,
        back_arc_spreading_rate_cm_per_year: 0.0,
        volcanic_activity_factor: 0.3,
    });

    mantle.hot_spots.push(HotSpot {
        lat_deg: 0.0,
        lon_deg: -155.0,
        surface_age_myr: 1.0,
    });

    mantle.hot_spots.push(HotSpot {
        lat_deg: 20.0,
        lon_deg: 120.0,
        surface_age_myr: 0.5,
    });

    println!("=== Initial States ===");
    core.describe();
    outer.describe();
    mantle.describe();

    let sim_years = 10_000_000.0;
    println!("\n=== Simulating {:.0} years ===", sim_years);

    core.update_crystallization(sim_years);
    outer.update_dynamics(&core, sim_years);
    mantle.update_advanced_dynamics(sim_years);
    mantle.d_prime_prime.transfer_heat(outer.heat_flux_to_mantle_mw_per_m2, 0.02);

    println!("\n=== States After Simulation ===");
    core.describe();
    outer.describe();
    mantle.describe();

    println!("\n=== Plate Motions and Dynamics ===");
    for (i, plate) in mantle.plates.iter().enumerate() {
        println!(
            "Plate {}: Area {:.1} km², Age {:.1} Myr, Velocity {:.2} cm/yr, Direction {:.1}°, Subducting: {}, Transform: {}, Back-arc: {}, Volcanic factor: {:.2}",
            i + 1,
            plate.area_km2,
            plate.age_myr,
            plate.velocity_cm_per_year,
            plate.motion_direction_deg,
            plate.is_subducting,
            plate.is_transform_boundary,
            plate.has_back_arc_spreading,
            plate.volcanic_activity_factor
        );
    }

    println!("\n=== Hot Spot Positions ===");
    for (i, hs) in mantle.hot_spots.iter().enumerate() {
        println!(
            "Hot Spot {}: Lat {:.1}°, Lon {:.1}°, Surface Age {:.1} Myr",
            i + 1,
            hs.lat_deg,
            hs.lon_deg,
            hs.surface_age_myr
        );
    }
}

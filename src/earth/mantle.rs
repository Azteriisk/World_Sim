use crate::earth::crust::Crust;

pub struct Plate {
    pub area_km2: f32,
    pub age_myr: f32,
    pub velocity_cm_per_year: f32,
    pub motion_direction_deg: f32,
    pub is_subducting: bool,
    pub is_transform_boundary: bool,
    pub shear_stress_mpa: f32,
    pub has_back_arc_spreading: bool,
    pub back_arc_spreading_rate_cm_per_year: f32,
    pub volcanic_activity_factor: f32,
}

impl Plate {
    pub fn update_motion(&mut self, mantle_flow_rate: f32, mantle_direction: f32) -> (bool, f32, f32, bool) {
        self.velocity_cm_per_year = mantle_flow_rate + (self.velocity_cm_per_year * 0.1);
        self.motion_direction_deg = (self.motion_direction_deg + mantle_direction * 0.05) % 360.0;
        self.age_myr += 0.1;

        let mut started_subduction = false;
        if self.age_myr > 100.0 && !self.is_subducting {
            self.is_subducting = true;
            started_subduction = true;
        }

        let mut spread_area = 0.0;
        if self.age_myr < 20.0 {
            spread_area = self.spread(10_000.0);
        }

        let earthquake_occurred = self.simulate_transform();
        let back_arc_area = self.simulate_back_arc_spreading();

        (started_subduction, spread_area, back_arc_area, earthquake_occurred)
    }

    pub fn subduct(&mut self, mantle: &mut LowerMantle) -> f32 {
        if self.is_subducting {
            let recycled_volume = self.area_km2 * 0.01;
            mantle.mix_composition(recycled_volume);
            self.area_km2 *= 0.99;
            recycled_volume
        } else {
            0.0
        }
    }

    pub fn spread(&mut self, growth_rate_km2: f32) -> f32 {
        self.area_km2 += growth_rate_km2;
        self.age_myr = 0.0;
        growth_rate_km2
    }

    pub fn simulate_transform(&mut self) -> bool {
        if self.is_transform_boundary {
            self.shear_stress_mpa += 5.0;
            if self.shear_stress_mpa > 300.0 {
                self.shear_stress_mpa = 0.0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn simulate_back_arc_spreading(&mut self) -> f32 {
        if self.has_back_arc_spreading {
            let added_area = self.back_arc_spreading_rate_cm_per_year * 1000.0;
            self.area_km2 += added_area;
            added_area
        } else {
            0.0
        }
    }
}

pub struct HotSpot {
    pub lat_deg: f32,
    pub lon_deg: f32,
    pub surface_age_myr: f32,
}

impl HotSpot {
    pub fn migrate(&mut self, mantle_flow_rate: f32, flow_direction_deg: f32) {
        let rad = flow_direction_deg.to_radians();
        self.lat_deg += (mantle_flow_rate / 10.0) * rad.sin();
        self.lon_deg += (mantle_flow_rate / 10.0) * rad.cos();

        if self.lat_deg > 90.0 { self.lat_deg = 90.0; }
        if self.lat_deg < -90.0 { self.lat_deg = -90.0; }
        if self.lon_deg > 180.0 { self.lon_deg -= 360.0; }
        if self.lon_deg < -180.0 { self.lon_deg += 360.0; }

        self.surface_age_myr += 0.1;
    }
}

pub struct Lithosphere {
    pub thickness_km: f32,
    pub temperature_c: f32,
    pub viscosity: f32,
    pub rigidity_factor: f32,
    pub heat_flux_in: f32,
    pub heat_flux_out: f32,
    pub composition: String,
    pub tectonic_stress_mpa: f32,
}

pub struct Asthenosphere {
    pub thickness_km: f32,
    pub temperature_c: f32,
    pub viscosity: f32,
    pub partial_melt_pct: f32,
    pub lubrication_factor: f32,
    pub heat_flux_in: f32,
    pub heat_flux_out: f32,
    pub composition: String,
    pub volatile_content_pct: f32,
    pub lateral_flow_rate_cm_per_year: f32,
    pub dominant_flow_direction_deg: f32,
}

impl Asthenosphere {
    pub fn update_lateral_flow(&mut self, core_influence: f32) {
        self.lateral_flow_rate_cm_per_year = (core_influence * 10.0).min(20.0);
        self.dominant_flow_direction_deg = (self.dominant_flow_direction_deg + core_influence * 5.0) % 360.0;
    }
}

pub struct TransitionZone {
    pub thickness_km: f32,
    pub temperature_c: f32,
    pub viscosity: f32,
    pub phase_change_depth_km: f32,
    pub heat_flux_in: f32,
    pub heat_flux_out: f32,
    pub composition: String,
    pub water_storage_capacity: f32,
}

pub struct LowerMantle {
    pub thickness_km: f32,
    pub temperature_c: f32,
    pub viscosity: f32,
    pub deep_convection_strength: f32,
    pub heat_flux_in: f32,
    pub heat_flux_out: f32,
    pub composition: String,
    pub stored_slab_volume_km3: f32,
    pub lateral_flow_rate_cm_per_year: f32,
    pub dominant_flow_direction_deg: f32,
}

impl LowerMantle {
    pub fn mix_composition(&mut self, slab_input_km3: f32) {
        self.stored_slab_volume_km3 += slab_input_km3;
        self.deep_convection_strength += 0.01;
        if self.deep_convection_strength > 1.0 {
            self.deep_convection_strength = 1.0;
        }
    }

    pub fn update_lateral_flow(&mut self) {
        self.lateral_flow_rate_cm_per_year = (self.deep_convection_strength * 5.0).min(10.0);
        self.dominant_flow_direction_deg = (self.dominant_flow_direction_deg + self.deep_convection_strength * 3.0) % 360.0;
    }
}

pub struct DPrimePrimeLayer {
    pub thickness_km: f32,
    pub temperature_c: f32,
    pub viscosity: f32,
    pub plume_generation_potential: f32,
    pub hot_spot_count: u32,
    pub heat_flux_in: f32,
    pub heat_flux_out: f32,
    pub composition: String,
    pub chemical_heterogeneity_factor: f32,
    pub temporal_variability: f32,
}

impl DPrimePrimeLayer {
    pub fn transfer_heat(&mut self, incoming_flux: f32, outgoing_flux: f32) {
        self.heat_flux_in = incoming_flux;
        self.heat_flux_out = outgoing_flux;
        self.temperature_c += (incoming_flux - outgoing_flux) * 8.0;
    }
}

pub struct Mantle {
    pub lithosphere: Lithosphere,
    pub asthenosphere: Asthenosphere,
    pub transition_zone: TransitionZone,
    pub lower_mantle: LowerMantle,
    pub d_prime_prime: DPrimePrimeLayer,
    pub plates: Vec<Plate>,
    pub hot_spots: Vec<HotSpot>,
    pub crust: Crust,
}

impl Mantle {
    pub fn new() -> Self {
        Self {
            lithosphere: Lithosphere {
                thickness_km: 100.0,
                temperature_c: 500.0,
                viscosity: 1e22,
                rigidity_factor: 0.9,
                heat_flux_in: 0.05,
                heat_flux_out: 0.04,
                composition: "Peridotite-rich upper mantle".to_string(),
                tectonic_stress_mpa: 50.0,
            },
            asthenosphere: Asthenosphere {
                thickness_km: 600.0,
                temperature_c: 1300.0,
                viscosity: 1e19,
                partial_melt_pct: 2.0,
                lubrication_factor: 0.8,
                heat_flux_in: 0.04,
                heat_flux_out: 0.03,
                composition: "Partially molten peridotite".to_string(),
                volatile_content_pct: 0.5,
                lateral_flow_rate_cm_per_year: 5.0,
                dominant_flow_direction_deg: 90.0,
            },
            transition_zone: TransitionZone {
                thickness_km: 250.0,
                temperature_c: 1600.0,
                viscosity: 1e21,
                phase_change_depth_km: 410.0,
                heat_flux_in: 0.03,
                heat_flux_out: 0.02,
                composition: "High-pressure mineral phases".to_string(),
                water_storage_capacity: 1.0,
            },
            lower_mantle: LowerMantle {
                thickness_km: 2200.0,
                temperature_c: 2500.0,
                viscosity: 1e23,
                deep_convection_strength: 0.6,
                heat_flux_in: 0.02,
                heat_flux_out: 0.015,
                composition: "Bridgmanite and ferropericlase".to_string(),
                stored_slab_volume_km3: 0.0,
                lateral_flow_rate_cm_per_year: 2.0,
                dominant_flow_direction_deg: 60.0,
            },
            d_prime_prime: DPrimePrimeLayer {
                thickness_km: 200.0,
                temperature_c: 3000.0,
                viscosity: 1e22,
                plume_generation_potential: 0.7,
                hot_spot_count: 1,
                heat_flux_in: 0.015,
                heat_flux_out: 0.02,
                composition: "ULVZ mixture".to_string(),
                chemical_heterogeneity_factor: 0.3,
                temporal_variability: 0.5,
            },
            plates: Vec::new(),
            hot_spots: Vec::new(),
            crust: Crust::new("continental"),
        }
    }

    pub fn describe(&self) {
        println!("Mantle description:");
        println!("  Lithosphere: Temp {:.0}°C, Stress {:.1} MPa, Composition: {}", self.lithosphere.temperature_c, self.lithosphere.tectonic_stress_mpa, self.lithosphere.composition);
        println!("  Asthenosphere: Temp {:.0}°C, Melt {:.1}%, Volatiles {:.1}%, Composition: {}", self.asthenosphere.temperature_c, self.asthenosphere.partial_melt_pct, self.asthenosphere.volatile_content_pct, self.asthenosphere.composition);
        println!("  Transition Zone: Temp {:.0}°C, Phase depth {:.0} km, Water capacity {:.1}", self.transition_zone.temperature_c, self.transition_zone.phase_change_depth_km, self.transition_zone.water_storage_capacity);
        println!("  Lower Mantle: Temp {:.0}°C, Slab vol {:.1} km³, Composition: {}", self.lower_mantle.temperature_c, self.lower_mantle.stored_slab_volume_km3, self.lower_mantle.composition);
        println!("  D'' Layer: Temp {:.0}°C, Hot spots {}, Composition: {}", self.d_prime_prime.temperature_c, self.d_prime_prime.hot_spot_count, self.d_prime_prime.composition);
        self.crust.describe();
    }

    pub fn update_advanced_dynamics(&mut self, years: f32) {
        self.lower_mantle.update_lateral_flow();
        self.asthenosphere.update_lateral_flow(self.lower_mantle.deep_convection_strength);

        // Track cumulative changes for consolidated output
        let mut total_volcanic_growth = 0.0;
        let mut total_tectonic_deformation = 0.0;
        let mut total_subduction_volume = 0.0;
        let mut total_spread_area = 0.0;
        let mut total_back_arc_area = 0.0;
        let mut subduction_events = 0;
        let mut earthquake_events = 0;

        for plate in &mut self.plates {
            let (started_subduction, spread_area, back_arc_area, earthquake_occurred) = 
                plate.update_motion(self.asthenosphere.lateral_flow_rate_cm_per_year, self.asthenosphere.dominant_flow_direction_deg);
            
            let subduction_volume = plate.subduct(&mut self.lower_mantle);

            if started_subduction {
                subduction_events += 1;
            }
            if earthquake_occurred {
                earthquake_events += 1;
            }

            total_subduction_volume += subduction_volume;
            total_spread_area += spread_area;
            total_back_arc_area += back_arc_area;

            if plate.volcanic_activity_factor > 0.5 {
                total_volcanic_growth += self.crust.grow_by_volcanism(500.0);
            }
            total_tectonic_deformation += self.crust.update_tectonics(plate.velocity_cm_per_year);
        }

        let erosion_km = self.crust.erode(years);

        // Only show significant changes
        if subduction_events > 0 {
            println!("⚠️ {} plate(s) started subduction.", subduction_events);
        }
        if earthquake_events > 0 {
            println!("⚡ {} transform fault slip event(s) occurred!", earthquake_events);
        }
        if total_spread_area > 1000.0 {
            println!("🌋 Plate spreading: gained {:.1} km² new area.", total_spread_area);
        }
        if total_back_arc_area > 100.0 {
            println!("🌊 Back-arc spreading: plate area increased by {:.1} km².", total_back_arc_area);
        }
        if total_subduction_volume > 1000.0 {
            println!("🌊 Plate subduction: recycling {:.1} km³ into mantle.", total_subduction_volume);
        }
        if total_volcanic_growth > 0.001 {
            println!("🌋 Crust growth: added {:.3} km from volcanism.", total_volcanic_growth);
        }
        if total_tectonic_deformation.abs() > 0.01 {
            println!("⛰️ Crust tectonic adjustment: thickness changed by {:.3} km.", total_tectonic_deformation);
        }
        if erosion_km > 0.1 {
            println!("🌊 Crust erosion: reduced thickness by {:.3} km over {:.0} years.", erosion_km, years);
        }

        for hot_spot in &mut self.hot_spots {
            hot_spot.migrate(self.asthenosphere.lateral_flow_rate_cm_per_year, self.asthenosphere.dominant_flow_direction_deg);
        }
    }
}

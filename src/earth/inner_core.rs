pub struct InnerCore {
    pub radius_km: f32,
    pub temperature_c: f32,
    pub pressure_gpa: f32,
    pub iron_pct: f32,
    pub nickel_pct: f32,
    pub other_elements_pct: f32,
    pub heat_flux_mw_per_m2: f32,
    pub crystallization_rate_mm_per_year: f32,
    pub mass_kg: f64,
    pub age_myr: f32,
    pub rotation_offset_deg_per_year: f32,
    pub magnetic_contribution_factor: f32,
    pub crystal_anisotropy_factor: f32,
    pub latent_heat_release_tj_per_year: f32,
    pub asymmetric_growth_factor: f32,
}

impl InnerCore {
    pub fn new() -> Self {
        Self {
            radius_km: 1221.0,
            temperature_c: 5400.0,
            pressure_gpa: 330.0,
            iron_pct: 80.0,
            nickel_pct: 20.0,
            other_elements_pct: 0.0,
            heat_flux_mw_per_m2: 0.05,
            crystallization_rate_mm_per_year: 1.0,
            mass_kg: 9.7e22,
            age_myr: 1000.0,
            rotation_offset_deg_per_year: 0.1,
            magnetic_contribution_factor: 0.9,
            crystal_anisotropy_factor: 0.5,
            latent_heat_release_tj_per_year: 50.0,
            asymmetric_growth_factor: 0.1,
        }
    }

    pub fn update_crystallization(&mut self, years: f32) {
        self.age_myr += years / 1_000_000.0;

        let growth_km = (self.crystallization_rate_mm_per_year * years) / 1_000_000.0;
        self.radius_km += growth_km;

        self.update_mass();
        self.update_heat_flux();

        self.latent_heat_release_tj_per_year = self.crystallization_rate_mm_per_year * 50.0;
    }

    fn update_mass(&mut self) {
        let radius_m = self.radius_km * 1_000.0;
        let volume_m3 = (4.0 / 3.0) * std::f32::consts::PI * radius_m.powi(3);
        let density = 12_800.0;

        self.mass_kg = (volume_m3 as f64) * (density as f64);
    }

    fn update_heat_flux(&mut self) {
        let radius_m = self.radius_km * 1_000.0;
        let area_m2 = 4.0 * std::f32::consts::PI * radius_m.powi(2);

        let outer_core_temp_c = 4300.0;
        let delta_t = self.temperature_c - outer_core_temp_c;

        let k = 1e6;
        self.heat_flux_mw_per_m2 = (k * delta_t / area_m2) * 1e-6;
    }

    pub fn update_anisotropy(&mut self, delta: f32) {
        self.crystal_anisotropy_factor = (self.crystal_anisotropy_factor + delta).clamp(0.0, 1.0);
    }

    pub fn adjust_asymmetry(&mut self, delta: f32) {
        self.asymmetric_growth_factor = (self.asymmetric_growth_factor + delta).clamp(0.0, 1.0);
    }

    pub fn cumulative_rotation(&self, years: f32) -> f32 {
        self.rotation_offset_deg_per_year * years
    }

    pub fn adjust_magnetic_contribution(&mut self, delta: f32) {
        self.magnetic_contribution_factor = (self.magnetic_contribution_factor + delta).clamp(0.0, 1.0);
    }

    pub fn reset_magnetic_contribution(&mut self, new_value: f32) {
        self.magnetic_contribution_factor = new_value.clamp(0.0, 1.0);
    }

    pub fn describe(&self) {
        println!("Inner Core:");
        println!("  Radius: {:.1} km", self.radius_km);
        println!("  Temperature: {:.1} °C", self.temperature_c);
        println!("  Pressure: {:.1} GPa", self.pressure_gpa);
        println!("  Composition: {:.1}% iron, {:.1}% nickel, {:.1}% other", self.iron_pct, self.nickel_pct, self.other_elements_pct);
        println!("  Heat flux: {:.3} MW/m²", self.heat_flux_mw_per_m2);
        println!("  Crystallization rate: {:.2} mm/year", self.crystallization_rate_mm_per_year);
        println!("  Mass: {:.2e} kg", self.mass_kg);
        println!("  Age: {:.1} million years", self.age_myr);
        println!("  Rotation offset: {:.3} deg/year", self.rotation_offset_deg_per_year);
        println!("  Magnetic contribution: {:.2}", self.magnetic_contribution_factor);
        println!("  Crystal anisotropy: {:.2}", self.crystal_anisotropy_factor);
        println!("  Latent heat release: {:.1} TJ/year", self.latent_heat_release_tj_per_year);
        println!("  Asymmetric growth factor: {:.2}", self.asymmetric_growth_factor);
    }
}

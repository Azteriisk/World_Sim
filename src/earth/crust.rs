pub struct Crust {
    pub thickness_km: f32,
    pub type_name: String,
    pub age_myr: f32,
    pub composition: String,
    pub average_density: f32,
    pub heat_flux_out: f32,
    pub tectonic_activity_factor: f32,
    pub volcanic_activity_factor: f32,
    pub erosion_rate_mm_per_yr: f32,
    pub is_active_margin: bool,

    // New fields for future simulation depth
    pub mineral_distribution: String,
    pub sediment_thickness_km: f32,
    pub surface_temperature_c: f32,
    pub surface_albedo: f32,
    pub groundwater_content_pct: f32,
    pub isostasy_adjustment_km: f32,
    pub vegetation_coverage_pct: f32,
    pub surface_roughness_factor: f32,
}

impl Crust {
    pub fn new(type_name: &str) -> Self {
        match type_name {
            "continental" => Self {
                thickness_km: 35.0,
                type_name: "continental".to_string(),
                age_myr: 1000.0,
                composition: "granite-dominated".to_string(),
                average_density: 2700.0,
                heat_flux_out: 0.06,
                tectonic_activity_factor: 0.5,
                volcanic_activity_factor: 0.2,
                erosion_rate_mm_per_yr: 0.1,
                is_active_margin: false,
                mineral_distribution: "silicates, minor iron".to_string(),
                sediment_thickness_km: 2.0,
                surface_temperature_c: 15.0,
                surface_albedo: 0.3,
                groundwater_content_pct: 5.0,
                isostasy_adjustment_km: 0.0,
                vegetation_coverage_pct: 50.0,
                surface_roughness_factor: 0.5,
            },
            "oceanic" => Self {
                thickness_km: 7.0,
                type_name: "oceanic".to_string(),
                age_myr: 50.0,
                composition: "basalt-dominated".to_string(),
                average_density: 2900.0,
                heat_flux_out: 0.08,
                tectonic_activity_factor: 0.7,
                volcanic_activity_factor: 0.6,
                erosion_rate_mm_per_yr: 0.5,
                is_active_margin: true,
                mineral_distribution: "basalts, sulfides".to_string(),
                sediment_thickness_km: 0.5,
                surface_temperature_c: 4.0,
                surface_albedo: 0.1,
                groundwater_content_pct: 1.0,
                isostasy_adjustment_km: 0.0,
                vegetation_coverage_pct: 0.0,
                surface_roughness_factor: 0.8,
            },
            _ => panic!("Invalid crust type! Use 'continental' or 'oceanic'."),
        }
    }

    pub fn grow_by_volcanism(&mut self, magma_supply_km3: f32) -> f32 {
        let added_thickness = magma_supply_km3 / 1_000_000.0;
        self.thickness_km += added_thickness;
        self.age_myr = 0.0;
        added_thickness
    }

    pub fn erode(&mut self, years: f32) -> f32 {
        let erosion_km = (self.erosion_rate_mm_per_yr * years) / 1_000_000.0;
        self.thickness_km -= erosion_km;
        if self.thickness_km < 5.0 {
            self.thickness_km = 5.0;
        }
        erosion_km
    }

    pub fn update_tectonics(&mut self, plate_motion_cm_per_year: f32) -> f32 {
        let deformation = plate_motion_cm_per_year / 100.0 * self.tectonic_activity_factor;
        self.thickness_km += deformation;
        if self.thickness_km > 70.0 {
            self.thickness_km = 70.0;
        }
        deformation
    }

    pub fn rejuvenate_subduction(&mut self) {
        self.age_myr = 0.0;
        self.thickness_km = 7.0;
        self.composition = "rejuvenated basaltic melt".to_string();
        println!("♻️ Crust rejuvenated via subduction or melting reset.");
    }

    pub fn adjust_isostasy(&mut self, load_change_km: f32) {
        self.isostasy_adjustment_km += load_change_km * 0.1;
        println!("⚖️ Isostasy adjusted by {:.3} km due to load change.", load_change_km * 0.1);
    }

    pub fn update_surface_temperature(&mut self, external_forcing: f32) {
        self.surface_temperature_c += external_forcing;
        println!("🌡️ Surface temperature adjusted by {:.2}°C.", external_forcing);
    }

    pub fn deposit_sediment(&mut self, thickness_km: f32) {
        self.sediment_thickness_km += thickness_km;
        println!("🏔️ Sediment deposition: added {:.3} km.", thickness_km);
    }

    pub fn simulate_erosion_feedback(&mut self) {
        if self.sediment_thickness_km > 5.0 {
            self.erosion_rate_mm_per_yr *= 1.2;
            println!("⚠️ Erosion rate increased due to high sediment cover.");
        }
    }

    pub fn update_vegetation(&mut self, change_pct: f32) {
        self.vegetation_coverage_pct += change_pct;
        if self.vegetation_coverage_pct > 100.0 {
            self.vegetation_coverage_pct = 100.0;
        }
        if self.vegetation_coverage_pct < 0.0 {
            self.vegetation_coverage_pct = 0.0;
        }
        println!("🌱 Vegetation coverage adjusted by {:.1}%.", change_pct);
    }

    pub fn describe(&self) {
        println!("Crust description:");
        println!("  Type: {}, Age: {:.1} Myr", self.type_name, self.age_myr);
        println!("  Thickness: {:.2} km, Composition: {}", self.thickness_km, self.composition);
        println!("  Density: {:.0} kg/m³, Heat flux: {:.3} MW/m²", self.average_density, self.heat_flux_out);
        println!("  Tectonic factor: {:.2}, Volcanic factor: {:.2}, Erosion rate: {:.2} mm/yr", self.tectonic_activity_factor, self.volcanic_activity_factor, self.erosion_rate_mm_per_yr);
        println!("  Mineral dist: {}, Sediment: {:.2} km", self.mineral_distribution, self.sediment_thickness_km);
        println!("  Surface temp: {:.1}°C, Albedo: {:.2}, Groundwater: {:.1}%", self.surface_temperature_c, self.surface_albedo, self.groundwater_content_pct);
        println!("  Isostasy adj: {:.3} km, Vegetation: {:.1}%, Roughness: {:.2}", self.isostasy_adjustment_km, self.vegetation_coverage_pct, self.surface_roughness_factor);
        println!("  Active margin: {}", self.is_active_margin);
    }
}

use crate::earth::inner_core::InnerCore;
use rand::random;

pub struct OuterCore {
    pub thickness_km: f32,
    pub temperature_c: f32,
    pub density: f32,
    pub iron_pct: f32,
    pub nickel_pct: f32,
    pub light_elements_pct: f32,
    pub light_elements_enrichment: f32,
    pub convection_strength: f32,
    pub magnetic_field_strength: f32,
    pub heat_flux_from_inner_mw_per_m2: f32,
    pub heat_flux_to_mantle_mw_per_m2: f32,
    pub total_time_myr: f32,
    pub magnetic_polarity_normal: bool,
    pub toroidal_flow_factor: f32,
    pub poloidal_flow_factor: f32,
    pub has_heat_anomaly: bool,
    pub cumulative_heat_to_mantle_tj: f64,
}

impl OuterCore {
    pub fn new(inner_core: &InnerCore) -> Self {
        let convection_strength = (inner_core.heat_flux_mw_per_m2 / 0.1).min(1.0);

        Self {
            thickness_km: 2200.0,
            temperature_c: 4500.0,
            density: 11000.0,
            iron_pct: 85.0,
            nickel_pct: 5.0,
            light_elements_pct: 10.0,
            light_elements_enrichment: 0.0,
            convection_strength,
            magnetic_field_strength: convection_strength * inner_core.magnetic_contribution_factor,
            heat_flux_from_inner_mw_per_m2: inner_core.heat_flux_mw_per_m2,
            heat_flux_to_mantle_mw_per_m2: 0.04,
            total_time_myr: 0.0,
            magnetic_polarity_normal: true,
            toroidal_flow_factor: convection_strength * 0.7,
            poloidal_flow_factor: convection_strength * 0.3,
            has_heat_anomaly: false,
            cumulative_heat_to_mantle_tj: 0.0,
        }
    }

    pub fn update_dynamics(&mut self, inner_core: &InnerCore, years: f32) {
        self.total_time_myr += years / 1_000_000.0;

        self.heat_flux_from_inner_mw_per_m2 = inner_core.heat_flux_mw_per_m2;
        self.convection_strength = (self.heat_flux_from_inner_mw_per_m2 / 0.1).min(1.0);

        let enrichment_delta = (inner_core.crystallization_rate_mm_per_year * years * 0.00001).min(1.0);
        self.enrich_light_elements(enrichment_delta);

        if self.light_elements_enrichment > 30.0 {
            self.convection_strength *= 0.95;
        }

        self.magnetic_field_strength = self.convection_strength * inner_core.magnetic_contribution_factor;

        self.toroidal_flow_factor = self.convection_strength * 0.7;
        self.poloidal_flow_factor = self.convection_strength * 0.3;

        self.maybe_trigger_reversal();
        self.maybe_generate_heat_anomaly();
        self.transfer_heat_to_mantle(years);
    }

    pub fn transfer_heat_to_mantle(&mut self, years: f32) {
        self.heat_flux_to_mantle_mw_per_m2 = self.convection_strength * 0.06;

        let heat_per_year_tj = (self.heat_flux_to_mantle_mw_per_m2 as f64 * 1e6 * 3.1536e7) / 1e12;
        self.cumulative_heat_to_mantle_tj += heat_per_year_tj * years as f64;
    }

    pub fn enrich_light_elements(&mut self, delta: f32) {
        self.light_elements_enrichment = (self.light_elements_enrichment + delta).clamp(0.0, 100.0);
    }

    pub fn maybe_trigger_reversal(&mut self) {
        if self.magnetic_field_strength < 0.3 && random::<f32>() < 0.05 {
            self.magnetic_polarity_normal = !self.magnetic_polarity_normal;
            println!("⚡ Magnetic field reversal occurred!");
        }
    }

    pub fn maybe_generate_heat_anomaly(&mut self) {
        if self.convection_strength > 0.8 && random::<f32>() < 0.1 {
            self.has_heat_anomaly = true;
            println!("🔥 Local heat anomaly generated at CMB!");
        }
    }

    pub fn describe(&self) {
        println!("Outer Core:");
        println!("  Thickness: {:.0} km", self.thickness_km);
        println!("  Temperature: {:.0} °C", self.temperature_c);
        println!("  Density: {:.0} kg/m³", self.density);
        println!("  Composition: {:.1}% iron, {:.1}% nickel, {:.1}% light elements", self.iron_pct, self.nickel_pct, self.light_elements_pct);
        println!("  Light element enrichment: {:.2}%", self.light_elements_enrichment);
        println!("  Convection strength: {:.2}", self.convection_strength);
        println!("  Magnetic field strength: {:.2}", self.magnetic_field_strength);
        println!("  Magnetic polarity normal: {}", self.magnetic_polarity_normal);
        println!("  Toroidal flow factor: {:.2}", self.toroidal_flow_factor);
        println!("  Poloidal flow factor: {:.2}", self.poloidal_flow_factor);
        println!("  Heat flux from inner core: {:.3} MW/m²", self.heat_flux_from_inner_mw_per_m2);
        println!("  Heat flux to mantle: {:.3} MW/m²", self.heat_flux_to_mantle_mw_per_m2);
        println!("  Cumulative heat to mantle: {:.2e} TJ", self.cumulative_heat_to_mantle_tj);
        println!("  Local heat anomaly: {}", self.has_heat_anomaly);
        println!("  Total simulated time: {:.2} million years", self.total_time_myr);
    }
}

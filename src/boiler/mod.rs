const CASING_THICKNESS: i64 = 1;
const WATER_TANK_VOLUME: i64 = 16_000;
const HEATER_HEAT_RATE: i64 = 320_000;
const STEAM_TANK_VOLUME: i64 = 160_000;

const COOLANT_COOLING_EFFICIENCY: f64 = 0.4;
const CASING_HEAT_CAPACITY: f64 = 50.0;
const HEATED_COOLANT_TEMP: f64 = 100_000.0;
const STEAM_ENERGY_EFFICIENCY: f64 = 0.2;
const WATER_THERMAL_ENTHALPY: f64 = 10.0;
const SODIUM_THERMAL_ENTHALPY: f64 = 5.0;
const BOLIER_WATER_CONDUCTIVITY: f64 = 0.7;

#[derive(Clone, Copy, Debug, Default)]
pub enum HeatingMode {
    #[default]
    DirectHeating,
    SodiumHeating,
}

#[derive(Clone, Default, Debug)]
pub struct BoilerLayout {
    length: i64,
    width: i64,
    height: i64,
    heating_mode: HeatingMode,
    heat_capacity: f64,
    spliter_layer: i64,
    heating_element: i64,
}
impl BoilerLayout {
    pub fn new(length: i64, width: i64, height: i64, heating_mode: HeatingMode) -> BoilerLayout {
        let heat_capacity = (length * width * height - (length - 2) * (width - 2) * (height - 2))
            as f64
            * CASING_HEAT_CAPACITY;
        BoilerLayout {
            length,
            width,
            height,
            heating_mode,
            heat_capacity,
            spliter_layer: 0,
            heating_element: 0,
        }
    }
    pub fn get_spliter_layer(&self) -> i64 {
        self.spliter_layer
    }
    pub fn get_heating_element(&self) -> i64 {
        self.heating_element
    }

    pub fn get_area(&self) -> i64 {
        self.length * self.width
    }
    pub fn get_water_tank(&self) -> i64 {
        let area = self.get_area();
        let spliter = self.spliter_layer;
        let heater = self.heating_element;
        ((spliter - 1) * area - heater) * WATER_TANK_VOLUME
    }
    pub fn get_heat_rate(&self) -> i64 {
        let heater = self.heating_element;
        heater * HEATER_HEAT_RATE
    }
    pub fn get_steam_tank(&self) -> i64 {
        let area = self.get_area();
        let height = self.height;
        let spliter = self.spliter_layer;
        (height - spliter) * area * STEAM_TANK_VOLUME
    }
    pub fn get_production(&self) -> i64 {
        min_tri(
            self.get_water_tank(),
            self.get_heat_rate(),
            self.get_steam_tank(),
        )
    }
    pub fn get_cooled_coolant_tank(&self) -> i64 {
        (self.get_steam_tank() as f64 * 1.6) as i64
    }
    pub fn get_hot_coolant_tank(&self) -> i64 {
        self.get_water_tank() * 16
    }
    fn get_max_temperature(&self) -> f64 {
        self.get_production() as f64 / self.heat_capacity / STEAM_ENERGY_EFFICIENCY
            * WATER_THERMAL_ENTHALPY
            / BOLIER_WATER_CONDUCTIVITY
    }
    fn get_max_heat_consumption(&self) -> f64 {
        self.get_production() as f64 * WATER_THERMAL_ENTHALPY / STEAM_ENERGY_EFFICIENCY
    }
    pub fn get_coolant_consumption(&self) -> i64 {
        min_tri(
            (self.get_hot_coolant_tank() as f64
                * COOLANT_COOLING_EFFICIENCY
                * (1.0 - self.get_max_temperature() / HEATED_COOLANT_TEMP)) as i64,
            self.get_cooled_coolant_tank(),
            (self.get_max_heat_consumption() / SODIUM_THERMAL_ENTHALPY) as i64,
        )
    }

    fn get_value(&self) -> i64 {
        match self.heating_mode {
            HeatingMode::DirectHeating => self.get_production(),
            HeatingMode::SodiumHeating => self.get_coolant_consumption(),
        }
    }
    fn get_max_spliter_layer(&self) -> i64 {
        self.height - CASING_THICKNESS
    }
    fn get_max_heating_element(&self) -> i64 {
        let inner_area = (self.length - CASING_THICKNESS) * (self.width - CASING_THICKNESS);
        let height = self.spliter_layer - CASING_THICKNESS;
        inner_area * height
    }
    fn calculate_layout(mut self) -> BoilerLayout {
        let mut max_value = 0;
        let mut best_layout = BoilerLayout::default();
        for i in CASING_THICKNESS..self.get_max_spliter_layer() {
            self.spliter_layer = i;
            for j in 0..self.get_max_heating_element() {
                self.heating_element = j;
                let current_value = self.get_value();
                // if j == 5 {
                //     println!("{}", current_value);
                //     println!("{:?}", self);
                //     println!("{}", self.get_max_temperature());
                //     println!("{:?}", best_layout);
                //     println!("{}", best_layout.get_max_temperature());
                //     panic!();
                // }

                if max_value < current_value {
                    best_layout = self.clone();
                    max_value = current_value;
                }
            }
        }
        best_layout
    }
}

fn min_tri(x: i64, y: i64, z: i64) -> i64 {
    std::cmp::min(std::cmp::min(x, y), z)
}

pub fn calculate_layout(
    length: i64,
    width: i64,
    height: i64,
    heating_mode: HeatingMode,
) -> BoilerLayout {
    BoilerLayout::new(length, width, height, heating_mode).calculate_layout()
}

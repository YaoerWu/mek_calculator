use std::fmt::{self, Display};

const LAYOUT_LENGTH: usize = 16;
const CASING_THICKNESS: i64 = 1;
const WATER_COOLING_RATE: f64 = 0.413425;
const SODIUM_COOLING_RATE: f64 = 0.82685;

#[derive(Clone, Copy, Debug, Default)]
pub enum CoolingMode {
    #[default]
    WaterCooling,
    SodiumCooling,
}

#[derive(Default, Debug)]
pub struct FissionFuelAssemblyLayout {
    length: i64,
    width: i64,
    height: i64,
    layout: [[i64; LAYOUT_LENGTH]; LAYOUT_LENGTH],
    cooling_mode: CoolingMode,
}
impl FissionFuelAssemblyLayout {
    fn new(
        reactor_length: i64,
        reactor_width: i64,
        reactor_height: i64,
        cooling_mode: CoolingMode,
    ) -> FissionFuelAssemblyLayout {
        let height = reactor_height - CASING_THICKNESS * 3;
        let length = reactor_length - CASING_THICKNESS * 2;
        let width = reactor_width - CASING_THICKNESS * 2;
        let mut layout = [[-1; LAYOUT_LENGTH]; LAYOUT_LENGTH];
        for i in 0..length {
            for j in 0..width {
                layout[i as usize][j as usize] = height;
            }
        }

        FissionFuelAssemblyLayout {
            length,
            width,
            height,
            layout,
            cooling_mode,
        }
    }

    fn get_valid_assembly(&self, postion: (i64, i64)) -> i64 {
        let (x, y) = postion;
        if x >= self.length || y >= self.width {
            return 0;
        }
        if x < 0 || y < 0 {
            return 0;
        }
        self.layout[x as usize][y as usize]
    }
    fn check_fuel_rod(&self, postion: (i64, i64)) -> bool {
        self.get_valid_assembly(postion) > 0
    }
    fn get_fuel_rod_value(&self, postion: (i64, i64)) -> i64 {
        if !self.check_fuel_rod(postion) {
            return 0;
        }
        let (x, y) = postion;
        let mut value = 0;
        if self.check_fuel_rod((x - 1, y)) {
            value += 1;
        }
        if self.check_fuel_rod((x + 1, y)) {
            value += 1;
        }
        if self.check_fuel_rod((x, y - 1)) {
            value += 1;
        }
        if self.check_fuel_rod((x, y + 1)) {
            value += 1;
        }
        value
    }
    fn remove_worst_assembly(&mut self) {
        let mut worst_x = 0;
        let mut worst_y = 0;
        let mut worst_value = 0;
        for x in 0..self.length {
            for y in 0..self.width {
                if !self.check_fuel_rod((x, y)) {
                    continue;
                }
                let contactsurface = self.get_fuel_rod_value((x, y));
                if worst_value < contactsurface {
                    worst_value = contactsurface;
                    worst_x = x;
                    worst_y = y;
                }
            }
        }
        if self.check_fuel_rod((worst_x, worst_y)) {
            self.layout[worst_x as usize][worst_y as usize] -= 1;
        } else {
            let mut exist_x = 0;
            let mut exist_y = 0;
            for x in 0..self.length {
                for y in 0..self.width {
                    if self.check_fuel_rod((x, y)) {
                        exist_x = x;
                        exist_y = y;
                    }
                }
            }
            self.layout[exist_x as usize][exist_y as usize] -= 1;
        }
    }

    fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }
    pub fn get_assembly_count(&self) -> i64 {
        let mut count = 0;
        for cow in self.layout.iter() {
            for i in cow.iter() {
                if *i > 0 {
                    count += *i;
                }
            }
        }
        count
    }

    fn get_fuel_rod_surface(&self, postion: (i64, i64)) -> i64 {
        if !self.check_fuel_rod(postion) {
            return 0;
        }
        let clamp = |x| {
            if x < 0 {
                0
            } else {
                x
            }
        };
        let self_count = self.get_valid_assembly(postion);
        let (x, y) = postion;
        let left = self_count - self.get_valid_assembly((x - 1, y));
        let right = self_count - self.get_valid_assembly((x + 1, y));
        let up = self_count - self.get_valid_assembly((x, y + 1));
        let down = self_count - self.get_valid_assembly((x, y - 1));
        clamp(left) + clamp(right) + clamp(up) + clamp(down) + 2
    }
    pub fn get_total_surface(&self) -> i64 {
        let mut count = 0;
        for x in 0..self.length {
            for y in 0..self.width {
                count += self.get_fuel_rod_surface((x, y));
            }
        }
        count
    }

    pub fn get_efficiency(&self) -> f64 {
        let assembly_count = self.get_assembly_count() as f64;
        let total_surface = self.get_total_surface() as f64;
        Self::min(1_f64, total_surface / assembly_count / 4.0)
    }
    pub fn get_max_speed(&self) -> f64 {
        let efficiency = self.get_efficiency();
        let volume = ((self.length + CASING_THICKNESS * 2)
            * (self.width + CASING_THICKNESS * 2)
            * (self.height + CASING_THICKNESS * 3)
            - self.length * self.width * (self.height + CASING_THICKNESS))
            as f64;
        let max_speed = volume
            * efficiency
            * (match self.cooling_mode {
                CoolingMode::WaterCooling => WATER_COOLING_RATE,
                CoolingMode::SodiumCooling => SODIUM_COOLING_RATE,
            });
        Self::min(max_speed, self.get_assembly_count() as f64)
    }
    fn check_reactors(&self) -> bool {
        self.get_assembly_count() as f64 - self.get_max_speed() < 1_f64
    }
}

impl Display for FissionFuelAssemblyLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let layout = self.layout;
        for col in layout.iter().enumerate() {
            let (i, col) = col;
            if col[0] < 0 {
                break;
            }
            write!(f, "col{: >2}: ", i)?;
            for i in col.iter() {
                if *i >= 0 {
                    write!(f, "{: >2} ", *i)?;
                } else {
                    break;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn calculate_layout(
    length: i64,
    width: i64,
    height: i64,
    cooling_mode: CoolingMode,
) -> FissionFuelAssemblyLayout {
    let mut layout = FissionFuelAssemblyLayout::new(length, width, height, cooling_mode);
    while !layout.check_reactors() {
        layout.remove_worst_assembly();
    }

    layout
}

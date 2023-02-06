pub mod boiler;
pub mod fission_reactor;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("reactor.csv")?;
    let bom = [0xEF, 0xBB, 0xBF];
    file.write_all(&bom)?;
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record(["长", "宽", "高", "水冷", "钠冷"])?;
    for height in 4..=18 {
        for width in 3..=height {
            for length in 3..=width {
                let water_layout = fission_reactor::calculate_layout(
                    length,
                    width,
                    height,
                    fission_reactor::CoolingMode::WaterCooling,
                );
                let sodium_layout = fission_reactor::calculate_layout(
                    length,
                    width,
                    height,
                    fission_reactor::CoolingMode::SodiumCooling,
                );
                wtr.write_record([
                    length.to_string(),
                    width.to_string(),
                    height.to_string(),
                    water_layout.get_max_speed().to_string(),
                    sodium_layout.get_max_speed().to_string(),
                ])?;
            }
        }
    }

    let mut file = File::create("boiler.csv")?;
    let bom = [0xEF, 0xBB, 0xBF];
    file.write_all(&bom)?;
    let mut wtr = csv::Writer::from_writer(file);

    wtr.write_record([
        "长",
        "宽",
        "高",
        "直接分压层",
        "直接加热元件",
        "直接蒸汽产出",
        "钠冷分压层",
        "钠冷加热元件",
        "钠冷蒸汽产出",
        "钠消耗",
    ])?;

    for height in 4..=18 {
        for width in 3..=height {
            for length in 3..=width {
                let direct_layout = boiler::calculate_layout(
                    length,
                    width,
                    height,
                    boiler::HeatingMode::DirectHeating,
                );
                let sodium_layout = boiler::calculate_layout(
                    length,
                    width,
                    height,
                    boiler::HeatingMode::SodiumHeating,
                );
                wtr.write_record([
                    length.to_string(),
                    width.to_string(),
                    height.to_string(),
                    direct_layout.get_spliter_layer().to_string(),
                    direct_layout.get_heating_element().to_string(),
                    direct_layout.get_production().to_string(),
                    sodium_layout.get_spliter_layer().to_string(),
                    sodium_layout.get_heating_element().to_string(),
                    sodium_layout.get_production().to_string(),
                    sodium_layout.get_coolant_consumption().to_string(),
                ])?;
            }
        }
    }
    Ok(())
}

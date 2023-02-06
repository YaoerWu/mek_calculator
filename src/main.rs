pub mod boiler;
pub mod fission_reactor;
use std::io;
use std::io::prelude::*;

use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

fn main() {
    let mut length;
    let mut width;
    let mut height;
    loop {
        length = Input::<String>::new()
            .with_prompt("输入长度")
            .interact_text()
            .unwrap()
            .parse()
            .unwrap();
        width = Input::<String>::new()
            .with_prompt("输入宽度")
            .interact_text()
            .unwrap()
            .parse()
            .unwrap();
        height = Input::<String>::new()
            .with_prompt("输入高度")
            .interact_text()
            .unwrap()
            .parse()
            .unwrap();

        if !check_attributes(length, width, height) {
            println!("指定的尺寸无法构成多方块结构");
        } else {
            break;
        }
    }

    let items = vec!["裂变反应堆", "锅炉"];
    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("选择计算器模式:")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();
    match user_selection {
        0 => {
            let items = vec!["水冷", "钠冷"];
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("选择反应堆冷却模式:")
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())
                .unwrap()
                .unwrap();
            let layout = fission_reactor::calculate_layout(
                length,
                width,
                height,
                match user_selection {
                    0 => fission_reactor::CoolingMode::WaterCooling,
                    1 => fission_reactor::CoolingMode::SodiumCooling,
                    _ => return,
                },
            );
            println!("反应堆燃料棒排布:");
            println!("{}", layout);
            println!("燃料表面积: {}", layout.get_total_surface());
            println!("沸腾效率: {}", layout.get_efficiency());
            println!("燃料棒总数: {}", layout.get_assembly_count());
            println!("最大燃烧速率: {}", layout.get_max_speed());
            println!(
                "最大冷却剂消耗速率: {}",
                layout.get_max_speed()
                    * match user_selection {
                        0 => 20_000_f64,
                        1 => 200_000_f64,
                        _ => return,
                    }
            );
        }
        1 => {
            let items = vec!["直接加热", "钠冷加热"];
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("选择锅炉加热方式:")
                .items(&items)
                .default(0)
                .interact_on_opt(&Term::stderr())
                .unwrap()
                .unwrap();
            let layout = boiler::calculate_layout(
                length,
                width,
                height,
                match user_selection {
                    0 => boiler::HeatingMode::DirectHeating,
                    1 => boiler::HeatingMode::SodiumHeating,
                    _ => return,
                },
            );
            println!("分压元件高度:{}", layout.get_spliter_layer());
            println!("发热元件总数:{}", layout.get_heating_element());
            println!("最大蒸汽产出:{}", layout.get_production());
            if user_selection == 1 {
                println!("最大过热钠消耗:{}", layout.get_coolant_consumption());
            }
        }
        _ => (),
    }
    pause();
}

fn check_attributes(length: i64, width: i64, height: i64) -> bool {
    if !((3..=18).contains(&length)) {
        return false;
    }
    if !((3..=18).contains(&width)) {
        return false;
    }
    if !((4..=18).contains(&height)) {
        return false;
    }
    true
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

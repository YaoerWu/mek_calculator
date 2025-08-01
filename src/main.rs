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
            .with_prompt("Enter length")
            .interact_text()
            .unwrap()
            .parse()
            .unwrap();
        width = Input::<String>::new()
            .with_prompt("Enter width")
            .interact_text()
            .unwrap()
            .parse()
            .unwrap();
        height = Input::<String>::new()
            .with_prompt("Enter height")
            .interact_text()
            .unwrap()
            .parse()
            .unwrap();

        if !check_attributes(length, width, height) {
            println!("The specified dimensions cannot form a multiblock structure");
        } else {
            break;
        }
    }

    let items = vec!["Fission Reactor", "Boiler"];
    let user_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select calculator mode:")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();
    match user_selection {
        0 => {
            let items = vec!["Water Cooling", "Sodium Cooling"];
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select reactor cooling mode:")
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
            println!("Reactor fuel rod layout:");
            println!("{}", layout);
            println!("Fuel surface area: {}", layout.get_total_surface());
            println!("Boiling efficiency: {}", layout.get_efficiency());
            println!("Total fuel rods: {}", layout.get_assembly_count());
            println!("Maximum burn rate: {}", layout.get_max_speed());
            println!(
                "Maximum coolant consumption rate: {}",
                layout.get_max_speed()
                    * match user_selection {
                        0 => 20_000_f64,
                        1 => 200_000_f64,
                        _ => return,
                    }
            );
        }
        1 => {
            let items = vec!["Direct Heating", "Sodium Heating"];
            let user_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select boiler heating method:")
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
            println!("Separator element height: {}", layout.get_splitter_layer());
            println!("Total heating elements: {}", layout.get_heating_element());
            println!("Maximum steam output: {}", layout.get_production());
            if user_selection == 1 {
                println!("Maximum superheated sodium consumption: {}", layout.get_coolant_consumption());
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

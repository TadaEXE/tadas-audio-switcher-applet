use tadas_audio_switcher_applet::{config, pactl_util};

use std::io;

pub fn list_sinks_cmd() {
    let sinks = pactl_util::list_sinks();
    let mut i = 1;
    for sink in &sinks {
        println!("{}) {}", i, sink.name);
        i += 1;
    }
}

pub fn make_slot_cmd() {
    list_sinks_cmd();

    let mut config = config::load_config();
    let sinks = pactl_util::list_sinks();

    println!("Please select one sink from the list above:");

    let mut input = String::new();
    let mut valid_input = false;

    let mut index = 0;

    while !valid_input {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let parsed: Result<usize, _> = input.trim().parse();
        if let Ok(p) = parsed {
            if p <= sinks.len() {
                index = p - 1;
                valid_input = true;
            }
        } else {
            println!("Please select one sink from the list above:");
        }
    }

    config.slots.push(config::Slot {
        id: config.id_counter,
        sink: sinks[index].clone(),
    });
    config.id_counter += 1;

    config::write_config(&config);
}

pub fn remove_slot_cmd(slot_id: u32) {
    let mut config = config::load_config();

    
}

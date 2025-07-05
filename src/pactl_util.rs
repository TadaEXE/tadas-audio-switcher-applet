use crate::config::Sink;
use std::process::Command;

pub fn list_sinks() -> Vec<Sink> {
    let output = Command::new("pactl")
        .args(["list", "sinks"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut sinks = vec![];
    let mut name = None;
    let mut description = None;

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("Name: ") {
            name = Some(line["Name: ".len()..].to_string());
        } else if line.starts_with("Description: ") {
            description = Some(line["Description: ".len()..].to_string());
        }

        if name.is_some() && description.is_some() {
            sinks.push(Sink {
                name: description.take().unwrap(),
                id: name.take().unwrap(),
            })
        }
    }

    sinks
}

pub fn set_sink(sink: &Sink) {
    Command::new("pactl")
        .args(["set-default-sink", &sink.id])
        .status()
        .unwrap();
}

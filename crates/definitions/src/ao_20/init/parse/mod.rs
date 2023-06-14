pub mod template;

use ini::{Ini, Properties};

fn get_number(props: &Properties, key: &str) -> usize {
    props.get(key).unwrap_or("0").parse().unwrap_or(0)
}

fn get_count(ini: &Ini, key: &str) -> usize {
    let init = ini
        .section(Some("INIT"))
        .expect("No INIT section! for {key}");

    init.get(key)
        .expect("No {key}!")
        .parse::<usize>()
        .expect("{key} is not a number")
}

fn to_number(value: &str) -> usize {
    value.parse::<usize>().expect("is not a number!")
}

use serde::Deserialize;

use figment::{
    providers::{Format, Toml},
    Figment,
};

#[derive(Debug, Deserialize)]
struct SomeConfig {
    a_list: Vec<String>,
    a_string: String,
    a_int: u64,
    a_float: f64,
    maybe_string: Option<String>,
    maybe_ints: Option<Vec<i32>>,
    maybe_matrix: Option<Vec<Vec<i32>>>,
}

fn main() {
    let filename = "config1.toml";
    let config: SomeConfig = Figment::new()
        .merge(Toml::file(filename))
        .extract()
        .unwrap();

    print_as_values(&filename, &config);
    convert_with_match(&config);

    let filename = "config2.toml";
    let config: SomeConfig = Figment::new()
        .merge(Toml::file(filename))
        .extract()
        .unwrap();

    print_as_values(&filename, &config);
    convert_with_match(&config);
}

fn convert_with_match(config: &SomeConfig) {
    println!("CONVERT:");
    print!("- maybe string: ");
    match config.maybe_string.as_ref() {
        Some(s) => println!("{}", s.to_uppercase()),
        None => println!("none"),
    }

    print!("- maybe ints: ");
    match config.maybe_ints.as_ref() {
        Some(ints) => println!(
            "{}",
            ints.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ),
        None => println!("none"),
    }

    print!("- maybe matrix: ");
    match config.maybe_matrix.as_ref() {
        Some(matrix) => {
            println!(
                "{:?}",
                matrix
                    .iter()
                    .map(|x| x.iter().map(|y| y.to_string()).collect::<Vec<String>>())
                    .collect::<Vec<Vec<String>>>()
            )
        }
        None => println!("none"),
    }

    println!(" ");
}

fn print_as_values(filename: &&str, config: &SomeConfig) {
    println!("CONFIG: {}", filename);
    println!("- a list: {:?}", config.a_list);
    println!("- a string: {}", config.a_string);
    println!("- a int: {}", config.a_int);
    println!("- a float: {}", config.a_float);
    println!(
        "- maybe string: {:?}",
        config.maybe_string.as_ref().unwrap_or(&String::from(""))
    );
    println!(
        "- maybe ints: {:?}",
        config.maybe_ints.as_ref().map_or(&vec![], |i| i)
    );
    println!(
        "- maybe matrix: {:?}",
        config.maybe_matrix.as_ref().map_or(&vec![vec![]], |i| i)
    );
    println!(" ");
}

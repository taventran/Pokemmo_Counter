use crate::pokemon_struct::Pokemon;
use csv::{self, StringRecordsIter};
use std::error::Error;
use std::fs;

// Helper function to ensure the string has 'static lifetime
fn ensure_static(s: &str) -> Result<&'static str, Box<dyn Error>> {
    if s.len() == 0 {
        return Err("String is empty".into());
    }
    Ok(Box::leak(s.to_owned().into_boxed_str()))
}

pub fn create_vec(path: &str) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut saved_data: Vec<Pokemon> = Vec::new();

    for result in reader.records() {
        let record = result.unwrap();
        let name = record.get(0).ok_or("Missing")?;
        let name_static: &'static str = ensure_static(name)?;
        let encounters: i32 = record
            .get(1)
            .unwrap()
            .trim()
            .parse()
            .expect("Couldn't get number");

        let temp_pokemon: Pokemon = Pokemon {
            name: name_static,
            encounters,
        };
        saved_data.push(temp_pokemon);
    }

    Ok(saved_data)
}

pub fn read_from_file(path: &str, poke: String) -> Result<(String, i32), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let poke_name = poke;
    for result in reader.records() {
        let record = result.unwrap();

        if (record.get(0).unwrap() == poke_name) {
            match record.get(1).unwrap().trim().parse::<i32>() {
                Ok(number) => {
                    println!("Parsed number: {}", number);
                    return Ok((poke_name.to_string(), number));
                }
                Err(_) => {
                    println!("Data: {}", record.get(1).unwrap());
                    println!("No number");
                    return Ok((poke_name.to_string(), -1));
                }
            }
        }
    }
    return Ok(("Not found".to_string(), -1));
}

pub fn read_data() -> i32 {
    // let file = File::open("output.txt");
    // let mut buf_reader = BufReader::new(file);
    // let mut data = vec![];
    let data = fs::read_to_string("output.txt").expect("Can't read");

    // Using the parse() method
    match data[12..].trim().parse::<i32>() {
        Ok(number) => {
            println!("Parsed number: {}", number);
            drop(data);
            return number;
        }
        Err(_) => {
            println!("Data: {}", &data[12..]);
            println!("No number");
            return 0;
        }
    }
}

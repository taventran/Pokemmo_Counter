use crate::pokemon_struct::Pokemon;
use csv;
use std::error::Error;

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
    println!("Input name: {}", poke_name);

    for result in reader.records() {
        let record = result.unwrap();
        let name = record.get(0).unwrap().to_string();
        println!("{:?}", name);
        if name == poke_name {
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

    Ok(("Not found".to_string(), -1))
}

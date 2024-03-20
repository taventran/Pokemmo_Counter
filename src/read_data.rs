use crate::pokemon_struct::Pokemon;
use csv;
use csv::Writer;
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;

// Get current directory to create new files there
pub fn get_exe_directory() -> Option<PathBuf> {
    if let Ok(mut path) = env::current_exe() {
        path.pop(); // Remove executable name to get directory
        Some(path)
    } else {
        None
    }
}

// Helper function to ensure the string has 'static lifetime
fn ensure_static(s: &str) -> Result<&'static str, Box<dyn Error>> {
    if s.len() == 0 {
        return Err("String is empty".into());
    }
    Ok(Box::leak(s.to_owned().into_boxed_str()))
}

// Create vector of all current pokemon in file
pub fn create_vec(path: &str) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    // Attempt to open the file with read permissions
    let full_path = get_exe_directory().unwrap().join(path);
    // println!("Path: {:?}", full_path);

    let file = OpenOptions::new().read(true).open(full_path);

    // Check if the file was successfully opened
    let _ = match file {
        Ok(file) => file,
        Err(_) => {
            // If the file doesn't exist, create it
            println!("No CSV file found... Creating save csv file...");
            fs::File::create(get_exe_directory().unwrap().join(path))?;
            let mut writer = Writer::from_path(get_exe_directory().unwrap().join(path))?;
            writer.write_record(&["Name", "Encounter"])?;
            fs::File::open(get_exe_directory().unwrap().join(path))?
        }
    };

    let mut reader = csv::Reader::from_path(get_exe_directory().unwrap().join(path))?;
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
    // Attempt to open the file with read permissions
    let file = OpenOptions::new()
        .read(true)
        .open(get_exe_directory().unwrap().join(path));

    // Check if the file was successfully opened
    let _ = match file {
        Ok(file) => file,
        Err(_) => {
            // If the file doesn't exist, create it
            println!("No CSV file found... Creating save csv file...");
            fs::File::create(get_exe_directory().unwrap().join(path))?;
            let mut writer = Writer::from_path(get_exe_directory().unwrap().join(path))?;
            writer.write_record(&["Name", "Encounter"])?;
            fs::File::open(get_exe_directory().unwrap().join(path))?
        }
    };

    let mut reader = csv::Reader::from_path(get_exe_directory().unwrap().join(path))?;
    let poke_name = poke.to_lowercase();
    // println!("Input name: {}", poke_name);

    for result in reader.records() {
        let record = result.unwrap();
        let name = record.get(0).unwrap().to_string();
        // println!("{:?}", name);
        if name == poke_name {
            match record.get(1).unwrap().trim().parse::<i32>() {
                Ok(number) => {
                    // println!("Parsed number: {}", number);
                    return Ok((poke_name.to_string(), number));
                }
                Err(_) => {
                    // println!("Data: {}", record.get(1).unwrap());
                    // println!("No number");
                    return Ok((poke_name.to_string(), -1));
                }
            }
        }
    }

    Ok(("Not found".to_string(), -1))
}

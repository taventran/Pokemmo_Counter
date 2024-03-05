use crate::pokemon_struct::Pokemon;
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io::Write;

// TODO: Implement a method to save with json
pub fn save_data(p_mon: &Pokemon) -> std::io::Result<()> {
    if (!fs::metadata("save.json").is_ok()) {
        let mut json_file = File::create("save.json").expect("Unable to make file");
        let json_text = serde_json::to_string(&p_mon).unwrap();
        json_file
            .write_all(json_text.as_bytes())
            .expect("Unable to write");
        return Ok(());
    }

    let json_text = serde_json::to_string(&p_mon).unwrap();
    let mut pokemon_vec: Vec<Pokemon> = serde_json::from_str(&json_text)?;
    // Find the Pokemon with the matching name
    if let Some(pokemon) = pokemon_vec.iter_mut().find(|p| p.name == p_mon.name) {
        println!("checking");
        // Update the encounters count for the matching Pokemon
        pokemon.encounters = p_mon.encounters;
        // Serialize the updated Pokemon vector back to JSON
        let updated_json_data = serde_json::to_string_pretty(&pokemon_vec)?;
        // Write the updated JSON data back to the file
        let mut json_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("save.json")?;

        json_file.write_all(updated_json_data.as_bytes())?;
        println!("Encounters updated for {}", p_mon.name);
    } else {
        println!("Checking");
        let json_data = serde_json::to_string_pretty(&p_mon)?;
        let mut json_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("save.json")?;
        json_file.write_all(json_data.as_bytes())?;
        println!("No Pokemon with name {} found.", p_mon.name);
    }

    println!("{}", json_text);

    let mut file = File::create("output.txt").expect("Unable to make file");
    let contents = format!("Encounters: {}", p_mon.encounters);
    file.write_all(contents.as_bytes())?;
    Ok(())
}

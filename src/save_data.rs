use crate::pokemon_struct::Pokemon;
use crate::read_data::get_exe_directory;
use csv;
use csv::Writer;
use std::fs::OpenOptions;

pub fn save_file(
    path: &str,
    mut all_pokemon: Vec<Pokemon>,
    cur_poke: Pokemon,
) -> std::io::Result<()> {
    // Update necessary pokemon
    for pokemon in all_pokemon.iter_mut() {
        if pokemon.name == cur_poke.name {
            pokemon.encounters = cur_poke.encounters;
            break;
        }
    }

    let mut writer = Writer::from_path(get_exe_directory().unwrap().join(path))?;
    // Write the headers
    writer.write_record(&["Name", "Encounter"])?;
    // Wrtie data
    for pokemon in &all_pokemon {
        writer.write_record(&[
            &pokemon.name.to_lowercase(),
            pokemon.encounters.to_string().as_str(),
        ])?;
    }
    Ok(())
}

pub fn new_pokemon(path: &str, name: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(get_exe_directory().unwrap().join(path))
        .unwrap();

    let mut writer = csv::Writer::from_writer(file);

    let _ = writer.write_record(&[name.to_lowercase(), 0.to_string()]);
    // Wrtie data

    Ok(())
}

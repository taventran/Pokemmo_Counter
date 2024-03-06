use crate::pokemon_struct::Pokemon;
use csv;
use csv::Writer;

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
    let mut writer = Writer::from_path(path)?;
    // Write the headers
    writer.write_record(&["Name", "Encounter"])?;
    // Wrtie data
    for pokemon in &all_pokemon {
        writer.write_record(&[&pokemon.name, pokemon.encounters.to_string().as_str()])?;
    }

    Ok(())
}

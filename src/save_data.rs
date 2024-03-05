use crate::pokemon_struct::Pokemon;
use std::fs::File;
use std::io::Write;

pub fn save_data(p_mon: &Pokemon) -> std::io::Result<()> {
    let mut file = File::create("output.txt").expect("Unable to make file");
    let contents = format!("Encounters: {}", p_mon.encounters);
    file.write_all(contents.as_bytes())?;
    drop(file);
    Ok(())
}

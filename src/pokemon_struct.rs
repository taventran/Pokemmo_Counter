// Save pokemon name and number of encounters
pub struct Pokemon<'a> {
    pub name: &'a str,
    pub encounters: u32,
}

// Clone method for pokemon
impl<'a> Pokemon<'a> {
    pub fn clone(&self) -> Pokemon<'a> {
        Pokemon {
            name: self.name,
            encounters: self.encounters,
        }
    }
}

// Save pokemon name and number of encounters
pub struct Pokemon<'a> {
    pub name: &'a str,
    pub encounters: u32,
}

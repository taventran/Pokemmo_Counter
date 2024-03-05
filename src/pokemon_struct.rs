use serde::Deserialize;
use serde::Serialize;

// Save pokemon name and number of encounters
#[derive(serde::Serialize, Deserialize)]
pub struct Pokemon<'a> {
    pub name: &'a str,
    pub encounters: i32,
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

// Create Serializer for Pokemon
// impl<'a> Serialize for Pokemon<'a> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("Pokemon", 2)?;
//         state.serialize_field("name", &self.name)?;
//         state.serialize_field("encounters", &self.encounters)?;
//         state.end()
//     }
// }

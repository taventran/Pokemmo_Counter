use serde::Deserialize;

// Save pokemon name and number of encounters
#[derive(Clone, serde::Serialize, Deserialize, Copy)]
pub struct Pokemon {
    pub name: &'static str,
    pub encounters: i32,
}

// // Clone method for pokemon
// impl Clone for Pokemon {
//     fn clone(&self) -> Pokemon {
//         Pokemon {
//             name: self.name,
//             encounters: self.encounters,
//         }
//     }
//}

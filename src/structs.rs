use serde::{Serialize, Deserialize};


// #[derive(Serialize, Debug)]
// struct ResponseType {
//     initial_cmd: String,
//     r#type: String,
//     text: String,
//     index: usize,
// }

#[derive(Deserialize)]
pub struct RequestType {
    text: String,
}
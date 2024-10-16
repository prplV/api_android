use serde::{Serialize, Deserialize};


// #[derive(Serialize, Debug)]
// struct ResponseType {
//     initial_cmd: String,
//     r#type: String,
//     text: String,
//     index: usize,
// }

pub mod backend_types {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct RequestType {
        text: String,
    }
}

mod postrges_types {
    pub struct users;
    pub struct notes;
}
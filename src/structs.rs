use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use postgres::Client;
// #[derive(Serialize, Debug)]
// struct ResponseType {
//     initial_cmd: String,
//     r#type: String,
//     text: String,
//     index: usize,
// }
type PgClient = Arc<Mutex<Client>>;
pub struct AppState {
    pub db_pool : PgClient,
}

impl AppState {
    pub fn new(pool : PgClient) -> Self {
        AppState {
            db_pool : pool,
        }
    }
}

pub mod backend_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct RequestType {
        text: String,
    }
}

mod postgres_types {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    pub struct Users {
        id : usize,
        name : String,
        mail : String,
        password : String,
    }
    #[derive(Serialize)]
    pub struct SingleNote {
        name : String,
    }
    #[derive(Serialize)]
    pub struct Notes {
        notes : Vec<SingleNote>,
    }
}

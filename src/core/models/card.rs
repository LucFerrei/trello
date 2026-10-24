use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    id: Uuid,
    name: String,
    descrition: String,
    attachments: Vec<String>,
}

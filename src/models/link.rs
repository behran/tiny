use bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Link {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub link: String,
    #[serde(skip_deserializing)]
    pub hash: Option<String>,
}

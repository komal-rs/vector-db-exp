use candid::CandidType;
use oasysdb::{collection::Record, metadata::Metadata};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType)]
pub struct RequestRecord {
    pub vector: Vec<f32>,
    pub data: String,
}

impl From<RequestRecord> for Record {
    fn from(req: RequestRecord) -> Self {
        Record {
            vector: oasysdb::vector::Vector::from(req.vector),
            data: Metadata::from(req.data),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType)]
pub struct RequestSearchResult {
    /// Vector ID.
    pub id: u32,
    /// Distance between the query to the collection vector.
    pub distance: f32,
    /// Data associated with the vector.
    pub data: String,
}

impl From<oasysdb::collection::SearchResult> for RequestSearchResult {
    fn from(result: oasysdb::collection::SearchResult) -> Self {
        let data = match result.data {
            Metadata::Text(text) => text,
            _ => String::new(),
        };

        RequestSearchResult {
            id: result.id,
            distance: result.distance,
            data,
        }
    }
}

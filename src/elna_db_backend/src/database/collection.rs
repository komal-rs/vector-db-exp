use ciborium::de;
use ic_stable_structures::{storable::Bound, Storable};
use oasysdb::collection::{Record, SearchResult};
use oasysdb::err::Error;
use oasysdb::vector::Vector;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{collections::HashSet, usize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub file_names: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Collection {
    pub dimension: usize,
    pub metadata: Metadata,
    inner: oasysdb::collection::Collection,
}

impl Storable for Collection {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        let mut bytes = vec![];
        ciborium::ser::into_writer(self, &mut bytes).unwrap();
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let canister_wasm: Collection = de::from_reader(bytes.as_ref()).unwrap();
        canister_wasm
    }

    const BOUND: Bound = Bound::Unbounded;
}

// impl Storable for Collection {
//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(&bytes, Self).unwrap()
//     }

//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(&self).unwrap())
//     }

//     const BOUND: Bound = Bound::Unbounded;
// }

impl Collection {
    pub fn new(dimension: usize) -> Self {
        let config = oasysdb::collection::Config::default();

        Collection {
            inner: oasysdb::collection::Collection::new(&config),
            dimension,
            metadata: Metadata {
                file_names: HashSet::new(),
            },
        }
    }

    pub fn append(&mut self, records: &Vec<Record>) -> Result<(), String> {
        self.inner.insert_many(records).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn query(&self, vector: &Vector, limit: u32) -> Result<Vec<SearchResult>, Error> {
        self.inner.search(vector, 100000)
    }
}

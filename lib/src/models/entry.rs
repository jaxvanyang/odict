use rkyv::{deserialize, to_bytes};

use crate::{error::Error, serializable, Etymology};

use super::{EntryRef, MediaURL};

serializable! {
  #[derive(Default)]
  #[serde(rename = "entry")]
  pub struct Entry {
    #[serde(rename = "@term")]
    pub term: String,

    #[serde(rename = "@rank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<u32>,

    #[serde(rename = "@see")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<EntryRef>,

    #[serde(default, rename = "ety")]
    pub etymologies: Vec<Etymology>,

    #[serde(default, rename = "media")]
    pub media: Vec<MediaURL>,
  }
}

impl AsRef<Entry> for Entry {
    fn as_ref(&self) -> &Entry {
        self
    }
}

impl Entry {
    pub fn serialize(&self) -> crate::Result<Vec<u8>> {
        let bytes =
            to_bytes::<rkyv::rancor::Error>(self).map_err(|e| Error::Serialize(e.to_string()))?;

        Ok(bytes.to_vec())
    }
}

impl ArchivedEntry {
    pub fn to_entry(&self) -> crate::Result<Entry> {
        let entry: Entry = deserialize::<Entry, rkyv::rancor::Error>(self)
            .map_err(|e| Error::Deserialize(e.to_string()))?;

        Ok(entry)
    }
}

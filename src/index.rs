use web_sys::{IdbIndexParameters, IdbObjectStore};

use crate::{Error, Result};

/// An index builder.
pub struct Index {
    pub(crate) name: String,
    pub(crate) key_path: String,
    pub(crate) unique: Option<bool>,
    pub(crate) multi_entry: Option<bool>,
}

impl Index {
    /// Creates a new index with given name and key path
    pub fn new(name: &str, key_path: &str) -> Self {
        Self {
            name: name.to_owned(),
            key_path: key_path.to_owned(),
            unique: None,
            multi_entry: None,
        }
    }

    /// Specify whether the index should be unique
    pub fn unique(mut self, unique: bool) -> Self {
        self.unique = Some(unique);
        self
    }

    /// Specify whether the index should be multi-entry, i.e., type of the value contained in key path is an array
    pub fn multi_entry(mut self, multi_entry: bool) -> Self {
        self.multi_entry = Some(multi_entry);
        self
    }
}

impl Index {
    pub(crate) fn create(self, object_store: &IdbObjectStore) -> Result<()> {
        if !object_store.index_names().contains(&self.name) {
            let mut params = IdbIndexParameters::new();

            if let Some(unique) = self.unique {
                params.unique(unique);
            }

            if let Some(multi_entry) = self.multi_entry {
                params.multi_entry(multi_entry);
            }

            object_store
                .create_index_with_str_and_optional_parameters(&self.name, &self.key_path, &params)
                .map_err(Error::IndexCreationFailed)?;
        }

        Ok(())
    }
}

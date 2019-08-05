//use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;

use crate::{KvsError, Result};

/// Key-Value store for storing values associated with provided keys for fast lookup
pub struct KvStore {
    vals: HashMap<String, String>,
    file_handle: File,
    buf_reader: BufReader<File>,
}

impl KvStore {
    /// Opens a key value store backed by a file at the provided path.
    pub fn open(path: &Path) -> Result<Self> {
        let file_path = Path::new(path).join("store.log");
        Ok(Self {
            vals: HashMap::new(),
            file_handle: OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_path)?,
            buf_reader: BufReader::new(File::open(&file_path)?),
        })
    }

    /// Get the value associated with the provided key from the store.
    ///
    /// Returns Option::Some(String) when a value is found for the key
    /// Returns Option::None when a value is not found for the key
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // Read values from store file
        let mut cmd_stream = Deserializer::from_reader(&mut self.buf_reader).into_iter::<Command>();
        while let Some(cmd) = cmd_stream.next() {
            match cmd? {
                Command::Set { key, value } => {
                    self.vals.insert(key, value);
                }
                Command::Remove { key } => {
                    self.vals.remove(&key);
                }
            }
        }
        // find value from in-memory hashmap
        Ok(self.vals.get(&key).cloned())
    }

    /// Sets the provided value for the provided key in the store.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        Ok(serde_json::to_writer(&mut self.file_handle, &cmd)?)
    }

    /// Removes the provided key (any value) from the store.
    pub fn remove(&mut self, key: String) -> Result<()> {
        // check if key exists
        if self.get(key.to_owned())?.is_none() {
            return Err(KvsError::KeyNotFound);
        }

        let cmd = Command::remove(key);
        Ok(serde_json::to_writer(&mut self.file_handle, &cmd)?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    pub fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    pub fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

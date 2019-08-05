extern crate structopt;

use kvs::{KvStore, KvsError, Result};
use std::path::Path;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs", about = "A simple, persistant and fast key value store")]
enum KvsConfig {
    #[structopt(name = "get", about = "Get value for provided key from store")]
    Get {
        #[structopt()]
        key: String,
    },
    #[structopt(name = "set", about = "Set value for provided key in store")]
    Set {
        #[structopt()]
        key: String,
        #[structopt()]
        value: String,
    },

    #[structopt(name = "rm", about = "Remove key and its value from store")]
    Remove {
        #[structopt()]
        key: String,
    },
}

fn main() -> Result<()> {
    let mut kvs = KvStore::open(Path::new("."))?;
    let opt = KvsConfig::from_args();
    match opt {
        KvsConfig::Get { key } => match kvs.get(key)? {
            None => {
                println!("Key not found");
            }
            Some(val) => {
                println!("{}", val);
            }
        },
        KvsConfig::Set { key, value } => kvs.set(key, value)?,
        KvsConfig::Remove { key } => match kvs.remove(key) {
            Ok(()) => {
                return Ok(());
            }
            Err(KvsError::KeyNotFound) => {
                println!("Key not found");
                exit(1);
            }
            Err(e) => return Err(e),
        },
    }
    Ok(())
}

#[macro_use]
extern crate structopt;

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

fn main() {
    let opt = KvsConfig::from_args();
    match opt {
        KvsConfig::Get { key } => {
            eprintln!("unimplemented method");
        }
        KvsConfig::Set { key, value } => {
            eprintln!("unimplemented method");
        }
        KvsConfig::Remove { key } => {
            eprintln!("unimplemented method");
        }
    }

    std::process::exit(1);
}

// 02_prod_code.rs  to be renamed   main.rs
// work with
//      error.rs.01 to be renamed   error.rs
//      mod.rs.01   to be renamed   mod.rs

// cargo run

// ! create and use error.rs

mod error;
mod fs;

pub use self::error::{Error, Result}; // reexport

use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}

// 01_prod_code.rs  to be renamed   main.rs
// work with
//      mod.rs.01   to be renamed   mod.rs

// cargo run

// ! transitioning to production code

pub type Result<T> = std::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

mod fs;

use crate::fs::list_files;

fn main() -> Result<()> {
    let files = list_files(".")?; // see the ? here
    println!("{files:#?}");
    Ok(())
}

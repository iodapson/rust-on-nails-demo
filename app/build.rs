extern crate cornucopia;
use cornucopia::{generate_managed, CodegenSettings, Error};
use std::path::Path;
use std::env;

// This script will generate a new cornucopia file every time your schema or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere and embed the generated
// file with a `include_str` statement in your project.
fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let schema_file = "schema.sql";
    //let destination = "src/cornucopia.rs";
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let settings = CodegenSettings{
        is_async: true,
        derive_ser: false,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    generate_managed(
        queries_path,
        vec![schema_file.into()],
        Some( Path::new(&out_dir).join("cornucopia.rs").to_str().unwrap() ),
        false,
        settings,
    )?;

    Ok(())
}
//Irrelevant Note: Resolved cornucopia dependency issue; https://github.com/cornucopia-rs/cornucopia/issues/158

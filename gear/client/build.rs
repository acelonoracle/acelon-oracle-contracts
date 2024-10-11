use sails_client_gen::ClientGenerator;
use std::{env, path::PathBuf};

fn main() {
    let out_dir_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let idl_file_path = out_dir_path.join("acelon_oracle.idl");

    // Generate IDL file for the program
    sails_idl_gen::generate_idl_to_file::<acelon_oracle_app::AcelonOracleProgram>(&idl_file_path)
        .unwrap();

    // Generate client code from IDL file
    ClientGenerator::from_idl_path(&idl_file_path)
        .with_mocks("mocks")
        .generate_to(PathBuf::from(env::var("OUT_DIR").unwrap()).join("acelon_oracle_client.rs"))
        .unwrap();
}

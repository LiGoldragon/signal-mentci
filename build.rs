use std::{env, path::PathBuf};

use schema_rust_next::build::{
    CargoSchemaMetadata, DependencySchema, GenerationDriver, GenerationPlan,
};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-changed=src/schema/lib.rs");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_CRIOME_SCHEMA_DIR");
        CargoSchemaMetadata::new("signal-mentci").emit_schema_directory(&self.crate_root);

        let ordinary_signal =
            DependencySchema::from_cargo_metadata("signal-criome", "signal-criome", "0.4.0")
                .expect("read signal-criome schema metadata")
                .expect("signal-criome schema directory exposed via DEP_SIGNAL_CRIOME_SCHEMA_DIR");

        GenerationDriver::new(
            GenerationPlan::wire_contract(&self.crate_root, "signal-mentci", "0.2.0")
                .with_dependency_schema(ordinary_signal),
        )
        .generate()
        .expect("generate signal-mentci schema artifacts")
        .write_or_check("SIGNAL_MENTCI_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in signal-mentci schema artifacts are fresh");
    }
}

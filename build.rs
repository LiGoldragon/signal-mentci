use schema_rust_next::build::ContractCrateBuild;

fn main() {
    ContractCrateBuild::from_environment(
        "signal-mentci",
        "0.1.0",
        "SIGNAL_MENTCI_UPDATE_SCHEMA_ARTIFACTS",
    )
    .expect_fresh();
}

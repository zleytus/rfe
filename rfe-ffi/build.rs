fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        documentation: false,
        include_guard: Some("rfe_h".to_string()),
        enumeration: cbindgen::EnumConfig {
            rename_variants: cbindgen::RenameRule::ScreamingSnakeCase,
            prefix_with_name: true,
            ..Default::default()
        },
        ..Default::default()
    };

    cbindgen::generate_with_config(crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file("rfe.h");
}

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
        parse: cbindgen::ParseConfig {
            parse_deps: true,
            include: Some(vec!["rfe".to_string()]),
            ..Default::default()
        },
        cpp_compat: true,
        defines: std::collections::HashMap::from_iter([
            ("target_os = windows".to_string(), "_WIN32".to_string()),
            ("target_os = macos".to_string(), "__APPLE__".to_string()),
            ("target_os = linux".to_string(), "__linux__".to_string()),
        ]),
        ..Default::default()
    };

    cbindgen::generate_with_config(crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file("include/rfe.h");
}

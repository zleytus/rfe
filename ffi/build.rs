fn main() {
    generate_c_bindings();
    generate_csharp_bindings();
}

fn generate_c_bindings() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        documentation: true,
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

fn generate_csharp_bindings() {
    csbindgen::Builder::default()
        .input_extern_file("src/common/mod.rs")
        .input_extern_file("src/common/result.rs")
        .input_extern_file("src/common/screen_data.rs")
        .input_extern_file("src/signal_generator/config.rs")
        .input_extern_file("src/signal_generator/model.rs")
        .input_extern_file("src/signal_generator/rf_explorer.rs")
        .input_extern_file("src/spectrum_analyzer/config.rs")
        .input_extern_file("src/spectrum_analyzer/model.rs")
        .input_extern_file("src/spectrum_analyzer/rf_explorer.rs")
        .csharp_dll_name("rfe")
        .generate_csharp_file("include/NativeMethods.g.cs")
        .unwrap();
}

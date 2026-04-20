use std::path::Path;

use wit_component::ComponentEncoder;

pub fn compile_components(cargo_artifact_env_key: &'static str, components: &[&'static str]) {
    for component in components {
        let cargo_artifact_env_key = format!("{cargo_artifact_env_key}_{component}");

        let component_path = std::env::var_os(cargo_artifact_env_key).unwrap();
        let wasm_bytes =
            std::fs::read(Path::new(&component_path)).expect("unable to read component path");
        let cwasm = compile_component(&wasm_bytes);

        let out_dir = std::env::var_os("OUT_DIR").expect("OUT_DIR missing");
        let out_path = Path::new(&out_dir).join(format!("{component}.cwasm"));
        std::fs::write(&out_path, &cwasm).expect("unable to write compiled artifact to disk");
    }
}
pub fn compile_component(wasm_bytes: &[u8]) -> Vec<u8> {
    // use shared config to guarantee compatibility
    let config = wasmtime::Config::default();

    // encode wasm module as wasm component
    let component_bytes = ComponentEncoder::default()
        .module(wasm_bytes)
        .unwrap()
        .encode()
        .expect("failed to encode wasm module bytes");

    let engine = wasmtime::Engine::new(&config).unwrap();
    let binary = engine
        .precompile_component(&component_bytes)
        .expect("failed to encode wasm module bytes");

    binary
}

fn main() {
    compile_components("CARGO_BIN_FILE_COMPONENTS", &["adder"]);
}

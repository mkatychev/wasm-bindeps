use wasmtime::{
    Config, Engine, Store,
    component::{Component, Linker},
    error::Context,
};

mod bindings {
    //! Generated code for the
    wasmtime::component::bindgen!({
        path: "../wit/adder/world.wit",
        world: "adder",
    });
}

pub fn add(binary: &[u8], x: u32, y: u32) -> wasmtime::Result<u32> {
    dbg!(binary.len());
    // Construct engine
    let config = Config::default();
    let engine = Engine::new(&config)?;

    // SAFETY: these are precompiled components
    let component =
        unsafe { Component::deserialize(&engine, binary).context("precompiled component error")? };

    // Construct store for storing running states of the component
    let mut store = Store::new(&engine, ());

    // Construct linker for linking interfaces.
    let linker = Linker::new(&engine);

    // Instantiate the component as an instance of the `adder` world,
    // with the generated bindings
    let instance = bindings::Adder::instantiate(&mut store, &component, &linker)
        .context("Failed to instantiate the example world")?;

    // Call the add function on instance
    instance
        .docs_adder_add()
        .call_add(&mut store, x, y)
        .context("calling add function")
}

#[macro_export]
macro_rules! include_out_dir_bytes {
    ($path:literal) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/", $path))
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adder() {
        let binary = include_out_dir_bytes!("adder.cwasm");
        let result = add(binary, 1, 1).unwrap();
        assert_eq!(result, 2);
    }
}

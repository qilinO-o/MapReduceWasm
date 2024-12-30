use crate::state::States;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Engine, Store};
use anyhow::Context;

bindgen!({
    path: "map.wit",
    world: "mapper",
    async: false
});

pub struct WasmMapRuntime {
    store: Store<States>,
    instance: Mapper,
}

impl WasmMapRuntime {
    pub fn new(wasm_binary: &Vec<u8>) -> anyhow::Result<Self> {
        let engine = Engine::default();
        // Construct component
        let component = Component::from_binary(&engine, &wasm_binary).context("Failed to load map component binary")?;
            
        // Construct store for storing running states of the component
        let wasi_view = States::new();
        let mut store = Store::new(&engine, wasi_view);
        // Construct linker for linking interfaces.
        // For this simple adder component, no need to link additional interfaces.
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;
        let instance = Mapper::instantiate(&mut store, &component, &linker)
            .context("Failed to instantiate the mapper world")?;
        Ok(Self {
            store,
            instance,
        })
    }

    pub fn do_map(&mut self, key: &String, value: &String) -> wasmtime::Result<Vec<(String, String)>> {
        self.instance
            .interface0.call_map(&mut self.store, &key, &value)
            .context("Failed to call map function")
    }
    
}
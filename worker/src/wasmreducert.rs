use crate::state::States;
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Engine, Store};
use anyhow::Context;

bindgen!({
    path: "reduce.wit",
    world: "reducer",
    async: false
});

pub struct WasmReduceRuntime {
    store: Store<States>,
    instance: Reducer,
}

impl WasmReduceRuntime {
    pub fn new(wasm_binary: &Vec<u8>) -> anyhow::Result<Self> {
        let engine = Engine::default();
        // Construct component
        let component = Component::from_binary(&engine, &wasm_binary).context("Failed to load reduce component binary")?;
            
        // Construct store for storing running states of the component
        let wasi_view = States::new();
        let mut store = Store::new(&engine, wasi_view);
        // Construct linker for linking interfaces.
        // For this simple adder component, no need to link additional interfaces.
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;
        let instance = Reducer::instantiate(&mut store, &component, &linker)
            .context("Failed to instantiate the reducer world")?;
        Ok(Self {
            store,
            instance,
        })
    }

    pub fn do_reduce(&mut self, key: &String, values: &Vec<String>) -> wasmtime::Result<(String, String)> {
        self.instance
            .interface0.call_reduce(&mut self.store, &key, &values)
            .context("Failed to call reduce function")
    }
    
}
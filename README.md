# MapReduceWasm

`MapReduceWasm` is a simple map-reduce framework using Rust. It features distributed multilingual tasks with [Wasm Component Model](https://component-model.bytecodealliance.org/introduction.html). With the map/reduce interfaces descripting `.wit` file, tasks can be designed using Rust, Python, C++ or any other language that supports Wasm Component Model.

## Building
Build the main framework:
```shell
cargo build --release
```
Build multilingual map/reduce task (use word count as an example):

Rust map function:
```shell
cd map_wc
cargo install cargo-component
cargo component build --release
```
Python map function:
```shell
cd map_wc_py
pip install componentize-py
componentize-py --wit-path ./map.wit --world mapper componentize app -o map_wc.wasm
```

## Start Server
Start Master server:
```shell
cargo run --release -- master localhost:12345 4 ./wasm_file/map_wc.wasm ./wasm_file/reduce_wc.wasm 10
```
Start Worker server:
```shell
cargo run --release -- worker localhost:12345
```
Or simply run the executable file with `Usage: <server_type> <addr:port> [num_reduces map_wasm_file reduce_wasm_file [timeout_sec]]`.

## Build a Task from Scratch
All following examples build the map task, reduce task is all the same.

Rust:
```shell
cargo component new map_xx --lib && cd map_xx
```
Then update the `wit/world.wit` with our [./worker/map.wit](./worker/map.wit). Adjust the `Cargo.toml` as the following:
```
[package.metadata.component]
package = "component:mapxx"
```
Then design your own map logic in `src/lib.rs`:
```Rust
#[allow(warnings)]
mod bindings;
use bindings::exports::component::mapxx::map::Guest;
struct Component;
#[allow(unused)]
impl Guest for Component {
    fn map(key: String, value: String) -> Vec<(String, String)> {
        ...
        return vec![("", "")];
    }
}
bindings::export!(Component with_types_in bindings);
```
There might be warnings, that's ok. As long as you build it with the following, the bindings will be generated and all warnings will be nowhere.
```shell
cargo component build --release
```
Find the compiled Wasm in `target/wasm32-wasip1/release/`.

Python:
```shell
mkdir map_xx && cd map_xx
touch app.py
```
Place the `map.wit` to this folder and create bindings:
```shell
componentize-py --wit-path ./map.wit --world mapper bindings .
```
> You do not need to generate the bindings in order to componentize in the next step. componentize will generate bindings on-the-fly and bundle them into the produced component.

Then design your own map logic in `app.py`:
```Python
from typing import List
from typing import Tuple
import mapper
class Map(mapper.Mapper):
    def map(self, key: str, value: str) -> List[Tuple[str, str]]:
        ...
        return [("", "")]
```
Finally compile our application to a Wasm component using the componentize subcommand:
```shell
componentize-py --wit-path ./map.wit --world mapper componentize app -o map_wc.wasm
```

Other language support at [guide](https://component-model.bytecodealliance.org/language-support.html).

## Misc
An idea that Wasm is a super light-weight VM which solves the plugin problem.
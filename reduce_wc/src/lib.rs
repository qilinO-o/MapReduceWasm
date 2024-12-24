#[allow(warnings)]
mod bindings;

use bindings::exports::component::reducewc::reduce::Guest;

struct Component;

impl Guest for Component {
    fn reduce(key: String, values: Vec<String>) -> (String, String) {
        return (key, values.get(0).unwrap().to_string());
    }
}

bindings::export!(Component with_types_in bindings);

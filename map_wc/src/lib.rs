#[allow(warnings)]
mod bindings;

use std::vec;

use bindings::exports::component::mapwc::map::Guest;

struct Component;

impl Guest for Component {
    fn map(key: String, value: String) -> Vec<(String, String)> {
        return vec![(key, value)];
    }
}

bindings::export!(Component with_types_in bindings);

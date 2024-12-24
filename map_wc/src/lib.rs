#[allow(warnings)]
mod bindings;

use bindings::exports::component::mapwc::map::Guest;

struct Component;

#[allow(unused)]
impl Guest for Component {
    fn map(key: String, value: String) -> Vec<(String, String)> {
        let terminators = ['.', ',', ' ', '\t', '\n', ';', ':', '"', '-', '\'', '(', ')', '[', ']', '?', '!', '_'];
        let mut words = value.split_terminator(&terminators);
        let mut ret = Vec::new();
        while let Some(w)  = words.next() {
            if w.len() > 0 {
                ret.push((w.to_string(), "1".to_string()));
            }
        }
        return ret;
    }
}

bindings::export!(Component with_types_in bindings);

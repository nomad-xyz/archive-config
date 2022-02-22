//! Pre-set configs bundled with the lib

use std::collections::HashMap;

use once_cell::sync::OnceCell;

use crate::NomadConfig;

// built-in config objects
static TEST_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/configs/test.json"));
static BUILTINS: OnceCell<HashMap<&'static str, NomadConfig>> = OnceCell::new();

/// Get a built-in config object
pub fn get_builtin(name: &str) -> Option<NomadConfig> {
    BUILTINS
        .get_or_init(|| {
            let mut map: HashMap<_, _> = Default::default();
            map.insert("test", serde_json::from_str(TEST_JSON).unwrap());
            // insert others here
            map
        })
        .get(name)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_builtins() {
        dbg!(get_builtin("test"));
    }
}

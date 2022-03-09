//! Pre-set configs bundled with the lib

use std::collections::HashMap;

use once_cell::sync::OnceCell;

use crate::NomadConfig;

// built-in config objects
static TEST_JSON: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/configs/test.json"));
static DEVELOPMENT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/configs/development.json"
));
static STAGING_JSON: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/configs/staging.json"));
static PRODUCTION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/configs/production.json"
));
static BUILTINS: OnceCell<HashMap<&'static str, NomadConfig>> = OnceCell::new();

/// Get a built-in config object
pub fn get_builtin(name: &str) -> Option<NomadConfig> {
    BUILTINS
        .get_or_init(|| {
            let mut map: HashMap<_, _> = Default::default();
            map.insert("test", serde_json::from_str(TEST_JSON).unwrap());
            map.insert(
                "development",
                serde_json::from_str(DEVELOPMENT_JSON).unwrap(),
            );
            map.insert("staging", serde_json::from_str(STAGING_JSON).unwrap());
            map.insert("production", serde_json::from_str(PRODUCTION_JSON).unwrap());
            map
        })
        .get(name)
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loads_builtins() {
        dbg!(get_builtin("test"));
    }

    #[test]
    fn test_validates() {
        dbg!(get_builtin("test")
            .expect("config not found")
            .validate()
            .expect("invalid config"));
    }

    #[test]
    fn development_loads_builtins() {
        dbg!(get_builtin("development"));
    }

    #[test]
    fn development_validates() {
        dbg!(get_builtin("development")
            .expect("config not found")
            .validate()
            .expect("invalid config"));
    }

    #[test]
    fn staging_loads_builtins() {
        dbg!(get_builtin("staging"));
    }

    #[test]
    fn staging_validates() {
        dbg!(get_builtin("staging")
            .expect("config not found")
            .validate()
            .expect("invalid config"));
    }

    #[test]
    fn production_loads_builtins() {
        dbg!(get_builtin("production"));
    }

    #[test]
    fn production_validates() {
        dbg!(get_builtin("production")
            .expect("config not found")
            .validate()
            .expect("invalid config"));
    }
}

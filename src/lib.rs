//! Nomad Configuration crate with wasm bindings

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

use std::collections::{HashMap, HashSet};

pub mod agent;
pub mod common;
pub mod contracts;
pub mod network;

pub mod builtin;
pub use builtin::*;

#[cfg(target_arch = "wasm32")]
/// Wasm bindings for common operations
pub mod wasm;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", global_allocator)]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use common::{NameOrDomain, NomadIdentifier};
use contracts::{BridgeContracts, CoreContracts};
use network::{Domain, NetworkInfo};

/// A Nomad configuration json format
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NomadConfig {
    /// A name for the enviroment (dev/staging/prod/local)
    pub environment: String,
    /// The set of networks used in this config
    pub networks: HashSet<String>,
    /// Pre-configured RPCs for any known networks
    pub rpcs: HashMap<String, HashSet<String>>,
    /// Protocol information (e.g. deploy-time)
    pub protocol: NetworkInfo,
    /// Core deploy information
    pub core: HashMap<String, CoreContracts>,
    /// Bridge contracts for each network
    bridge: HashMap<String, BridgeContracts>,
}

impl NomadConfig {
    /// Resolve a name or domain
    pub fn resolve_domain(&self, domain: NameOrDomain) -> Option<String> {
        self.protocol.resolve_domain(domain)
    }

    /// Syntactically validate the config
    pub fn validate(&self) -> eyre::Result<()> {
        // Check core and bridge exist for all listed networks
        for network in self.networks.iter() {
            eyre::ensure!(
                self.protocol.networks.contains_key(network),
                "Protocol details for network named '{}' not present.",
                network
            );

            // Check that if there is a core for the domain, it contains each
            // replica specified by the connections
            let domain = self.protocol.networks.get(network).unwrap();

            // Check that each network has the expected name
            eyre::ensure!(
                domain.name == *network,
                "Network at key {} has non-matching name: {}",
                network,
                domain.name
            );

            for connection in domain.connections.iter() {
                // Check that IF a core contracts exists, it has al configured
                // replicas
                if let Some(contracts) = self.core.get(network) {
                    eyre::ensure!(
                        contracts.has_replica(connection),
                        "Replica named '{}' not present on core named '{}' despite being listed in core connections",
                        connection,
                        network,
                    );
                }
            }
        }

        // Check each core contains replicas ONLY for its listed connections
        for (name, network) in self.core.iter() {
            eyre::ensure!(
                self.networks.contains(name),
                "Core named '{}' not present in configured networks",
                name,
            );

            // Check each replica
            for replica in network.replicas() {
                // Check that the network is known
                eyre::ensure!(
                    self.networks.contains(name),
                    "Replica named '{}' on core named '{}' not present in base config's configured networks",
                    replica,
                    name,
                );
                // Check for replicas found, but not configured
                eyre::ensure!(
                    self.protocol.networks.get(name).unwrap().connections.contains(replica),
                    "Replica named '{}' on core named '{}' not present in core's configured connections",
                    replica,
                    name
                );
            }
        }

        // Check that no extra bridges are listed
        for network in self.bridge.keys() {
            eyre::ensure!(
                self.networks.contains(network),
                "Bridge named '{}' not present in configured networks",
                network,
            );
        }

        Ok(())
    }

    /// Syntactcially validate the config, consuming and returning self
    pub fn chained_validate(self) -> eyre::Result<Self> {
        self.validate()?;
        Ok(self)
    }

    /// Add a network, replacing any previous network by that name.
    /// If the config is not valid, this function will error and have no effect.
    ///
    /// ## Returns
    ///
    /// The existing network by that name, which was overwritten by the new one
    ///
    /// ## Note:
    ///
    /// This function currently clones the config. This is due to lazy
    /// programming. In the future we'll chill out on the memory usage here
    pub fn add_domain(&mut self, network: Domain) -> eyre::Result<Option<Domain>> {
        let cache = self.clone();

        let name = network.name.clone();
        self.networks.insert(name.clone());
        let held = self.protocol.networks.insert(name, network);

        let valid = self.validate();
        // rewind
        if valid.is_err() {
            *self = cache;
        }
        valid.map(|_| held)
    }

    /// Add a bridge configuration to this config.
    ///
    /// ## Preconditions
    ///
    /// - `name` must already be in the config networks set
    /// - `name` must already have a registered network object in the protocol
    /// block
    ///
    /// Note that these preconditions can be satisfied via `add_domain()`
    pub fn add_core(
        &mut self,
        name: impl AsRef<str>,
        core: CoreContracts,
    ) -> eyre::Result<Option<CoreContracts>> {
        let name = name.as_ref();
        eyre::ensure!(
            self.networks.contains(name),
            "Cannot add core for network named '{}', network not present. Hint: call `add_domain` fist",
            name
        );
        eyre::ensure!(
            self.protocol.networks.contains_key(name),
            "Cannot add bridge for network named '{}', protocol block not present. Hint: call `add_domain` fist",
            name
        );

        Ok(self.core.insert(name.to_owned(), core))
    }

    /// Add a bridge configuration to this config.
    ///
    /// ## Preconditions
    ///
    /// - `name` must already be in the config networks set
    /// - `name` must already have a registered core
    ///
    /// Note that these preconditions can be satisfied via `add_domain()` and
    /// `add_core()`
    pub fn add_bridge(
        &mut self,
        name: impl AsRef<str>,
        bridge: BridgeContracts,
    ) -> eyre::Result<Option<BridgeContracts>> {
        let name = name.as_ref();
        eyre::ensure!(
            self.networks.contains(name),
            "Cannot add bridge for network named '{}', network not present. Hint: call `add_domain` fist",
            name
        );
        eyre::ensure!(
            self.protocol.networks.contains_key(name),
            "Cannot add bridge for network named '{}', protocol block not present. Hint: call `add_domain` fist",
            name
        );
        eyre::ensure!(
            self.core.contains_key(name),
            "Cannot add bridge for network named '{}', core not present. Hint: call `add_core` fist",
            name
        );

        Ok(self.bridge.insert(name.to_owned(), bridge))
    }

    /// Returns a config containing ONLY the networks directly connected to the
    /// specified network. This should be used for agent bootup
    pub fn trim_to_network(&self, network: impl AsRef<str>) -> eyre::Result<NomadConfig> {
        let network = network.as_ref();
        let mut trimmed = self.clone();
        trimmed.protocol = trimmed.protocol.trim_for_network(network)?;
        trimmed.networks = trimmed.protocol.networks();
        trimmed.core = trimmed
            .core
            .into_iter()
            .filter(|(k, _)| trimmed.networks.contains(k))
            .collect();
        trimmed.bridge = trimmed
            .bridge
            .into_iter()
            .filter(|(k, _)| trimmed.networks.contains(k))
            .collect();

        trimmed.chained_validate()
    }

    /// Find the replica of home_network on target_network
    pub fn locate_replica_of(
        &self,
        home_network: NameOrDomain,
        target_network: NameOrDomain,
    ) -> Option<NomadIdentifier> {
        let home_network = self.resolve_domain(home_network)?;
        let target_network = self.resolve_domain(target_network)?;

        self.core
            .get(&target_network)
            .and_then(|contracts| contracts.replica_of(&home_network))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn it_loads_the_sample_config() {
        let path: PathBuf = env!("CARGO_MANIFEST_DIR")
            .parse::<PathBuf>()
            .unwrap()
            .join("configs/test.json");

        let _config: NomadConfig =
            serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        dbg!(&_config);
    }

    #[test]
    fn it_allows_default_config() {
        dbg!(NomadConfig::default());
    }
}

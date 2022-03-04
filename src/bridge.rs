//! Nomad-bridge related configuration structs

use std::collections::HashSet;

use crate::common::{NomadIdentifier, NomadLocator, Proxy};

/// Deploy-time custom tokens
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct DeployedCustomToken {
    /// Token domain and ID
    pub token: NomadLocator,
    /// Token name
    pub name: String,
    /// Token Symbol
    pub symbol: String,
    /// Token decimals
    pub decimals: u8,
    /// Address of the UBC
    pub controller: NomadIdentifier,
    /// Deployed token information
    pub addresses: Proxy,
}

/// EVM Bridge Contracts
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmBridgeContracts {
    /// Bridge Route proxy
    pub bridge_router: Proxy,
    /// Token Registry proxy
    pub token_registry: Proxy,
    /// Bridge Token proxy
    pub bridge_token: Proxy,
    /// Eth Helper address
    #[serde(default)]
    pub eth_helper: Option<NomadIdentifier>,
    /// Custom Tokens (if any)
    #[serde(default)]
    pub customs: HashSet<DeployedCustomToken>,
}

/// Bridge contract abstraction
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum BridgeContracts {
    /// EVM Bridge Contracts
    Evm(EvmBridgeContracts),
    // leaving open future things here
}

impl Default for BridgeContracts {
    fn default() -> Self {
        BridgeContracts::Evm(Default::default())
    }
}

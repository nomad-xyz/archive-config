//! Agent configuration

use nomad_types::agent::{BaseAgentConfig, LogConfig, RpcStyles};
use std::path::PathBuf;

/// Full agent configuration
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentConfig {
    /// RPC specifier
    pub rpc_style: RpcStyles,
    /// Path to the DB
    pub db: PathBuf,
    /// Metrics port
    pub metrics: u16,
    /// Logging configuration
    pub logging: LogConfig,
    /// Updater configuration
    pub updater: BaseAgentConfig,
    /// Relayer configuration
    pub relayer: BaseAgentConfig,
    /// Processor configuration
    pub processor: BaseAgentConfig,
    /// Watcher configuration
    pub watcher: BaseAgentConfig,
    /// Kathy configuration
    pub kathy: BaseAgentConfig,
}

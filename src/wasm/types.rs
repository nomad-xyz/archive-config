//! THIS IS AUTOGENERATED CODE, DO NOT EDIT
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const _: &'static str = r#"
export type NomadIdentifier = string;
export type NameOrDomain = number | string;

export interface NomadLocator {
  domain: number;
  id: NomadIdentifier;
}

export interface LogConfig {
  fmt: string;
  level: string;
}

export interface IndexConfig {
  from: number | string;
  chunk: number | string;
}

export interface BaseAgentConfig {
  enabled: boolean;
  interval: number | string;
}

export interface AgentConfig {
  rpcStyle: string;
  timelag: number | string;
  db: string;
  logging: LogConfig;
  index: IndexConfig;
  updater: BaseAgentConfig;
  relayer: BaseAgentConfig;
  processor: BaseAgentConfig;
  watcher: BaseAgentConfig;
  kathy: BaseAgentConfig;
}

export interface Proxy {
  implementation: NomadIdentifier;
  proxy: NomadIdentifier;
  beacon: NomadIdentifier;
}

export interface EvmCoreContracts {
  upgradeBeaconController: NomadIdentifier;
  xAppConnectionManager: NomadIdentifier;
  updaterManager: NomadIdentifier;
  governanceRouter: Proxy;
  home: Proxy;
  replicas: Record<string, Proxy>;
}

export type CoreContracts = EvmCoreContracts;

export interface DeployedCustomToken {
  token: NomadLocator;
  name: string;
  symbol: string;
  decimals: number;
  controller: NomadIdentifier;
  addresses: Proxy;
}

export interface EvmBridgeContracts {
  bridgeRouter: Proxy;
  tokenRegistry: Proxy;
  bridgeToken: Proxy;
  ethHelper?: NomadIdentifier;
  customs?: Array<DeployedCustomToken>;
}

export type BridgeContracts = EvmBridgeContracts;

export interface Governance {
  recoveryManager: NomadIdentifier;
  recoveryTimelock: number | string;
}

export interface ContractConfig {
  optimisticSeconds: number | string;
  processGas: number | string;
  reserveGas: number | string;
  maximumGas: number | string;
  governance: Governance;
  updater: NomadIdentifier;
  watchers: Array<NomadIdentifier>;
}

export interface NetworkSpecs {
  finalizationBlocks: number | string;
  blockTime: number | string;
  supports1559: boolean;
}

export interface CustomTokenSpecifier {
  token: NomadLocator;
  name: string;
  symbol: string;
  decimals: number;
}

export interface BridgeConfiguration {
  weth?: NomadIdentifier;
  customs: Array<CustomTokenSpecifier>;
}

export interface Domain {
  name: string;
  domain: number;
  connections: Array<string>;
  configuration: ContractConfig;
  specs: NetworkSpecs;
}

export interface NetworkInfo {
  governor: NomadLocator;
  networks: Record<string, Domain>;
}

export interface NomadConfig {
  enviroment: string;
  networks: Array<string>;
  rpcs: Record<string, Array<string>>;
  protocol: NetworkInfo;
  core: Record<string, CoreContracts>;
  bridge: Record<string, BridgeContracts>;
  agent: Record<string, AgentConfig>;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "NomadLocator")]
    pub type NomadLocator;

    #[wasm_bindgen(typescript_type = "LogConfig")]
    pub type LogConfig;

    #[wasm_bindgen(typescript_type = "IndexConfig")]
    pub type IndexConfig;

    #[wasm_bindgen(typescript_type = "BaseAgentConfig")]
    pub type BaseAgentConfig;

    #[wasm_bindgen(typescript_type = "AgentConfig")]
    pub type AgentConfig;

    #[wasm_bindgen(typescript_type = "Proxy")]
    pub type Proxy;

    #[wasm_bindgen(typescript_type = "EvmCoreContracts")]
    pub type EvmCoreContracts;

    #[wasm_bindgen(typescript_type = "CoreContracts")]
    pub type CoreContracts;

    #[wasm_bindgen(typescript_type = "DeployedCustomToken")]
    pub type DeployedCustomToken;

    #[wasm_bindgen(typescript_type = "EvmBridgeContracts")]
    pub type EvmBridgeContracts;

    #[wasm_bindgen(typescript_type = "BridgeContracts")]
    pub type BridgeContracts;

    #[wasm_bindgen(typescript_type = "Governance")]
    pub type Governance;

    #[wasm_bindgen(typescript_type = "ContractConfig")]
    pub type ContractConfig;

    #[wasm_bindgen(typescript_type = "NetworkSpecs")]
    pub type NetworkSpecs;

    #[wasm_bindgen(typescript_type = "CustomTokenSpecifier")]
    pub type CustomTokenSpecifier;

    #[wasm_bindgen(typescript_type = "BridgeConfiguration")]
    pub type BridgeConfiguration;

    #[wasm_bindgen(typescript_type = "Domain")]
    pub type Domain;

    #[wasm_bindgen(typescript_type = "NetworkInfo")]
    pub type NetworkInfo;

    #[wasm_bindgen(typescript_type = "NomadConfig")]
    pub type NomadConfig;
}

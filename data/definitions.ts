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

export interface EvmBridgeContracts {
  bridgeRouter: Proxy;
  tokenRegistry: Proxy;
  bridgeToken: Proxy;
  ethHelper?: NomadIdentifier;
}

export type BridgeContracts = EvmBridgeContracts;

export interface Governance {
  recoveryManager: NomadIdentifier;
  recoveryTimelock: number | string;
}

export interface ProtocolConfiguration {
  blockTime: number | string;
  optimisticSeconds: number | string;
  processGas: number | string;
  reserveGas: number | string;
  maximumGas: number | string;
}

export interface Domain {
  name: string;
  domain: number;
  connections: Array<string>;
  configurationi: ProtocolConfiguration;
  governance: Governance;
  updaters: Array<NomadIdentifier>;
  watchers: Array<NomadIdentifier>;
  agents: AgentConfig;
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
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Version {
    pub Version: String,
    pub Os: String,
    pub KernelVersion: String,
    pub GoVersion: String,
    pub GitCommit: String,
    pub Arch: String,
    pub ApiVersion: String,
    pub BuildTime: String,
    pub Experimental: bool,
}

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Info {
    pub ID: String,
    pub Containers: i32,
    pub ContainersRunning: i32,
    pub ContainersPaused: i32,
    pub ContainersStopped: i32,
    pub Images: i32,
    pub Driver: String,
    pub DriverStatus: Vec<[String; 2]>,
    pub SystemStatus: Vec<[String; 2]>,
    // Plugins            PluginsInfo
    pub MemoryLimit: bool,
    pub SwapLimit: bool,
    pub CpuCfsPeriod: bool,
    pub CpuCfsQuota: bool,
    pub CPUShares: bool,
    pub CPUSet: bool,
    pub IPv4Forwarding: bool,
    pub BridgeNfIptables: bool,
    pub BridgeNfIp6tables: bool,
    pub Debug: bool,
    pub NFd: i32,
    pub OomKillDisable: bool,
    pub NGoroutines: i32,
    pub SystemTime: String,
    pub ExecutionDriver: String,
    pub LoggingDriver: String,
    pub NEventsListener: i32,
    pub KernelVersion: String,
    pub OperatingSystem: String,
    pub OSType: String,
    pub Architecture: String,
    pub IndexServerAddress: String,
    // RegistryConfig     *registry.ServiceConfig
    pub NCPU: i32,
    pub MemTotal: i64,
    pub DockerRootDir: String,
    pub HttpProxy: String,
    pub HttpsProxy: String,
    pub NoProxy: String,
    pub Name: String,
    pub Labels: Vec<String>,
    pub ExperimentalBuild: bool,
    pub ServerVersion: String,
    pub ClusterStore: String,
    pub ClusterAdvertise: String,
}

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

use rocker::util::*;
use rocker::types::*;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    let info = Info {
        Architecture: "amd64".to_string(),
        BridgeNfIp6tables: true,
        BridgeNfIptables: true,
        ClusterAdvertise: "".to_string(),
        ClusterStore: "".to_string(),
        Containers: 0,
        ContainersPaused: 0,
        ContainersRunning: 0,
        ContainersStopped: 0,
        CpuCfsPeriod: true,
        CpuCfsQuota: true,
        CPUSet: true,
        CPUShares: true,
        Debug: true,
        DockerRootDir: "/var/lib/docker".to_string(),
        Driver: "aufs".to_string(),
        DriverStatus: vec![["Status".to_string(), "Healthy".to_string()]],
        SystemStatus: vec![],
        ExecutionDriver: "native".to_string(),
        ExperimentalBuild: true,
        HttpProxy: "".to_string(),
        HttpsProxy: "".to_string(),
        ID: "7TRN:IPZB:QYBB:VPBQ:UMPP:KARE:6ZNR:XE6T:7EWV:PKF4:ZOJD:TPYS".to_string(),
        Images: 0,
        IndexServerAddress: "https://index.docker.io/v1/".to_string(),
        IPv4Forwarding: true,
        KernelVersion: "3.16".to_string(),
        Labels: vec!["storage=ssd".to_string()], // Vec::<String>::new(),
        LoggingDriver: "json".to_string(),
        MemoryLimit: true,
        MemTotal: 0,
        Name: "rust-daemon".to_string(),
        NCPU: 0,
        NEventsListener: 0,
        NFd: 0,
        NGoroutines: 0,
        NoProxy: "".to_string(),
        OomKillDisable: true,
        OperatingSystem: "Debian".to_string(),
        OSType: "linux".to_string(),
        ServerVersion: "1.11-dev".to_string(),
        SwapLimit: true,
        SystemTime: "".to_string(),
    };

    let encoded = json::encode(&info).unwrap();

    // String case to &str automatically
    Ok(Response::with((json_type(), status::Ok, encoded)))
}

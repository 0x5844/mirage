use serde::{Deserialize, Serialize};
use std::env;
use sys_info;

#[derive(Serialize, Deserialize, Debug)]
pub struct OSInfo {
    pub os_type: String,
    pub architecture: String,
    pub host_name: Option<String>,
    pub os_version: Option<String>,
    pub cpu_speed: Option<u64>,
    pub memory_information: Option<u64>,
    pub load_average: Option<f64>,
    pub total_processes: Option<u64>,
    pub boot_time: Option<i64>,
}

pub fn gather_os_info() -> OSInfo {
    OSInfo {
        os_type: env::consts::OS.to_string(),
        architecture: env::consts::ARCH.to_string(),
        host_name: sys_info::hostname().ok(),
        os_version: sys_info::os_release().ok(),
        cpu_speed: sys_info::cpu_speed().ok(),
        memory_information: sys_info::mem_info().ok().map(|m| m.total),
        load_average: sys_info::loadavg().ok().map(|l| l.one),
        total_processes: sys_info::proc_total().ok(),
        boot_time: sys_info::boottime().ok().map(|b| b.tv_sec),
    }
}

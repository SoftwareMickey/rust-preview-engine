use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DevServerInfo {
    pub port : u16,
    pub pid : u32,
    pub last_ping : u64
}
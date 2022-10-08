use std::str;

use serde::{Deserialize, Serialize};
use systemstat::{ByteSize, Memory, PlatformMemory};

extern crate chrono;

#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct Stats {
    pub loadavg: Loadavg,
    pub cpu_usage: String,
    #[serde(with = "MemoryRef")]
    pub memory_usage: Memory,
}

#[derive(Serialize, Deserialize)]
pub struct StatsResponse {
    pub result: bool,
    pub data: Stats,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Loadavg {
    pub one: f32,
    pub five: f32,
    pub fifteen: f32,
}

// Serde calls this the definition of the remote type. It is just a copy of the
// remote data structure. The `remote` attribute gives the path to the actual
// type we intend to derive code for.
#[derive(Serialize, Deserialize)]
#[serde(remote = "PlatformMemory")]
struct PlatformMemoryDef {
    #[serde(with = "ByteSizeRef")]
    pub total: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub active: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub inactive: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub wired: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub free: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub purgeable: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub speculative: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub compressor: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub throttled: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub external: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub internal: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub uncompressed_in_compressor: ByteSize,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ByteSize")]
struct ByteSizeRef(u64);

#[derive(Serialize, Deserialize)]
#[serde(remote = "Memory")]
pub struct MemoryRef {
    #[serde(with = "ByteSizeRef")]
    pub total: ByteSize,
    #[serde(with = "ByteSizeRef")]
    pub free: ByteSize,
    #[serde(with = "PlatformMemoryDef")]
    pub platform_memory: PlatformMemory,
}

// Ref::https://serde.rs/remote-derive.html

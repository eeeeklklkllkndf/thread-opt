use crate::utils::node_reader::read_file;
use compact_str::CompactString;
use serde::Deserialize;
extern crate alloc;
use crate::policy::usage_top1::common::CmdType as Top1Enum;
use alloc::vec::Vec;
use anyhow::Result;
use once_cell::sync::Lazy;

pub type ByteArray = heapless::Vec<u8, 16>;

pub static PROFILE: Lazy<Config> = Lazy::new(|| {
    let profile = read_file::<65536>(b"/data/adb/modules/thread_opt/thread_opt.toml\0").unwrap();
    #[cfg(debug_assertions)]
    log::debug!("{profile:?}");

    let profile: Config = toml::from_str(&profile).unwrap();
    #[cfg(debug_assertions)]
    for i in &profile.comm_match {
        for j in &i.packages {
            log::info!("{j}");
        }
    }
    profile
});

#[derive(Deserialize)]
pub struct Config {
    pub comm_match: Vec<NameMatch>,
    pub usage_top1: Vec<UsageTop1>,
    pub usage_top2: Vec<UsageTop2>,
}

#[derive(Deserialize)]
pub struct NameMatch {
    pub packages: Vec<CompactString>,
    pub policy: Policy,
}

#[derive(Deserialize)]
pub struct UsageTop1 {
    pub packages: Vec<CompactString>,
    #[serde(deserialize_with = "deserialize_byte_array_one")]
    pub max_comm: ByteArray,
    pub max_comm_core: Top1Enum,
    pub policy: Policy,
}

#[derive(Deserialize)]
pub struct UsageTop2 {
    pub packages: Vec<CompactString>,
    #[serde(deserialize_with = "deserialize_byte_array_one")]
    pub max_comm: ByteArray,
    #[serde(default, deserialize_with = "deserialize_byte_array_one_op")]
    pub second_comm: Option<ByteArray>,
}

#[derive(Deserialize)]
pub struct Policy {
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub top: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub only6: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub only7: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub middle: Vec<ByteArray>,
    #[serde(deserialize_with = "deserialize_byte_array")]
    pub background: Vec<ByteArray>,
}

fn deserialize_byte_array<'de, D>(deserializer: D) -> Result<Vec<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<CompactString> = Vec::deserialize(deserializer)?;
    let mut result = Vec::new();
    for s in strings {
        let mut bytes = s.as_bytes();
        if bytes.len() > 16 {
            bytes = &bytes[..16];
        }
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        result.push(vec);
    }
    Ok(result)
}

fn deserialize_byte_array_one<'de, D>(deserializer: D) -> Result<ByteArray, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<CompactString> = Vec::deserialize(deserializer)?;
    if let Some(s) = strings.into_iter().next() {
        let mut bytes = s.as_bytes();
        if bytes.len() > 16 {
            bytes = &bytes[..16];
        }
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        return Ok(vec);
    }
    Ok(heapless::Vec::new())
}

fn deserialize_byte_array_one_op<'de, D>(deserializer: D) -> Result<Option<ByteArray>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let strings: Vec<CompactString> = Vec::deserialize(deserializer)?;
    if let Some(s) = strings.into_iter().next() {
        let mut bytes = s.as_bytes();
        if bytes.len() > 16 {
            bytes = &bytes[..16];
        }
        let vec = heapless::Vec::from_slice(bytes)
            .map_err(|()| serde::de::Error::custom("String exceeds capacity"))?;
        return Ok(Some(vec));
    }
    Ok(None)
}

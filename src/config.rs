use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::parser::PacketEncoding;

// ============================ sing-box 结构 ============================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingBoxConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<LogOptions>,
    pub inbounds: Vec<Inbound>,
    pub outbounds: Vec<Outbound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<RouteOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

/// 用内部 tag（"type"）区分 inbound 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Inbound {
    Mixed {
        tag: String,
        listen: String,
        listen_port: u16,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Outbound {
    Vless(VlessOutbound),
    Direct { tag: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlessOutbound {
    pub tag: String,
    pub server: String,
    pub server_port: u16,
    pub uuid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    /// "xudp" / "packetaddr"，不确定就别设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_encoding: Option<PacketEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TlsOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TlsOptions {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utls: Option<UtlsOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reality: Option<RealityOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtlsOptions {
    pub enabled: bool,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityOptions {
    pub enabled: bool,
    pub public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Transport {
    Ws(WsOptions),
    Grpc(GrpcOptions),
    Http(HttpOptions),
    Httpupgrade(UpgradeOptions),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WsOptions {
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrpcOptions {
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouteOptions {
    /// JSON 字段名是 "final"（Rust 关键字，需 rename）
    #[serde(rename = "final", skip_serializing_if = "Option::is_none")]
    pub final_outbound: Option<String>,
}

fn default_path() -> String {
    "/".to_owned()
}

use serde::{Deserialize, Serialize};

// ==================== 顶层 ====================

/// 对应 JSON 根对象
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inbounds: Vec<Inbound>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outbounds: Vec<Outbound>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,
}

// ==================== 入站 ====================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inbound {
    /// type 是 Rust 关键字，用原始标识符 r#type
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<u16>,
}

// ==================== 出站 ====================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outbound {
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<u16>,

    /// 需要格式校验时改用 uuid::Uuid（开启 serde feature）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<Tls>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}

// ==================== TLS ====================

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Tls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utls: Option<Utls>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Utls {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

// ==================== 传输层 ====================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transport {
    #[serde(rename = "type")]
    pub r#type: String, // "tcp" / "udp" / "ws" ...
}

// ==================== 路由 ====================

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Route {
    /// final 同属 Rust 保留字，需 rename
    #[serde(rename = "final", default, skip_serializing_if = "Option::is_none")]
    pub r#final: Option<String>,
}

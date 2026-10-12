use serde::{Deserialize, Serialize};

/// sing-box 顶层配置，对应文档 Introduction 的 Structure 一节
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SingBoxConfig {
    /// JSON Schema 地址，对应 JSON 中的 `$schema`
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// 日志配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Log>,

    /// DNS 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Dns>,

    /// NTP 配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp: Option<Ntp>,

    /// 证书配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,

    /// 证书提供者列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_providers: Vec<CertificateProvider>,

    /// HTTP 客户端列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_clients: Vec<HttpClient>,

    /// 网络命名空间列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub network_namespaces: Vec<NetworkNamespace>,

    /// 端点列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<Endpoint>,

    /// 入站列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inbounds: Vec<Inbound>,

    /// 出站列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outbounds: Vec<Outbound>,

    /// 路由配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,

    /// 服务列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Service>,

    /// 实验性功能配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<Experimental>,
}

/// 日志配置，详见 Manual → Log
/// 当前页面未提供具体字段，用 flatten 保留任意字段；
/// 后续对照对应文档页补充强类型字段即可。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Log {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// DNS 配置，详见 Manual → DNS
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dns {
    servers: Vec<DnsServer>,
}

/// NTP 配置，详见 Manual → NTP
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ntp {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 证书配置，详见 Manual → Certificate
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 证书提供者，详见 Manual → Certificate Provider
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CertificateProvider {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// HTTP 客户端，详见 Manual → HTTP Client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpClient {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 网络命名空间，详见 Manual → Network Namespace
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkNamespace {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 端点，详见 Manual → Endpoint
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 入站，详见 Manual → Inbound
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inbound {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 出站，详见 Manual → Outbound
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Outbound {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 路由配置，详见 Manual → Route
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Route {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 服务，详见 Manual → Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// 实验性功能，详见 Manual → Experimental
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Experimental {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DnsServer {
    pub r#type: DnsServerType,
    pub tag: String,
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum DnsServerType {
    #[default]
    Empty,
    Local,
    Hosts,
    Tcp,
    Udp,
    Tls,
    Quic,
    Https,
    H3,
    Dhcp,
    Mdns,
    FakeIp,
    Tailscale,
    OpenConnect,
    OpenVpn,
    Resolved,
}

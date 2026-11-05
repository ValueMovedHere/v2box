use std::{collections::BTreeMap, convert::Infallible, fmt, str::FromStr};

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;
use uuid::Uuid;

use crate::config::*;
use crate::errors::Error;

// ==================== 字符串枚举（未知值不报错） ====================
// 好处：遇到新客户端产出的新取值时不会解析失败，并且能原样写回。

macro_rules! string_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($variant:ident => $val:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis enum $name {
            $($variant,)+
            /// 未知取值，保留原始字符串
            Unknown(String),
        }

        impl $name {
            $vis fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $val,)+
                    Self::Unknown(raw) => raw,
                }
            }
        }

        impl FromStr for $name {
            type Err = Infallible;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($val => Self::$variant,)+
                    other => Self::Unknown(other.to_owned()),
                })
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                ser.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                let raw = String::deserialize(de)?;
                match raw.parse() {
                    Ok(v) => Ok(v),
                    Err(never) => match never {}, // Infallible
                }
            }
        }
    };
}

string_enum! {
    /// query: encryption，VLESS 目前规范上只接受 none
    pub enum Encryption { None => "none" }
}
string_enum! {
    /// query: type（底层传输）
    pub enum Network {
        Tcp => "tcp",
        Raw => "raw",              // 新版客户端用 raw 表示裸 TCP
        Ws  => "ws",
        Grpc => "grpc",
        Http => "http",
        H2  => "h2",
        Httpupgrade => "httpupgrade",
        Quic => "quic",
        Kcp  => "kcp",
    }
}
string_enum! {
    /// query: security
    pub enum Security { None => "none", Tls => "tls", Reality => "reality", Xtls => "xtls" }
}
string_enum! {
    /// query: fp（uTLS 指纹）
    pub enum Fingerprint {
        Chrome => "chrome", Firefox => "firefox", Safari => "safari",
        Ios => "ios", Android => "android", Edge => "edge",
        Three60 => "360", Qq => "qq", Random => "random", Randomized => "randomized",
    }
}
string_enum! {
    /// query: flow（仅 tcp + tls 场景）
    pub enum Flow { Vision => "xtls-rprx-vision", VisionUdp443 => "xtls-rprx-vision-udp443" }
}
string_enum! {
    /// sing-box outbound: packet_encoding
    pub enum PacketEncoding { Xudp => "xudp", PacketAddr => "packetaddr" }
}

impl Default for Encryption {
    fn default() -> Self {
        Self::None
    }
}
impl Default for Network {
    fn default() -> Self {
        Self::Tcp
    }
}
impl Default for Security {
    fn default() -> Self {
        Self::None
    }
}

// ============================ query 参数 ============================

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct VlessParams {
    /// encryption=none —— 分享链接规范要求显式出现，所以不 skip
    pub encryption: Encryption,

    /// type=tcp|ws|grpc|h2|...  （Rust 关键字，需要 rename）
    #[serde(rename = "type")]
    pub network: Network,

    /// security=none|tls|reality
    pub security: Security,

    /// sni=
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,

    /// fp=chrome|firefox|safari|ios|android|edge|random|randomized
    #[serde(rename = "fp", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Fingerprint>,

    /// alpn=h2,http/1.1（逗号分隔，需要自定义序列化）
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "serde_alpn::deserialize",
        serialize_with = "serde_alpn::serialize"
    )]
    pub alpn: Option<Vec<String>>,

    /// path=/ws（url/serde_urlencoded 已自动 percent-decode）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// host=... —— HTTP/WS 的 Host 头，注意不是连接地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,

    /// serviceName=（gRPC）
    #[serde(rename = "serviceName", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,

    /// headerType=none|http（老式 tcp 伪装）
    #[serde(rename = "headerType", skip_serializing_if = "Option::is_none")]
    pub header_type: Option<String>,

    /// flow=xtls-rprx-vision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<Flow>,

    /// allowInsecure=1|0|true|false
    #[serde(
        rename = "allowInsecure",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "serde_boolish::deserialize",
        serialize_with = "serde_boolish::serialize"
    )]
    pub allow_insecure: Option<bool>,

    /// pbk=（Reality 公钥）
    #[serde(rename = "pbk", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// sid=（Reality short id）
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
}

/// `alpn=h2,http/1.1` 这种逗号分隔值
mod serde_alpn {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<Vec<String>>, s: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(list) => s.serialize_str(&list.join(",")),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<String>>, D::Error> {
        let raw = Option::<String>::deserialize(d)?;
        Ok(raw.map(|s| {
            s.split(',')
                .map(|x| x.trim().to_owned())
                .filter(|x| !x.is_empty())
                .collect()
        }))
    }
}

/// `allowInsecure=1` 这类“1/0/true/false”布尔
mod serde_boolish {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(true) => s.serialize_str("1"),
            Some(false) => s.serialize_str("0"),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<bool>, D::Error> {
        let raw = Option::<String>::deserialize(d)?;
        Ok(
            raw.and_then(|s| match s.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" => Some(true),
                "0" | "false" | "no" => Some(false),
                _ => None, // 不识别的写法按“未指定”处理
            }),
        )
    }
}

// ============================ 链接本体 ============================

/// `vless://<uuid>@<host>:<port>?<query>#<remark>`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct VlessUri {
    pub uuid: Uuid,
    /// 连接地址（CDN 场景下是 CDN 域名/IP）
    pub host: String,
    pub port: u16,
    pub params: VlessParams,
    /// fragment 备注名，已 percent-decode（示例里是 "🌐 via CDN 1"）
    pub tag: Option<String>,
}

/// fragment 需要转义的字符；非 ASCII（emoji、中文）会自动 percent-encode
const FRAGMENT_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'\\');

impl FromStr for VlessUri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s.trim())?;
        if url.scheme() != "vless" {
            return Err(Error::Scheme(url.scheme().to_owned()));
        }
        if url.password().is_some() {
            return Err(Error::UnexpectedPassword);
        }

        let uuid =
            Uuid::parse_str(url.username()).map_err(|_| Error::Uuid(url.username().to_owned()))?;
        let host = url.host_str().ok_or(Error::MissingHost)?.to_owned();
        // vless 分享链接几乎总带端口，缺失时按 443 兜底
        let port = url.port().unwrap_or(443);

        // url.query() 返回的仍是编码态，serde_urlencoded 会做一次 percent-decode
        let params: VlessParams = serde_urlencoded::from_str(url.query().unwrap_or(""))?;

        let tag = match url.fragment() {
            Some(f) => Some(
                percent_encoding::percent_decode_str(f)
                    .decode_utf8()
                    .map_err(|_| Error::FragmentNotUtf8)?
                    .into_owned(),
            ),
            None => None,
        };

        Ok(Self {
            uuid,
            host,
            port,
            params,
            tag,
        })
    }
}

impl fmt::Display for VlessUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let query = serde_urlencoded::to_string(&self.params).map_err(|_| fmt::Error)?;
        let host = if self.host.contains(':') {
            format!("[{}]", self.host) // IPv6 需要方括号
        } else {
            self.host.clone()
        };

        write!(f, "vless://{}@{}:{}?{}", self.uuid, host, self.port, query)?;
        if let Some(tag) = &self.tag {
            write!(f, "#{}", utf8_percent_encode(tag, FRAGMENT_SET))?;
        }
        Ok(())
    }
}

impl TryFrom<String> for VlessUri {
    type Error = Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<VlessUri> for String {
    fn from(v: VlessUri) -> Self {
        v.to_string()
    }
}

impl VlessUri {
    pub fn to_outbound(&self, tag: &str) -> Result<VlessOutbound, Error> {
        let p = &self.params;

        // WS / HTTP 的 Host 头：优先 host，其次 sni，最后回落连接地址
        let host_header = p
            .host
            .clone()
            .or_else(|| p.sni.clone())
            .unwrap_or_else(|| self.host.clone());
        // TLS SNI：优先 sni，其次 host
        let server_name = p
            .sni
            .clone()
            .or_else(|| p.host.clone())
            .unwrap_or_else(|| self.host.clone());

        let tls_enabled = matches!(
            p.security,
            Security::Tls | Security::Reality | Security::Xtls
        );

        if let Some(flow) = &p.flow {
            if !matches!(p.network, Network::Tcp | Network::Raw) || !tls_enabled {
                return Err(Error::FlowRequiresTcpTls(flow.to_string()));
            }
        }
        if matches!(p.security, Security::Reality) && p.public_key.is_none() {
            return Err(Error::RealityNeedsPublicKey);
        }

        let transport = match &p.network {
            // 裸 TCP；若带 headerType=http 则近似映射成 sing-box 的 http transport
            Network::Tcp | Network::Raw => match p.header_type.as_deref() {
                Some("http") => Some(Transport::Http(HttpOptions {
                    host: Some(vec![host_header.clone()]),
                    path: p.path.clone(),
                })),
                _ => None,
            },
            Network::Ws => Some(Transport::Ws(WsOptions {
                path: p.path.clone().unwrap_or_else(|| "/".to_owned()),
                headers: BTreeMap::from([("Host".to_owned(), host_header.clone())]),
                max_early_data: None,
                early_data_header_name: None,
            })),
            Network::Grpc => Some(Transport::Grpc(GrpcOptions {
                service_name: p.service_name.clone().unwrap_or_default(),
            })),
            Network::Http | Network::H2 => Some(Transport::Http(HttpOptions {
                host: Some(vec![host_header.clone()]),
                path: p.path.clone(),
            })),
            Network::Httpupgrade => Some(Transport::Httpupgrade(UpgradeOptions {
                host: Some(host_header.clone()),
                path: p.path.clone(),
            })),
            other => return Err(Error::UnsupportedTransport(other.to_string())),
        };

        let tls = tls_enabled.then(|| TlsOptions {
            enabled: true,
            server_name: Some(server_name),
            insecure: p.allow_insecure,
            alpn: p.alpn.clone(),
            utls: p.fingerprint.as_ref().map(|fp| UtlsOptions {
                enabled: true,
                fingerprint: fp.as_str().to_owned(),
            }),
            reality: match p.security {
                Security::Reality => Some(RealityOptions {
                    enabled: true,
                    public_key: p.public_key.clone().unwrap_or_default(),
                    short_id: p.short_id.clone(),
                }),
                _ => None,
            },
        });

        Ok(VlessOutbound {
            tag: tag.to_owned(),
            server: self.host.clone(),
            server_port: self.port,
            uuid: self.uuid,
            flow: p.flow.as_ref().map(|f| f.to_string()),
            packet_encoding: None, // 需要 UDP over WS 时可设 Some(PacketEncoding::Xudp)
            tls,
            transport,
        })
    }

    /// 一个可直接运行的“最小可用”配置：本地 mixed 入站 + 单个 vless 出站
    pub fn to_sing_box_config(
        &self,
        tag: &str,
        listen: &str,
        listen_port: u16,
    ) -> Result<SingBoxConfig, Error> {
        Ok(SingBoxConfig {
            log: Some(LogOptions {
                level: Some("warn".to_owned()),
            }),
            inbounds: vec![Inbound::Mixed {
                tag: "mixed-in".to_owned(),
                listen: listen.to_owned(),
                listen_port,
            }],
            outbounds: vec![
                Outbound::Vless(self.to_outbound(tag)?),
                Outbound::Direct {
                    tag: "direct".to_owned(),
                },
            ],
            route: Some(RouteOptions {
                final_outbound: Some(tag.to_owned()),
            }),
        })
    }
}

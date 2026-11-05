#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("URL 解析失败: {0}")]
    Url(#[from] url::ParseError),
    #[error("协议不是 vless: {0}")]
    Scheme(String),
    #[error("链接里出现了非预期的密码段")]
    UnexpectedPassword,
    #[error("缺少主机名")]
    MissingHost,
    #[error("UUID 非法: {0}")]
    Uuid(String),
    #[error("query 解析失败: {0}")]
    Query(#[from] serde_urlencoded::de::Error),
    #[error("query 序列化失败: {0}")]
    SerializeQuery(#[from] serde_urlencoded::ser::Error),
    #[error("fragment 不是合法 UTF-8")]
    FragmentNotUtf8,
    #[error("暂不支持的传输方式: {0}")]
    UnsupportedTransport(String),
    #[error("flow={0} 只能用于 tcp + tls(tls/reality)")]
    FlowRequiresTcpTls(String),
    #[error("security=reality 必须带 pbk（public key）")]
    RealityNeedsPublicKey,
}

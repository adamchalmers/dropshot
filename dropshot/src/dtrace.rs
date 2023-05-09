// Copyright 2023 Oxide Computer Company
//! DTrace probes and support

use http::uri::Uri;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RequestInfo {
    pub id: String,
    pub local_addr: std::net::SocketAddr,
    pub remote_addr: std::net::SocketAddr,
    pub method: String,
    pub path: String,
    pub query: Option<String>,
    #[serde(skip)]
    pub headers: http::HeaderMap<http::HeaderValue>,
    #[serde(serialize_with = "serialize_uri")]
    pub uri: Uri,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResponseInfo {
    pub id: String,
    pub local_addr: std::net::SocketAddr,
    pub remote_addr: std::net::SocketAddr,
    pub status_code: u16,
    pub message: String,
}

#[cfg(feature = "usdt-probes")]
#[usdt::provider(provider = "dropshot")]
mod probes {
    use crate::dtrace::{RequestInfo, ResponseInfo};
    fn request__start(_: &RequestInfo) {}
    fn request__done(_: &ResponseInfo) {}
}

/// The result of registering a server's DTrace USDT probes.
#[derive(Debug, Clone, PartialEq)]
pub enum ProbeRegistration {
    /// The probes are explicitly disabled at compile time.
    Disabled,

    /// Probes were successfully registered.
    Succeeded,

    /// Registration failed, with an error message explaining the cause.
    Failed(String),
}

fn serialize_uri<S>(uri: &Uri, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&uri.to_string())
}

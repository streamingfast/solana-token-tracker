// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="60")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="70")]
    pub mints: ::prost::alloc::vec::Vec<Mint>,
    #[prost(message, repeated, tag="80")]
    pub burns: ::prost::alloc::vec::Vec<Burn>,
    #[prost(message, repeated, tag="120")]
    pub initialized_account: ::prost::alloc::vec::Vec<InitializedAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mint {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="4")]
    pub to: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burn {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(double, tag="5")]
    pub amount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializedAccount {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)

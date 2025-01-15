// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdates {
    #[prost(message, optional, tag="1")]
    pub clock: ::core::option::Option<::substreams::pb::substreams::Clock>,
    #[prost(message, repeated, tag="2")]
    pub balance_updates: ::prost::alloc::vec::Vec<BalanceUpdate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdate {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub call_path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub new_balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, optional, tag="1")]
    pub clock: ::core::option::Option<::substreams::pb::substreams::Clock>,
    #[prost(message, repeated, tag="2")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub call_path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burns {
    #[prost(message, optional, tag="1")]
    pub clock: ::core::option::Option<::substreams::pb::substreams::Clock>,
    #[prost(message, repeated, tag="2")]
    pub burns: ::prost::alloc::vec::Vec<Burn>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burn {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub call_path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub burner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenCreations {
    #[prost(message, optional, tag="1")]
    pub clock: ::core::option::Option<::substreams::pb::substreams::Clock>,
    #[prost(message, repeated, tag="2")]
    pub token_creations: ::prost::alloc::vec::Vec<TokenCreation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenCreation {
    #[prost(string, tag="1")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub call_path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub total_supply: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub decimals: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub issuer: ::prost::alloc::string::String,
    #[prost(bool, tag="10")]
    pub is_burnable: bool,
    #[prost(int32, tag="11")]
    pub issue_chain_id: i32,
    #[prost(map="string, string", tag="12")]
    pub external_info: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)

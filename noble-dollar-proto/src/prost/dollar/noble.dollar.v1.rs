// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(uint64, tag = "1")]
    pub total_holders: u64,
    #[prost(string, tag = "2")]
    pub total_principal: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total_yield_accrued: ::prost::alloc::string::String,
}
impl ::prost::Name for Stats {
    const NAME: &'static str = "Stats";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.Stats".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.Stats".into()
    }
}
/// Paused is an event emitted when the module pause
/// state is changed to paused.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Paused {}
impl ::prost::Name for Paused {
    const NAME: &'static str = "Paused";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.Paused".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.Paused".into()
    }
}
/// Unpaused is an event emitted when the module pause
/// state is changed to unpaused.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Unpaused {}
impl ::prost::Name for Unpaused {
    const NAME: &'static str = "Unpaused";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.Unpaused".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.Unpaused".into()
    }
}
/// YieldClaimed is an event emitted whenever a user claims
/// accrued yield.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YieldClaimed {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for YieldClaimed {
    const NAME: &'static str = "YieldClaimed";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.YieldClaimed".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.YieldClaimed".into()
    }
}
/// IndexUpdated is an event emitted when the index is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexUpdated {
    #[prost(int64, tag = "1")]
    pub old_index: i64,
    #[prost(int64, tag = "2")]
    pub new_index: i64,
    #[prost(string, tag = "3")]
    pub total_principal: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub yield_accrued: ::prost::alloc::string::String,
}
impl ::prost::Name for IndexUpdated {
    const NAME: &'static str = "IndexUpdated";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.IndexUpdated".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.IndexUpdated".into()
    }
}
/// GenesisState defines the genesis state of the Noble Dollar module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// portal contains the genesis state of the Noble Dollar Portal submodule.
    #[prost(message, optional, tag = "1")]
    pub portal: ::core::option::Option<super::portal::v1::GenesisState>,
    /// vaults contains the genesis state of the Noble Dollar Vaults submodule.
    #[prost(message, optional, tag = "2")]
    pub vaults: ::core::option::Option<super::vaults::v1::GenesisState>,
    /// paused contains the genesis paused state of the Noble Dollar.
    #[prost(bool, tag = "3")]
    pub paused: bool,
    /// index contains the genesis index of the Noble Dollar, used for rebasing.
    #[prost(int64, tag = "4")]
    pub index: i64,
    /// principal contains the genesis principal amounts of Noble Dollar holders.
    #[prost(map = "string, string", tag = "5")]
    pub principal:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// stats contains the genesis statistics around the Noble Dollar.
    #[prost(message, optional, tag = "6")]
    pub stats: ::core::option::Option<Stats>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.GenesisState".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryIndex {}
impl ::prost::Name for QueryIndex {
    const NAME: &'static str = "QueryIndex";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryIndex".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryIndex".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIndexResponse {
    #[prost(string, tag = "1")]
    pub index: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryIndexResponse {
    const NAME: &'static str = "QueryIndexResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryIndexResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryIndexResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryPaused {}
impl ::prost::Name for QueryPaused {
    const NAME: &'static str = "QueryPaused";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryPaused".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryPaused".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryPausedResponse {
    #[prost(bool, tag = "1")]
    pub paused: bool,
}
impl ::prost::Name for QueryPausedResponse {
    const NAME: &'static str = "QueryPausedResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryPausedResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryPausedResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPrincipal {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPrincipal {
    const NAME: &'static str = "QueryPrincipal";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryPrincipal".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryPrincipal".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPrincipalResponse {
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPrincipalResponse {
    const NAME: &'static str = "QueryPrincipalResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryPrincipalResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryPrincipalResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryYield {
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryYield {
    const NAME: &'static str = "QueryYield";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryYield".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryYield".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryYieldResponse {
    #[prost(string, tag = "1")]
    pub claimable_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryYieldResponse {
    const NAME: &'static str = "QueryYieldResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryYieldResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryYieldResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryStats {}
impl ::prost::Name for QueryStats {
    const NAME: &'static str = "QueryStats";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryStats".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryStats".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStatsResponse {
    #[prost(uint64, tag = "1")]
    pub total_holders: u64,
    #[prost(string, tag = "2")]
    pub total_principal: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total_yield_accrued: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryStatsResponse {
    const NAME: &'static str = "QueryStatsResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.QueryStatsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.QueryStatsResponse".into()
    }
}
/// MsgClaimYield is a message holders of the Noble Dollar can use to claim their yield.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimYield {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgClaimYield {
    const NAME: &'static str = "MsgClaimYield";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.MsgClaimYield".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.MsgClaimYield".into()
    }
}
/// MsgClaimYieldResponse is the response of the ClaimYield message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgClaimYieldResponse {}
impl ::prost::Name for MsgClaimYieldResponse {
    const NAME: &'static str = "MsgClaimYieldResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.MsgClaimYieldResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.MsgClaimYieldResponse".into()
    }
}
/// MsgSetPausedState allows the authority to configure the Noble Dollar Portal paused state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetPausedState {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub paused: bool,
}
impl ::prost::Name for MsgSetPausedState {
    const NAME: &'static str = "MsgSetPausedState";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.MsgSetPausedState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.MsgSetPausedState".into()
    }
}
/// MsgSetPausedStateResponse is the response of the SetPausedState message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgSetPausedStateResponse {}
impl ::prost::Name for MsgSetPausedStateResponse {
    const NAME: &'static str = "MsgSetPausedStateResponse";
    const PACKAGE: &'static str = "noble.dollar.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.dollar.v1.MsgSetPausedStateResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.dollar.v1.MsgSetPausedStateResponse".into()
    }
}
include!("noble.dollar.v1.serde.rs");
include!("noble.dollar.v1.tonic.rs");
// @@protoc_insertion_point(module)

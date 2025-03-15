// @generated
// This file is @generated by prost-build.
/// OwnershipTransferStarted is emitted whenever an ownership transfer is started.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipTransferStarted {
    /// previous_owner is the address of the previous owner.
    #[prost(string, tag = "1")]
    pub previous_owner: ::prost::alloc::string::String,
    /// new_owner is the address of the new owner.
    #[prost(string, tag = "2")]
    pub new_owner: ::prost::alloc::string::String,
}
impl ::prost::Name for OwnershipTransferStarted {
    const NAME: &'static str = "OwnershipTransferStarted";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.OwnershipTransferStarted".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.OwnershipTransferStarted".into()
    }
}
/// OwnershipTransferStarted is emitted whenever an ownership transfer is finalized.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipTransferred {
    /// previous_owner is the address of the previous owner.
    #[prost(string, tag = "1")]
    pub previous_owner: ::prost::alloc::string::String,
    /// new_owner is the address of the new owner.
    #[prost(string, tag = "2")]
    pub new_owner: ::prost::alloc::string::String,
}
impl ::prost::Name for OwnershipTransferred {
    const NAME: &'static str = "OwnershipTransferred";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.OwnershipTransferred".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.OwnershipTransferred".into()
    }
}
/// BlockedAddressesAdded is emitted whenever addresses are added to the blocklist.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockedAddressesAdded {
    /// accounts is the list of addresses that were added to the blocklist.
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BlockedAddressesAdded {
    const NAME: &'static str = "BlockedAddressesAdded";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.BlockedAddressesAdded".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.BlockedAddressesAdded".into()
    }
}
/// BlockedAddressesRemoved is emitted whenever addresses are removed from the blocklist.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockedAddressesRemoved {
    /// accounts is the list of addresses that were removed from the blocklist.
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for BlockedAddressesRemoved {
    const NAME: &'static str = "BlockedAddressesRemoved";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.BlockedAddressesRemoved".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.BlockedAddressesRemoved".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// owner is the address that can control this submodule.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// pending_owner is the address of the new owner during an ownership transfer.
    #[prost(string, tag = "2")]
    pub pending_owner: ::prost::alloc::string::String,
    /// blocked_addresses is a list of blocked user addresses.
    #[prost(string, repeated, tag = "3")]
    pub blocked_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.GenesisState".into()
    }
}
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryOwner {}
impl ::prost::Name for QueryOwner {
    const NAME: &'static str = "QueryOwner";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.QueryOwner".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.QueryOwner".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerResponse {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pending_owner: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryOwnerResponse {
    const NAME: &'static str = "QueryOwnerResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.QueryOwnerResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.QueryOwnerResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddresses {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAddresses {
    const NAME: &'static str = "QueryAddresses";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.QueryAddresses".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.QueryAddresses".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddressesResponse {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAddressesResponse {
    const NAME: &'static str = "QueryAddressesResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.QueryAddressesResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.QueryAddressesResponse".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAddress {
    const NAME: &'static str = "QueryAddress";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.QueryAddress".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.QueryAddress".into()
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct QueryAddressResponse {
    #[prost(bool, tag = "1")]
    pub blocked: bool,
}
impl ::prost::Name for QueryAddressResponse {
    const NAME: &'static str = "QueryAddressResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.QueryAddressResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.QueryAddressResponse".into()
    }
}
/// MsgTransferOwnership implements the transferOwnership (0xf2fde38b) method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferOwnership {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_owner: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgTransferOwnership {
    const NAME: &'static str = "MsgTransferOwnership";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgTransferOwnership".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgTransferOwnership".into()
    }
}
/// MsgTransferOwnershipResponse is the response of the TransferOwnership action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgTransferOwnershipResponse {}
impl ::prost::Name for MsgTransferOwnershipResponse {
    const NAME: &'static str = "MsgTransferOwnershipResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgTransferOwnershipResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgTransferOwnershipResponse".into()
    }
}
/// MsgAcceptOwnership implements the acceptOwnership (0x79ba5097) method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcceptOwnership {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgAcceptOwnership {
    const NAME: &'static str = "MsgAcceptOwnership";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgAcceptOwnership".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgAcceptOwnership".into()
    }
}
/// MsgAcceptOwnershipResponse is the response of the AcceptOwnership action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgAcceptOwnershipResponse {}
impl ::prost::Name for MsgAcceptOwnershipResponse {
    const NAME: &'static str = "MsgAcceptOwnershipResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgAcceptOwnershipResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgAcceptOwnershipResponse".into()
    }
}
/// MsgAddToBlocklist implements the addToBlocklist (0xf71a55f8) method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToBlocklist {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgAddToBlocklist {
    const NAME: &'static str = "MsgAddToBlocklist";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgAddToBlocklist".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgAddToBlocklist".into()
    }
}
/// MsgAddToBlocklistResponse is the response of the AddToBlocklist action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgAddToBlocklistResponse {}
impl ::prost::Name for MsgAddToBlocklistResponse {
    const NAME: &'static str = "MsgAddToBlocklistResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgAddToBlocklistResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgAddToBlocklistResponse".into()
    }
}
/// MsgRemoveFromBlocklist implements the removeFromBlocklist (0xab63e69c) method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveFromBlocklist {
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgRemoveFromBlocklist {
    const NAME: &'static str = "MsgRemoveFromBlocklist";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgRemoveFromBlocklist".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgRemoveFromBlocklist".into()
    }
}
/// MsgRemoveFromBlocklistResponse is the response of the RemoveFromBlocklist action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgRemoveFromBlocklistResponse {}
impl ::prost::Name for MsgRemoveFromBlocklistResponse {
    const NAME: &'static str = "MsgRemoveFromBlocklistResponse";
    const PACKAGE: &'static str = "aura.blocklist.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.blocklist.v1.MsgRemoveFromBlocklistResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.blocklist.v1.MsgRemoveFromBlocklistResponse".into()
    }
}
include!("aura.blocklist.v1.serde.rs");
include!("aura.blocklist.v1.tonic.rs");
// @@protoc_insertion_point(module)

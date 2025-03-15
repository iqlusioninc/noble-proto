// @generated
// This file is @generated by prost-build.
/// Module is the config object of the Forwarding module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// authority defines the custom module authority.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "noble.forwarding.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "noble.forwarding.module.v1.Module".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/noble.forwarding.module.v1.Module".into()
    }
}
include!("noble.forwarding.module.v1.serde.rs");
// @@protoc_insertion_point(module)

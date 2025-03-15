// @generated
// This file is @generated by prost-build.
/// Module is the config object of the Aura module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// denom is the denom this module is allowed to govern, burn, mint, etc.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "aura.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "aura.module.v1.Module".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/aura.module.v1.Module".into()
    }
}
include!("aura.module.v1.serde.rs");
// @@protoc_insertion_point(module)

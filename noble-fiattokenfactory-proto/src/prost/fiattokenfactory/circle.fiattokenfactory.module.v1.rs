// @generated
// This file is @generated by prost-build.
/// Module is the config object of the FiatTokenFactory module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Module {}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "circle.fiattokenfactory.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "circle.fiattokenfactory.module.v1.Module".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/circle.fiattokenfactory.module.v1.Module".into()
    }
}
include!("circle.fiattokenfactory.module.v1.serde.rs");
// @@protoc_insertion_point(module)

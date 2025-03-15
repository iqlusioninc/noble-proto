// Re-export dependency crates for downstream users
pub use cosmos_sdk_proto;
pub use pbjson;
pub use prost;
pub use prost_types;
pub use serde;
pub use tendermint_proto;
#[cfg(feature = "grpc")]
pub use tonic;

// Module exports
pub mod forwarding {
    pub mod amino {
        include!("prost/forwarding/amino.rs");
    }

    pub mod cosmos_app_v1alpha1 {
        include!("prost/forwarding/cosmos.app.v1alpha1.rs");
    }

    pub mod cosmos_auth_v1beta1 {
        include!("prost/forwarding/cosmos.auth.v1beta1.rs");
    }

    pub mod cosmos_base_v1beta1 {
        include!("prost/forwarding/cosmos.base.v1beta1.rs");
    }

    pub mod cosmos_msg_v1 {
        include!("prost/forwarding/cosmos.msg.v1.rs");
    }

    pub mod cosmos_query_v1 {
        include!("prost/forwarding/cosmos.query.v1.rs");
    }

    pub mod cosmos_proto {
        include!("prost/forwarding/cosmos_proto.rs");
    }

    pub mod module {
        pub mod v1 {
            include!("prost/forwarding/noble.forwarding.module.v1.rs");
        }
    }

    pub mod v1 {
        include!("prost/forwarding/noble.forwarding.v1.rs");
    }
}

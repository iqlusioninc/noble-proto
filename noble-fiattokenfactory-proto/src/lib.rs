// Re-export dependency crates for downstream users
pub use cosmos_sdk_proto;
pub use tendermint_proto;

// Module exports
pub mod fiattokenfactory {
    pub mod v1 {
        include!("prost/fiattokenfactory/circle.fiattokenfactory.v1.rs");
    }

    pub mod module {
        pub mod v1 {
            include!("prost/fiattokenfactory/circle.fiattokenfactory.module.v1.rs");
        }
    }

    pub mod cosmos {
        pub mod app {
            pub mod v1alpha1 {
                include!("prost/fiattokenfactory/cosmos.app.v1alpha1.rs");
            }
        }
        pub mod base {
            pub mod query {
                pub mod v1beta1 {
                    include!("prost/fiattokenfactory/cosmos.base.query.v1beta1.rs");
                }
            }
            pub mod v1beta1 {
                include!("prost/fiattokenfactory/cosmos.base.v1beta1.rs");
            }
        }
        pub mod msg {
            pub mod v1 {
                include!("prost/fiattokenfactory/cosmos.msg.v1.rs");
            }
        }
    }

    pub mod cosmos_proto {
        include!("prost/fiattokenfactory/cosmos_proto.rs");
    }

    pub mod amino {
        include!("prost/fiattokenfactory/amino.rs");
    }
}

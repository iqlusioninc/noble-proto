// Re-export dependency crates for downstream users
pub use cosmos_sdk_proto;
pub use tendermint_proto;

// Module exports
pub mod aura {
    pub mod v1 {
        include!("prost/aura/aura.v1.rs");
    }

    pub mod blocklist {
        pub mod v1 {
            include!("prost/aura/aura.blocklist.v1.rs");
        }
    }

    pub mod module {
        pub mod v1 {
            include!("prost/aura/aura.module.v1.rs");
        }
    }

    pub mod cosmos {
        pub mod app {
            pub mod v1alpha1 {
                include!("prost/aura/cosmos.app.v1alpha1.rs");
            }
        }

        pub mod base {
            pub mod query {
                pub mod v1beta1 {
                    include!("prost/aura/cosmos.base.query.v1beta1.rs");
                }
            }
        }

        pub mod msg {
            pub mod v1 {
                include!("prost/aura/cosmos.msg.v1.rs");
            }
        }

        pub mod query {
            pub mod v1 {
                include!("prost/aura/cosmos.query.v1.rs");
            }
        }
    }
}

// Extra proto definitions
pub mod cosmos_proto {
    include!("prost/aura/cosmos_proto.rs");
}

pub mod amino {
    include!("prost/aura/amino.rs");
}

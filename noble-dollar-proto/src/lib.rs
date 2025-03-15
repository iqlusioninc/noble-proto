// Re-export dependency crates for downstream users
pub use cosmos_sdk_proto;
pub use tendermint_proto;

// Module exports for Noble Dollar
pub mod dollar {
    pub mod v1 {
        include!("prost/dollar/noble.dollar.v1.rs");
    }

    pub mod portal {
        pub mod v1 {
            include!("prost/dollar/noble.dollar.portal.v1.rs");
        }

        pub mod ntt {
            pub mod v1 {
                include!("prost/dollar/noble.dollar.portal.ntt.v1.rs");
            }
        }
    }

    pub mod vaults {
        pub mod v1 {
            include!("prost/dollar/noble.dollar.vaults.v1.rs");
        }
    }

    pub mod module {
        pub mod v1 {
            include!("prost/dollar/noble.dollar.module.v1.rs");
        }
    }
}

// Extra proto definitions
pub mod cosmos_proto {
    include!("prost/dollar/cosmos_proto.rs");
}

pub mod cosmos {
    pub mod app {
        pub mod v1alpha1 {
            include!("prost/dollar/cosmos.app.v1alpha1.rs");
        }
    }

    // These are empty modules in the provided code but keeping them for completeness
    pub mod msg {
        pub mod v1 {
            include!("prost/dollar/cosmos.msg.v1.rs");
        }
    }

    pub mod query {
        pub mod v1 {
            include!("prost/dollar/cosmos.query.v1.rs");
        }
    }
}

pub mod amino {
    include!("prost/dollar/amino.rs");
}

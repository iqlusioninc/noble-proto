// Re-export dependency crates for downstream users
pub use cosmos_sdk_proto;
pub use tendermint_proto;

// Module exports
pub mod halo {
    pub mod v1 {
        include!("prost/halo/halo.v1.rs");
    }

    pub mod aggregator {
        pub mod v1 {
            include!("prost/halo/halo.aggregator.v1.rs");
        }
    }

    pub mod entitlements {
        pub mod v1 {
            include!("prost/halo/halo.entitlements.v1.rs");
        }
    }

    pub mod module {
        pub mod v1 {
            include!("prost/halo/halo.module.v1.rs");
        }
    }
}

// Extra proto definitions
pub mod cosmos_proto {
    include!("prost/halo/cosmos_proto.rs");
}

pub mod amino {
    include!("prost/halo/amino.rs");
}


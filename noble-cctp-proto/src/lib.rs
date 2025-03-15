// Re-export dependency crates for downstream users
pub use cosmos_sdk_proto;
pub use tendermint_proto;

// Module exports
pub mod cctp {
    pub mod amino {
        include!("prost/cctp/amino.rs");
    }

    pub mod circle {
        pub mod cctp {
            pub mod module {
                pub mod v1 {
                    include!("prost/cctp/circle.cctp.module.v1.rs");
                }
            }
            pub mod v1 {
                include!("prost/cctp/circle.cctp.v1.rs");
            }
        }
    }
}

// Extra proto definitions
pub mod cosmos_proto {
    include!("prost/cctp/cosmos_proto.rs");
}

pub mod amino {
    include!("prost/cctp/amino.rs");
}

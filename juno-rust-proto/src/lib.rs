#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::bare_urls, rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod traits;
mod type_urls;

pub use prost;
pub use prost_types::Any;
pub use tendermint_proto as tendermint;

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/ibc-go/cosmos.auth.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {
        pub mod v1beta1 {
            include!("prost/ibc-go/cosmos.base.v1beta1.rs");
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/ibc-go/cosmos.upgrade.v1beta1.rs");
        }
    }
}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// Interchain accounts support.
        pub mod interchain_accounts {
            pub mod controller {
                pub mod v1 {
                    include!("prost/ibc-go/ibc.applications.interchain_accounts.controller.v1.rs");
                }
            }

            pub mod host {
                pub mod v1 {
                    include!("prost/ibc-go/ibc.applications.interchain_accounts.host.v1.rs");
                }
            }

            pub mod v1 {
                include!("prost/ibc-go/ibc.applications.interchain_accounts.v1.rs");
            }
        }

        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc-go/ibc.applications.transfer.v1.rs");
            }

            pub mod v2 {
                include!("prost/ibc-go/ibc.applications.transfer.v2.rs");
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC channels.
        pub mod channel {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.channel.v1.rs");
            }
        }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.client.v1.rs");
            }
        }

        /// IBC commitments.
        pub mod commitment {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.commitment.v1.rs");
            }
        }

        /// IBC connections.
        pub mod connection {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.connection.v1.rs");
            }
        }

        /// IBC types.
        pub mod types {
            pub mod v1 {
                include!("prost/ibc-go/ibc.core.types.v1.rs");
            }
        }
    }

    /// IBC light clients.
    pub mod lightclients {
        pub mod localhost {
            pub mod v2 {
                include!("prost/ibc-go/ibc.lightclients.localhost.v2.rs");
            }
        }
        pub mod solomachine {
            pub mod v2 {
                include!("prost/ibc-go/ibc.lightclients.solomachine.v2.rs");
            }

            pub mod v3 {
                include!("prost/ibc-go/ibc.lightclients.solomachine.v3.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("prost/ibc-go/ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
}

/// ICS23 protobuf definitions.
pub mod ics23 {
    include!("prost/ibc-go/cosmos.ics23.v1.rs");
}

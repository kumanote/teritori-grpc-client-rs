pub mod tonic {
    pub use tonic::*;
}

pub mod prost {
    pub use prost::*;
    pub mod types {
        pub use prost_types::*;
    }
}

pub mod tendermint {
    pub mod abci {
        include!("prost/tendermint.abci.rs");
    }
    pub mod crypto {
        include!("prost/tendermint.crypto.rs");
    }
    pub mod p2p {
        include!("prost/tendermint.p2p.rs");
    }
    pub mod types {
        include!("prost/tendermint.types.rs");
    }
    pub mod version {
        include!("prost/tendermint.version.rs");
    }
}

pub mod cosmos {
    pub mod auth {
        pub mod v1beta1 {
            include!("prost/cosmos.auth.v1beta1.rs");
        }
    }

    pub mod bank {
        pub mod v1beta1 {
            include!("prost/cosmos.bank.v1beta1.rs");
        }
    }

    pub mod base {
        pub mod abci {
            pub mod v1beta1 {
                include!("prost/cosmos.base.abci.v1beta1.rs");
            }
        }
        pub mod kv {
            pub mod v1beta1 {
                include!("prost/cosmos.base.kv.v1beta1.rs");
            }
        }
        pub mod query {
            pub mod v1beta1 {
                include!("prost/cosmos.base.query.v1beta1.rs");
            }
        }
        pub mod reflection {
            pub mod v1beta1 {
                include!("prost/cosmos.base.reflection.v1beta1.rs");
            }
        }
        pub mod snapshots {
            pub mod v1beta1 {
                include!("prost/cosmos.base.snapshots.v1beta1.rs");
            }
        }
        pub mod store {
            pub mod v1beta1 {
                include!("prost/cosmos.base.store.v1beta1.rs");
            }
        }
        pub mod tendermint {
            pub mod v1beta1 {
                include!("prost/cosmos.base.tendermint.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("prost/cosmos.base.v1beta1.rs");
        }
    }

    pub mod capability {
        pub mod v1beta1 {
            include!("prost/cosmos.capability.v1beta1.rs");
        }
    }

    pub mod crisis {
        pub mod v1beta1 {
            include!("prost/cosmos.crisis.v1beta1.rs");
        }
    }

    pub mod crypto {
        pub mod multisig {
            include!("prost/cosmos.crypto.multisig.rs");
            pub mod v1beta1 {
                include!("prost/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
        pub mod ed25519 {
            include!("prost/cosmos.crypto.ed25519.rs");
        }
        pub mod secp256k1 {
            include!("prost/cosmos.crypto.secp256k1.rs");
        }
    }

    pub mod distribution {
        pub mod v1beta1 {
            include!("prost/cosmos.distribution.v1beta1.rs");
        }
    }

    pub mod evidence {
        pub mod v1beta1 {
            include!("prost/cosmos.evidence.v1beta1.rs");
        }
    }

    pub mod genutil {
        pub mod v1beta1 {
            include!("prost/cosmos.genutil.v1beta1.rs");
        }
    }

    pub mod gov {
        pub mod v1beta1 {
            include!("prost/cosmos.gov.v1beta1.rs");
        }
    }

    pub mod mint {
        pub mod v1beta1 {
            include!("prost/cosmos.mint.v1beta1.rs");
        }
    }

    pub mod params {
        pub mod v1beta1 {
            include!("prost/cosmos.params.v1beta1.rs");
        }
    }

    pub mod slashing {
        pub mod v1beta1 {
            include!("prost/cosmos.slashing.v1beta1.rs");
        }
    }

    pub mod staking {
        pub mod v1beta1 {
            include!("prost/cosmos.staking.v1beta1.rs");
        }
    }

    pub mod tx {
        pub mod signing {
            pub mod v1beta1 {
                include!("prost/cosmos.tx.signing.v1beta1.rs");
            }
        }
        pub mod v1beta1 {
            include!("prost/cosmos.tx.v1beta1.rs");
        }
    }

    pub mod upgrade {
        pub mod v1beta1 {
            include!("prost/cosmos.upgrade.v1beta1.rs");
        }
    }

    pub mod vesting {
        pub mod v1beta1 {
            include!("prost/cosmos.vesting.v1beta1.rs");
        }
    }
}

pub mod ibc {
    pub mod applications {
        pub mod transfer {
            pub mod v1 {
                include!("prost/ibc.applications.transfer.v1.rs");
            }
        }
    }
    pub mod core {
        pub mod channel {
            pub mod v1 {
                include!("prost/ibc.core.channel.v1.rs");
            }
        }
        pub mod client {
            pub mod v1 {
                include!("prost/ibc.core.client.v1.rs");
            }
        }
        pub mod commitment {
            pub mod v1 {
                include!("prost/ibc.core.commitment.v1.rs");
            }
        }
        pub mod connection {
            pub mod v1 {
                include!("prost/ibc.core.connection.v1.rs");
            }
        }
        pub mod types {
            pub mod v1 {
                include!("prost/ibc.core.types.v1.rs");
            }
        }
    }
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include!("prost/ibc.lightclients.localhost.v1.rs");
            }
        }
        pub mod solomachine {
            pub mod v1 {
                include!("prost/ibc.lightclients.solomachine.v1.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("prost/ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
}

pub mod ics23 {
    include!("prost/ics23.rs");
}

pub mod teritori {
    pub mod airdrop {
        pub mod v1beta1 {
            include!("prost/teritori.airdrop.v1beta1.rs");
        }
    }
    pub mod mint {
        pub mod v1beta1 {
            include!("prost/teritori.mint.v1beta1.rs");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_block() -> Result<(), Box<dyn std::error::Error>> {
        let addr = "http://localhost:9090";
        let mut client =
            cosmos::base::tendermint::v1beta1::service_client::ServiceClient::connect(addr).await?;
        let request =
            tonic::Request::new(cosmos::base::tendermint::v1beta1::GetLatestBlockRequest {});
        let response = client.get_latest_block(request).await?;
        let latest_height = response.into_inner().block.unwrap().header.unwrap().height;
        let request =
            tonic::Request::new(cosmos::base::tendermint::v1beta1::GetBlockByHeightRequest {
                height: latest_height,
            });
        let response = client.get_block_by_height(request).await?;
        assert_eq!(
            latest_height,
            response.into_inner().block.unwrap().header.unwrap().height
        );
        Ok(())
    }
}

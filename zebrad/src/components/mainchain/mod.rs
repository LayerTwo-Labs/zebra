//! Manages a connection to the mainchain CUSF enforcer.

mod error;
mod gen;

use color_eyre::eyre::eyre;
use error::{GetSidechainsError, InitMainchainError};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use gen::cusf::mainchain::v1::validator_service_client;
use gen::cusf::mainchain::v1::{get_sidechains_response::SidechainInfo, GetSidechainsRequest};

/// Mainchain configuration section.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    /// The address of the mainchain enforcer.
    pub enforcer_addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enforcer_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50051),
        }
    }
}

/// Sidechain slot number for this sidechain.
pub const THIS_SIDECHAIN: u32 = 5;

/// Client for the mainchain enforcer.
#[derive(Debug, Clone)]
pub struct MainchainEnforcerClient {
    inner: validator_service_client::ValidatorServiceClient<tonic::transport::Channel>,
}

impl MainchainEnforcerClient {
    /// Creates a new client connected to the given address.
    pub async fn new(addr: SocketAddr) -> Result<Self, InitMainchainError> {
        let endpoint =
            tonic::transport::channel::Endpoint::new(format!("http://{addr}")).map_err(|err| {
                InitMainchainError::InitEnforcer(eyre!("unable to create endpoint: {err}"))
            })?;

        let channel = endpoint.connect().await.map_err(|err| {
            InitMainchainError::InitEnforcer(eyre!(
                "unable to connect to endpoint at `{addr}`: {err}",
            ))
        })?;
        let mut inner = validator_service_client::ValidatorServiceClient::new(channel);

        // Verify we're able to fetch data
        match inner
            .get_sidechains(tonic::Request::new(GetSidechainsRequest {}))
            .await
        {
            Ok(response) => {
                let sidechains = response.into_inner();
                info!("fetched sidechains: {sidechains:?}");
            }
            Err(e) => {
                return Err(InitMainchainError::InitEnforcer(eyre!(
                    "unable to fetch sidechains: {e}"
                )));
            }
        }

        Ok(Self { inner })
    }

    /// Fetches the list of active sidechains from the mainchain enforcer.
    pub async fn get_sidechains(self) -> Result<Vec<SidechainInfo>, GetSidechainsError> {
        let mut inner = self.inner;
        let response = inner
            .get_sidechains(tonic::Request::new(GetSidechainsRequest {}))
            .await?;
        Ok(response.into_inner().sidechains)
    }
}

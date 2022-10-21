use entropy_beacon_cosmos::provide::{ActiveRequestsQuery, ActiveRequestsResponse};

use crate::cosmos::{network::Network, queries::QueryError, wallet::Wallet};

pub struct Beacon {
    pub network: Network,
    pub signer: Wallet,
    pub address: String,
}

impl Beacon {
    pub fn new(network: Network, signer: Wallet, address: String) -> Self {
        Self {
            network,
            signer,
            address,
        }
    }

    pub async fn fetch_active_requests(&self) -> Result<ActiveRequestsResponse, QueryError> {
        serde_json::from_value::<ActiveRequestsResponse>(
            self.network
                .query(self.address.clone(), ActiveRequestsQuery {})
                .await?,
        )
        .map_err(|e| QueryError::ParseError(e.to_string()))
    }
}

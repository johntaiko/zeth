use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use zeth_primitives::{Address, B256};

#[cfg(feature = "succinct")]
use super::proof::succinct::SP1Proof;

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum ProofRequest {
    Sgx(SgxRequest),
    PseZk(PseZkRequest),
    #[cfg(feature = "powdr")]
    Powdr(PowdrRequest),
    #[cfg(feature = "succinct")]
    Succinct(SP1Request),
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgxRequest {
    /// the l2 block number
    pub block: u64,
    /// l2 node for get block by number
    pub l2_rpc: String,
    /// l1 node for signal root verify and get txlist from proposed transaction.
    pub l1_rpc: String,
    /// the protocol instance data
    #[serde_as(as = "DisplayFromStr")]
    pub prover: Address,
    pub graffiti: B256,
}

pub type PowdrRequest = SgxRequest;

pub type SP1Request = SgxRequest;

#[derive(Clone, Serialize, Deserialize)]
pub struct PseZkRequest {}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProofResponse {
    Sgx(SgxResponse),
    PseZk(PseZkResponse),
    #[cfg(feature = "succinct")]
    SP1(SP1Response),
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgxResponse {
    /// proof format: 4b(id)+20b(pubkey)+65b(signature)
    pub proof: String,
    pub quote: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PseZkResponse {}

#[cfg(feature = "succinct")]
#[derive(Clone, Serialize, Deserialize)]
pub struct SP1Response {
    pub proof: String,
    pub pi_hash: String,
}

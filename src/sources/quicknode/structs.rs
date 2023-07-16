use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::DefaultOnError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorDetails {
    #[serde(rename = "execution_optimistic")]
    pub execution_optimistic: bool,
    pub finalized: bool,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub index: String,
    pub balance: String,
    pub status: String,
    pub validator: Validator,
}
#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validator {
    pub pubkey: String,
    #[serde(rename = "withdrawal_credentials")]
    pub withdrawal_credentials: String,
    #[serde(rename = "effective_balance")]
    pub effective_balance: String,
    pub slashed: bool,
    #[serde(rename = "activation_eligibility_epoch")]
    pub activation_eligibility_epoch: String,
    #[serde(rename = "activation_epoch")]
    pub activation_epoch: String,
    #[serde(rename = "exit_epoch")]
    pub exit_epoch: String,
    #[serde(rename = "withdrawable_epoch")]
    pub withdrawable_epoch: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(rename = "total_eth_amount")]
    #[serde(default)]
    pub total_eth_amount: f32
}

pub struct Quicknode {
    pub base_url: String,
    pub apis_validation_path: String,
}
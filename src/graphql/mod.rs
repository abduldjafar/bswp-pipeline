
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_json::json;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BswpStakehouses {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub stakehouse_accounts: Vec<StakehouseAccounts>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub struct StakehouseAccounts {
    pub id: String,
    pub number_of_deposits: String,
    pub register_validator_block_number: String,
    #[serde(rename = "totalETHForSLOTSentToDepositContract")]
    pub total_ethfor_slotsent_to_deposit_contract: String,
    pub lifecycle_status: String,
    pub account_id: String,
    #[serde(rename = "totalDETHMinted")]
    pub total_dethminted: String,
    #[serde(rename = "totalSLOT")]
    pub total_slot: String,
    #[serde(rename = "totalDETHMintedFormatted")]
    pub total_dethminted_formatted: String,
    #[serde(rename = "totalSLOTFormatted")]
    pub total_slotformatted: String,
    #[serde(rename = "totalCollateralizedSLOTInVaultFormatted")]
    pub total_collateralized_slotin_vault_formatted: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(rename = "stakeHouseMetadata")]
    #[serde(default)]
    pub stake_house_metadata: StakeHouseMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StakeHouseMetadata {
    #[serde(rename = "sETHRedemptionRateFormatted")]
    pub s_ethredemption_rate_formatted: String,
    #[serde(rename = "sETHExchangeRateFormatted")]
    pub s_ethexchange_rate_formatted: String,
    #[serde(rename = "sETHPayoffRateFormatted")]
    pub s_ethpayoff_rate_formatted: String,
}

pub struct Bswp {
    pub graphql_stackhouses_query : String,
    pub api_url: String,
    pub firts: i32,
    pub skip: i32 
}

impl Bswp {

    pub fn new() -> Self {
        let firts = 1000;
        let skip = 0;
        let api_url = "https://api.thegraph.com/subgraphs/name/bswap-eng/stakehouse-protocol".to_string();
        let graphql_stackhouses_query = format!(
            r#"{{
                stakehouseAccounts(
                    orderBy: id
                    orderDirection: desc
                    first: {}
                    skip: {}
                    where: {{
                        registerValidatorBlockNumber_not: "0",
                        lifecycleStatus_gt: "2",
                        stakeHouseMetadata_not: "null"

                    }}
                ) {{
                    id
                    numberOfDeposits
                    registerValidatorBlockNumber
                    totalETHForSLOTSentToDepositContract
                    lifecycleStatus
                    accountId
                    totalDETHMinted
                    totalSLOT
                    totalDETHMintedFormatted
                    totalSLOTFormatted
                    totalCollateralizedSLOTInVaultFormatted
                    stakeHouseMetadata {{
                        sETHRedemptionRateFormatted
                        sETHExchangeRateFormatted
                        sETHPayoffRateFormatted
                    }}
                }}
            }}"#,firts,skip);

        return Bswp { 
            graphql_stackhouses_query,
            firts,
            skip,
            api_url
        }
    }


    pub  fn set_firts(&mut self, value: i32) {
        self.firts = value;
    }

    pub  fn set_skip(&mut self, value: i32) {
        self.skip = value;
    }

    pub fn update_graphql_query(&mut self)  {
        let firts = self.firts;
        let skip = self.skip;

        let graphql_stackhouses_query = format!(
            r#"{{
                stakehouseAccounts(
                    orderBy: id
                    orderDirection: desc
                    first: {}
                    skip: {}
                    where: {{
                        registerValidatorBlockNumber_not: "0",
                        lifecycleStatus_gt: "2",
                        stakeHouseMetadata_not: "null"
                    }}
                ) {{
                    id
                    numberOfDeposits
                    registerValidatorBlockNumber
                    totalETHForSLOTSentToDepositContract
                    lifecycleStatus
                    accountId
                    totalDETHMinted
                    totalSLOT
                    totalDETHMintedFormatted
                    totalSLOTFormatted
                    totalCollateralizedSLOTInVaultFormatted
                    stakeHouseMetadata {{
                        sETHRedemptionRateFormatted
                        sETHExchangeRateFormatted
                        sETHPayoffRateFormatted
                    }}
                }}
            }}"#,firts,skip);

            self.graphql_stackhouses_query = graphql_stackhouses_query;
    }

    pub async fn get_public_key_datas(&self) -> Result<BswpStakehouses, reqwest::Error> {
        let url = self.api_url.to_string();
        let query = &self.graphql_stackhouses_query;
        
        let client = reqwest::Client::new();
        let payload = json!({
            "query": query,
        });
        
        let response = client
            .post(url)
            .json(&payload)
            .send().await?;
        
            let data: BswpStakehouses = response.json().await?;
            Ok(data)
    }
    
    
}
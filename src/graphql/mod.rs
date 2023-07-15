use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BswpStakehouses {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub stakehouse_accounts: Vec<StakehouseAccount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakehouseAccount {
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
    pub stake_house_metadata: StakeHouseMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakeHouseMetadata {
    #[serde(rename = "sETHRedemptionRateFormatted")]
    pub s_ethredemption_rate_formatted: String,
    #[serde(rename = "sETHExchangeRateFormatted")]
    pub s_ethexchange_rate_formatted: String,
    #[serde(rename = "sETHPayoffRateFormatted")]
    pub s_ethpayoff_rate_formatted: String,
}

pub struct Bswp {
    pub graphql_stackhouses_query : &str,
    pub api_url: &str,
    pub firts: i32,
    pub skip: i32 
}

impl Bswp {

    pub fn new(&self) -> Self {
        let firts = 1000;
        let skip = 0;
        let api_url = "https://api.thegraph.com/subgraphs/name/bswap-eng/stakehouse-protocol";
        let graphql_stackhouses_query = format!(r#"
        {{
            stakehouseAccounts(
              orderBy: id
              orderDirection: desc
              first: {}
              skip: {}
              where: {{
                registerValidatorBlockNumber_not: "0" 
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
          }}
        "#,firts,skip).as_str();

        return Bswp { 
            graphql_stackhouses_query,
            firts,
            skip,
            api_url
        }
    }
    pub fn get_public_key_datas(&self) -> Result<BswpStakehouses, reqwest::Error> {
        let url = self.api_url;
        
        
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(url)
            .json(&serde_json::json!({
                graphql_stackhouses_query
            }))
            .send()?;
        
        let data: BswpStakehouses = response.json()?;
        Ok(data)
    }
    
    
}
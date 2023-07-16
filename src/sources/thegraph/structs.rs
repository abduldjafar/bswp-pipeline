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
}


pub struct Thegraph {
    pub graphql_stackhouses_query : String,
    pub api_url: String,
    pub firts: i32,
    pub skip: i32 
}

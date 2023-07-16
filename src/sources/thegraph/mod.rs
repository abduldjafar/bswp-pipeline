pub mod structs;
use serde_json::json;
use structs::{Thegraph,BswpStakehouses};

impl Thegraph {
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
                }}
            }}"#,firts,skip);

        return Thegraph { 
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

    pub async fn get_validator_datas(&self) {

    }
    
    
}
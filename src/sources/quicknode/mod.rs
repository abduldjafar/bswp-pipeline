pub mod structs;

use std::env;

use reqwest::header::HeaderValue;
use structs::{ValidatorDetails,Quicknode};

impl Quicknode {
    
    pub fn new() -> Self { 
        let apis_validation_path = String::new();
        let base_url = env::var("BASE_URL").expect("BASE_URL environment variable not found");

        Quicknode {
            base_url,
            apis_validation_path
        }
    }

    pub fn set_validator(&mut self, validator: &str){
        self.apis_validation_path = format!("{}/eth/v1/beacon/states/finalized/validators/{}", self.base_url, validator);
    }

    pub async fn get_validator_details(&self) -> Result<ValidatorDetails, reqwest::Error> {
        
        let url = self.apis_validation_path.to_string();

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("accept", HeaderValue::from_static("application/json"))
            .send()
            .await?
            .json()
            .await?;
    
        Ok(response)
    }
}
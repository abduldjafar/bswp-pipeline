use bswp_pipeline::sources::thegraph::structs::{Thegraph, BswpStakehouses};
use bswp_pipeline::sources::quicknode::structs::Quicknode;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// page number
    #[arg(short, long, default_value_t = 0,help="first batch pub keys data")]
    start_page: i32,

    #[arg(short, long, default_value_t = 0,help="end batch pub keys data")]
    end_page: i32,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let mut page = args.start_page;

    let mut bswp_stakehouses = Thegraph::new();
    bswp_stakehouses.set_firts(1000);

    loop {
        
        bswp_stakehouses.set_skip(page);
        bswp_stakehouses.update_graphql_query();

        let datas: BswpStakehouses = bswp_stakehouses.get_public_key_datas().await?;

        let file_name = format!("data/stack_houses_page_{}.json", page);
        let mut file = File::create(&file_name).await?;

        for data in datas.data.stakehouse_accounts {
            let mut validators = Quicknode::new();
            validators.set_validator(&data.id);
            let mut validators_details = validators.get_validator_details().await?;

            let current_balance = validators_details.data.balance.parse::<f32>()?;
            validators_details.data.validator.total_eth_amount = (current_balance - 32_000_000_000.0) / 1_000_000_000.0;

            let data_str = serde_json::to_string(&validators_details)?;
            file.write_all(data_str.as_bytes()).await?;
            file.write_all("\n".as_bytes()).await?;
        }

        

        if page == args.end_page {
            break;
        }

        page += 1;
    }

    Ok(())
}

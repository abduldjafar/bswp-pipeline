use bswp_pipeline::graphql::{Bswp, BswpStakehouses};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bswp_stakehouses = Bswp::new();
    let mut page = 0;
    bswp_stakehouses.set_firts(1000);

    loop {
        bswp_stakehouses.set_skip(page);
        bswp_stakehouses.update_graphql_query();

        let datas: BswpStakehouses = bswp_stakehouses.get_public_key_datas().await?;
        if datas.data.stakehouse_accounts.is_empty() {
            println!("no datas");
            break;
        }

        let file_name = format!("data/stack_houses_page_{}.json", page);
        let mut file = File::create(&file_name).await?;
        
        for data in datas.data.stakehouse_accounts {
            let data_str = serde_json::to_string(&data)?;
            file.write_all(data_str.as_bytes()).await?;
            file.write_all("\n".as_bytes()).await?;
        }

        page += 1;
    }

    Ok(())
}

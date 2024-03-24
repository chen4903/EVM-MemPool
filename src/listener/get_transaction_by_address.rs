use dotenv::dotenv;
use reqwest::get;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    hash: String,
    from: String,
    to: String,
    value: String,
}

//使用Etherscan 获取一个地址的交易数据 返回一个交易数组 目前定义读取交易hash from to value 如果当前地址没有交易  将会返回一个空数组
//address要获取交易的目标地址 start_block 开始过滤的区块 end_block结束过滤的区块
pub async fn fetch_transactions(
    address: &str,
    start_block: u64,
    end_block: u64,
) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("ETHERSCAN_API_KEY").expect("Api error! Please check .env!");

    let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&page=1&offset=10&sort=asc&apikey={}", address, start_block, end_block, api_key);

    let response = get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            let mut transaction_infos = Vec::new();

            for transaction in transactions {
                let hash = transaction["hash"].as_str().unwrap_or("N/A").to_string();
                let from = transaction["from"].as_str().unwrap_or("N/A").to_string();
                let to = transaction["to"].as_str().unwrap_or("N/A").to_string();
                let value = transaction["value"].as_str().unwrap_or("N/A").to_string();

                let transaction_info = TransactionInfo {
                    hash,
                    from,
                    to,
                    value,
                };

                println!("{:?}", transaction_info);

                transaction_infos.push(transaction_info);
            }

            return Ok(transaction_infos);
        }
    } else {
        // 如果响应不成功，抛出异常
        return Err(format!(
            "HTTP request failed with status code: {}",
            response.status()
        )
        .into());
    }

    Ok(Vec::new())
}

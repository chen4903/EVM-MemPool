#![allow(dead_code)]
use ethers::core::types::H160;
use ethers::providers::{Middleware, Provider, StreamExt, Ws};
use std::{
    future,
    sync::{mpsc, Arc},
    error::Error,
    ops::Deref
};
use reqwest::get;
use serde::{Deserialize, Serialize};
use crate::utils::tools;

/// @dev：
///     用于解析ETHERSCAN返回的数据
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    hash: String,
    from: String,
    to: String,
    value: String,
    input: String
}

/// @dev：
///     TODO：还存在问题
///     实时获取mempool池子的所有hash，并且解析hash
/// @param：
///     wss_url：WSS提供商的RPC
pub async fn listen_analysis_all_pool(wss_url: String) -> Result<(), Box<dyn std::error::Error>> {

    let provider = Arc::new(Provider::<Ws>::connect(wss_url).await.unwrap());

    let (sender, receiver) = mpsc::channel();

    let provider_clone = Arc::clone(&provider);
    tokio::spawn(async move {
        provider_clone
            .deref()
            .subscribe_pending_txs()
            .await
            .unwrap()
            .for_each(|tx_hash| {
                sender.send(tx_hash).unwrap();
                future::ready(())
            })
            .await;
    });

    // producer:comsumer = 1:1
    // TODO: producer:comsumer = 1:n
    while let Ok(tx_hash) = receiver.recv() {
        println!("{:?}", tx_hash);
        if let Ok(receipt) = provider.get_transaction(tx_hash).await {
            let target_address: H160 = "0x28C6c06298d514Db089934071355E5743bf21d60"
                .parse()
                .unwrap(); // e.g. Binance hot wallet
            match receipt {
                None => println!("could not find transaction for now"),
                Some(txn) => {
                    if txn.clone().from == target_address {
                        println!("details: {:?}", txn);
                    }
                }
            }
        } else {
            println!(
                "Failed to fetch transaction receipt for hash: {:?}",
                tx_hash
            );
        }
    }

    Ok(())
}

/// @dev：
///     获得某个地址的所有交易，包括普通交易、内部交易
///     函数自带打印功能
/// @param：
///     api_key：区块链浏览器的API接口，ETHERSCAN_API_KEY
///     address：你要查询的地址
///     start_block：从哪个区块开始遍历
///     end_block：查询到哪个区块为止
/// @return：
///     Vec<TransactionInfo> 包含了hash, from, to, value, input
pub async fn fetch_address_all_txs(
    api_key: String,
    address: &str,
    start_block: u64,
    end_block: u64,
) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
    let mut transaction_infos = Vec::new();
    
    let url_normal = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
        address, 
        start_block, 
        end_block, 
        api_key
    );
    let url_internal = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
        address, 
        start_block, 
        end_block, 
        api_key
    );

    let response_normal = get(&url_normal).await?;
    let response_internal = get(&url_internal).await?;

    if response_normal.status().is_success() {
        let body = response_normal.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            

            for transaction in transactions {
                let hash = transaction["hash"].as_str().unwrap().to_string();
                let from = transaction["from"].as_str().unwrap().to_string();
                let to = transaction["to"].as_str().unwrap().to_string();
                let value = transaction["value"].as_str().unwrap().to_string();
                let input = transaction["input"].as_str().unwrap().to_string();

                let transaction_info = TransactionInfo {
                    hash,
                    from,
                    to,
                    value,
                    input,
                };

                println!("{:?}", transaction_info);

                transaction_infos.push(transaction_info);
            }
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response_normal.status()
        )
        .into());
    }

    if response_internal.status().is_success() {
        let body = response_internal.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            

            for transaction in transactions {
                let hash = transaction["hash"].as_str().unwrap().to_string();
                let from = transaction["from"].as_str().unwrap().to_string();
                let to = transaction["to"].as_str().unwrap().to_string();
                let value = transaction["value"].as_str().unwrap().to_string();
                let input = transaction["input"].as_str().unwrap().to_string();

                let transaction_info = TransactionInfo {
                    hash,
                    from,
                    to,
                    value,
                    input,
                };

                println!("{:?}", transaction_info);

                transaction_infos.push(transaction_info);
            }
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response_internal.status()
        )
        .into());
    }

    return Ok(transaction_infos);

}


/// @dev：
///     查询某个地址是否有混币器相关交易
/// @param：
///     api_key：区块链浏览器的API接口，ETHERSCAN_API_KEY
///     address：你要查询的地址
///     start_block：从哪个区块开始遍历
///     end_block：查询到哪个区块为止
/// @return：
///     是否有混币器交易
pub async fn is_invoke_mixing_service(
    api_key: String,
    address: &str,
    start_block: u64,
    end_block: u64,
) -> Result<bool, Box<dyn Error>>{
    let mut is_invoke = false;
    
    let url_normal = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
        address, 
        start_block, 
        end_block, 
        api_key
    );
    let url_internal = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
        address, 
        start_block, 
        end_block, 
        api_key
    );

    let response_normal = get(&url_normal).await?;
    let response_internal = get(&url_internal).await?;
    // 得到混币器列表
    let addresses = tools::get_db_address(2);

    if response_normal.status().is_success() {
        let body = response_normal.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            for transaction in transactions {
                let from = transaction["from"].as_str().unwrap();
                let to = transaction["to"].as_str().unwrap();
                for addr in &addresses {
                    if addr.to_lowercase() == to.to_lowercase() || addr.to_lowercase() == from.to_lowercase() {
                        is_invoke = true;
                    }
                }
            }
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response_internal.status()
        )
        .into());
    }

    if response_internal.status().is_success() {
        let body = response_internal.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            for transaction in transactions {
                let from = transaction["from"].as_str().unwrap();
                let to = transaction["to"].as_str().unwrap();
                for addr in &addresses {
                    if addr.to_lowercase() == to.to_lowercase() || addr.to_lowercase() == from.to_lowercase() {
                        is_invoke = true;
                    }
                }
            }
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response_internal.status()
        )
        .into());
    }

    return Ok(is_invoke);
}

/// @dev：
///     获得某个地址的普通交易
///     函数自带打印功能
/// @param：
///     api_key：区块链浏览器的API接口，ETHERSCAN_API_KEY
///     address：你要查询的地址
///     start_block：从哪个区块开始遍历
///     end_block：查询到哪个区块为止
/// @return：
///     Vec<TransactionInfo> 包含了hash, from, to, value, input
pub async fn fetch_address_normal_txs(
    api_key: String,
    address: &str,
    start_block: u64,
    end_block: u64,
) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
    
    let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
        address, 
        start_block, 
        end_block, 
        api_key
    );

    let response = get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            let mut transaction_infos = Vec::new();

            for transaction in transactions {
                let hash = transaction["hash"].as_str().unwrap().to_string();
                let from = transaction["from"].as_str().unwrap().to_string();
                let to = transaction["to"].as_str().unwrap().to_string();
                let value = transaction["value"].as_str().unwrap().to_string();
                let input = transaction["input"].as_str().unwrap().to_string();

                let transaction_info = TransactionInfo {
                    hash,
                    from,
                    to,
                    value,
                    input,
                };

                println!("{:?}", transaction_info);

                transaction_infos.push(transaction_info);
            }

            return Ok(transaction_infos);
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response.status()
        )
        .into());
    }

    Ok(Vec::new())
}

/// @dev：
///     获得某个地址的内部交易
///     函数自带打印功能
/// @param：
///     api_key：区块链浏览器的API接口，ETHERSCAN_API_KEY
///     address：你要查询的地址
///     start_block：从哪个区块开始遍历
///     end_block：查询到哪个区块为止
/// @return：
///     Vec<TransactionInfo> 包含了hash, from, to, value, input
pub async fn fetch_address_internal_txs(
    api_key: String,
    address: &str,
    start_block: u64,
    end_block: u64,
) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
    
    let url = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
        address, 
        start_block, 
        end_block, 
        api_key
    );

    let response = get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(transactions) = json_data["result"].as_array() {
            let mut transaction_infos = Vec::new();

            for transaction in transactions {
                let hash = transaction["hash"].as_str().unwrap().to_string();
                let from = transaction["from"].as_str().unwrap().to_string();
                let to = transaction["to"].as_str().unwrap().to_string();
                let value = transaction["value"].as_str().unwrap().to_string();
                let input = transaction["input"].as_str().unwrap().to_string();

                let transaction_info = TransactionInfo {
                    hash,
                    from,
                    to,
                    value,
                    input,
                };

                println!("{:?}", transaction_info);

                transaction_infos.push(transaction_info);
            }

            return Ok(transaction_infos);
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response.status()
        )
        .into());
    }

    Ok(Vec::new())
}
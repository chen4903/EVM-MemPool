#![allow(dead_code)]
use std::error::Error;
use reqwest::get;
use serde::{Deserialize, Serialize};
use crate::utils::tools;

/// @dev：Used to parse the data returned by ETHERSCAN
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub input: String,
    pub methodId: String
}

pub struct Fetch {
    API_KEY: String,
}

impl Fetch {
    /// @param api_key Etherscan API kEY
    pub fn new(api_key: String) -> Self {
        Fetch { API_KEY: api_key }
    }

    /// @dev Obtain all transactions for a certain address, including normal transactions and internal transactions
    /// @param address The address's txs you fetch
    /// @param start_block The blocko fetch txs from
    /// @param end_block The blocko fetch txs to
    /// @return A vector of txs
    pub async fn fetch_address_all_txs(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
    ) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
        let mut transaction_infos = Vec::new();
        
        let url_normal = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
            address, 
            start_block, 
            end_block, 
            self.API_KEY.clone()
        );
        let url_internal = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
            address, 
            start_block, 
            end_block, 
            self.API_KEY.clone()
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
                    let methodId = transaction["methodId"].as_str().unwrap().to_string();

                    let transaction_info = TransactionInfo {
                        hash,
                        from,
                        to,
                        value,
                        input,
                        methodId
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
                    let methodId = transaction["methodId"].as_str().unwrap().to_string();

                    let transaction_info = TransactionInfo {
                        hash,
                        from,
                        to,
                        value,
                        input,
                        methodId
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


    /// @dev Check that if an address is invoke to mixing service
    /// @param address The address's txs you fetch
    /// @param start_block The blocko fetch txs from
    /// @param end_block The blocko fetch txs to
    /// @return True or false
    pub async fn is_invoke_mixing_service(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
    ) -> Result<bool, Box<dyn Error>>{
        let mut is_invoke = false;
        
        let url_normal = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
            address, 
            start_block, 
            end_block, 
            self.API_KEY.clone()
        );
        let url_internal = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
            address, 
            start_block, 
            end_block, 
            self.API_KEY.clone()
        );

        let response_normal = get(&url_normal).await?;
        let response_internal = get(&url_internal).await?;

        let addresses = tools::get_db_address("mixing_service");

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

    /// @dev Obtain normal transactions for a certain address
    /// @param address The address's txs you fetch
    /// @param start_block The blocko fetch txs from
    /// @param end_block The blocko fetch txs to
    /// @return A vector of txs
    pub async fn fetch_address_normal_txs(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
    ) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
        
        let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
            address, 
            start_block, 
            end_block, 
            self.API_KEY.clone()
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
                    let methodId = transaction["methodId"].as_str().unwrap().to_string();

                    let transaction_info = TransactionInfo {
                        hash,
                        from,
                        to,
                        value,
                        input,
                        methodId
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

    /// @dev Obtain internal transactions for a certain address
    /// @param address The address's txs you fetch
    /// @param start_block The blocko fetch txs from
    /// @param end_block The blocko fetch txs to
    /// @return A vector of txs
    pub async fn fetch_address_internal_txs(
        &self,
        address: &str,
        start_block: u64,
        end_block: u64,
    ) -> Result<Vec<TransactionInfo>, Box<dyn Error>> {
        
        let url = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&address={}&startblock={}&endblock={}&sort=asc&apikey={}",
            address, 
            start_block, 
            end_block, 
            self.API_KEY.clone()
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
                    let methodId = transaction["methodId"].as_str().unwrap().to_string();

                    let transaction_info = TransactionInfo {
                        hash,
                        from,
                        to,
                        value,
                        input,
                        methodId
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
}

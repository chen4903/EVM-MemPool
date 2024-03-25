#![allow(dead_code)]
use std::fs::File;
use std::io::Write;
use std::fs;
use reqwest::get;

/// @dev：
///     TODO：这种分页的合约尚未完成，拉下来需要进一步分开：https://etherscan.io/address/0x80d69e79258FE9D056c822461c4eb0B4ca8802E2#code
///           像这种单页的可以正常拉取下来：0xB20bd5D04BE54f870D5C0d3cA85d82b34B836405
///     获取某个已经verify的合约的solidity源码，默认输出到项目根路径下的output文件夹
/// @param：
///     api_key：区块链浏览器的API接口，ETHERSCAN_API_KEY
///     address：你要查询的地址
pub async fn get_contract_solidity_code(
    api_key: String,
    address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let url = format!("https://api.etherscan.io/api?module=contract&action=getsourcecode&address={}&apikey={}",
        address, 
        api_key
    );

    let response = get(&url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json_data: serde_json::Value = serde_json::from_str(&body)?;

        if let Some(contract_details) = json_data["result"].as_array() {

            let SourceCode = contract_details[0]["SourceCode"].as_str().unwrap().to_string();
            let ContractName = contract_details[0]["ContractName"].as_str().unwrap().to_string();
            let CompilerVersion = contract_details[0]["CompilerVersion"].as_str().unwrap().to_string();
            let ConstructorArguments = contract_details[0]["ConstructorArguments"].as_str().unwrap().to_string();

            let content = format!("// address: {}\r\n// version: {}\r\n// constructor arguments: {}\r\n\r\n{}",address, CompilerVersion,ConstructorArguments,SourceCode);

            write_file(ContractName, content);
        }
    } else {
        return Err(format!(
            "HTTP request failed with status code: {}",
            response.status()
        )
        .into());
    }

    Ok(())
}

fn write_file(file_name: String, output: String) {
    let output_dir = "./output";
    // 创建文件夹
    match fs::metadata(output_dir) {
        Ok(metadata) => {
            if metadata.is_dir() {

            } else {
                match fs::create_dir(output_dir) {
                    Ok(_) => {},
                    Err(err) => eprintln!("create output dir fail:{}", err),
                }
            }
        }
        Err(_) => {
            match fs::create_dir(output_dir) {
                Ok(_) => {},
                Err(err) => eprintln!("create output dir fail:{}", err),
            }
        }
    }

    // 写入文件
    let path = format!("./output/{}.sol", file_name);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("create file error: {}", e);
            return;
        }
    };

    // 换行
    match file.write_all(output.replace("\r\n", "\n").as_bytes()) {
        Ok(_) => {},
        Err(e) => eprintln!("write file errer: {}", e),
    }
}
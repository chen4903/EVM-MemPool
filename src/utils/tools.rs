#![allow(dead_code)]
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    io::Write,
    path::Path,
    fs,
};
use reqwest::get;
use serde::{Deserialize, Serialize};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::error::Error;

/// @dev：Used to parse the data For addresses.json
#[derive(Debug, Serialize, Deserialize)]
struct AddressData {
    eth: Data,
    bsc: Data
}

/// @dev：Used to parse the data For addresses.json
#[derive(Debug, Serialize, Deserialize)]
struct Data {
    hacker: Vec<String>,
    protocol: Vec<String>,
    mixing_service: Vec<String>,
    potential_hacker: Vec<String>
}

/// @notice We currently only focus on Ethereum
/// @dev Get the address from db(a json file)
/// @param option: "hacker", "protocol", "mixing_service" or "potential_hacker", the other returns a new vector
pub fn get_db_address(option: &str) -> Vec<String>{
    let file = File::open("src/utils/addresses.json").expect("Failed to open file");
    let reader = BufReader::new(file);

    let json_data: AddressData = serde_json::from_reader(reader).expect("Failed to parse JSON");

    if option == "hacker" {
        return json_data.eth.hacker;
    }else if option == "protocol" {
        return json_data.eth.protocol;
    } else if option == "mixing_service" {
        return json_data.eth.mixing_service;
    } else if option == "potential_hacker" {
        return json_data.eth.potential_hacker;
    } else {
        return Vec::new();
    }
}

/// @dev Send an email
/// @param sender Email from
/// @param receiver The email address to receive
/// @param title The email title
/// @param content The email content
/// @param password Sender's email server password
/// @param smtp_server Email server smtp code
pub fn send_email(
    sender: String,
    receiver: String,
    title: String,
    content: String,
    password: String,
    smtp_server: String
) -> Result<(), Box<dyn Error>> {

    let email = Message::builder()
        .from(sender.parse()?)
        .to(receiver.parse()?)
        .subject(title)
        .body(content)?;

    let creds = Credentials::new(sender, password);

    let mailer = SmtpTransport::relay(smtp_server.as_str())?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => eprintln!("Could not send the email: {:?}", e),
    }

    Ok(())
}

pub fn write_addresses_db(address: String) {
    let json_file_path = Path::new("src/utils/addresses.json");
    let file = File::open(json_file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut data: AddressData = serde_json::from_reader(reader).expect("Failed to parse JSON");

    data.eth.potential_hacker.push(address);

    let new_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(json_file_path)
        .expect("Failed to open file for writing");
    let mut writer = BufWriter::new(new_file);

    serde_json::to_writer_pretty(&mut writer, &data).expect("Failed to write JSON to file");
}

/// @notice This function is not complete yet
/// @dev Obtain the solidity source code of a verified contract and output it to the output folder
/// @param api_key ETHERSCAN API KEY
/// @param address Which contract address' sourcecode you want to get 
/// @TODO A single page like this can be pulled down normally: 0xB20bd5D04BE54f870D5C0d3cA85d82b34B836405.
///       But this type of paginated contract is not yet completed and needs to be further separated when 
///       pulled down: https://etherscan.io/address/0x80d69e79258FE9D056c822461c4eb0B4ca8802E2#code
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

/// @dev Write a file
/// @param file_name File name
/// @param output The output path
fn write_file(file_name: String, output: String) {
    let output_dir = "./output";
    // Create a new file
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

    // write file
    let path = format!("./output/{}.sol", file_name);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("create file error: {}", e);
            return;
        }
    };

    match file.write_all(output.replace("\r\n", "\n").as_bytes()) {
        Ok(_) => {},
        Err(e) => eprintln!("write file errer: {}", e),
    }
}
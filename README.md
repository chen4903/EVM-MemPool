# SecHelper

We have [Chinese Version](https://github.com/chen4903/SecHelper/blob/master/README_CN.md).

## Brief

A tool for assisting in monitoring, analyzing, and alerting blockchain security threats.

## Prerequisites

Rust

## TODO

- [x] Obtain all transactions for a certain address.
- [x] Check if there are any related mixing service transactions at a certain address.
- [x] Monitor the interaction of a certain contract, and send an email to notify the user if there is hacker interaction (confirmed transaction).
- [x] Monitor the mixing service address and record the user addresses it interacts with, which may be the addresses of hackers who are about to launch an attack.
- [x] Access ChatGPT API, users can inquire to obtain relevant security advice.
- [x] Monitor for any abnormal transactions in a certain contract and notify users via email
  - [x] Pool：If the latest 30 transactions all involve removing liquidity;
  - [ ] TODO
- [ ] 

## Usage

> Before using, you need to configure the `.env` file first.

### execute

guardian

- `message_robot()`: Create a robot to monitor the address m, and send email to receiver when the m has action.
- `warning_robot()`: Create a robot to monitor the address m, and send email to receiver when the m has too many certain tx. Check each 30 seconds and the newest 240 blocks. 

### listener

fetcher

- `fetch_address_all_txs()`: Obtain all transactions for a certain address, including normal transactions and internal transactions.
- `fetch_address_normal_txs()`: Obtain normal transactions for a certain address.
- `fetch_address_internal_txs()`: Obtain internal transactions for a certain address.
- `is_invoke_mixing_service()`: Check that if an address is invoke to mixing service.

listen

- `monitor_mixing_service()`: Monitor mixing service, record the users who interact with it.
- `subscribe_address()`: Subscribe a certain address's all new txs.
- `subscribe_event()`: Monitor a certain address if it has a certain action

### utils

tools

- `get_contract_solidity_code()`: Obtain the solidity source code of a verified contract and output it to the output folder. (Not complete).
- `send_email()`: Send an email.
- `function_sig()`: Get a function's signature

### ai

chatgpt

- `chatgpt()`: Consult ChatGPT for security issues and listen to its security recommendations.




























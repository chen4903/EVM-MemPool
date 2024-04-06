# SecHelper

## Brief

一个用于辅助监控、分析、预警区块链安全威胁的工具。

## Prerequisites

你需要有Rust环境

## TODO

- [x] 查询某个地址的所有交易。
- [x] 查询某个地址是否有相关混币器交易。
- [x] 监控某个合约的交互情况，如果有黑客交互(已经确认交易)，则发邮件通知用户。
- [x] 监控混币器发送给用户的地址，这些地址可能是将来用来发起攻击、部署钓鱼合约的地址。
- [x] 接入ChatGPT的API，用户可以询问来获取相关的安全建议。
- [ ] 监控某个合约是否有异常交易，并发邮件通知用户
  - [ ] 池子：如果最新的30笔的交易都是移除流动性；
  - [ ] TODO


## Usage

> 在使用之前，你需要配置`.env`文件先。

### execute

guardian

- `message_robot()`：监听某个地址的行为，如果有交易，则发出email通知。

### listener

fetcher

- `fetch_address_all_txs`()：获得某个地址的所有交易，包括普通交易、内部交易。
- `fetch_address_normal_txs()`：获得某个地址的普通交易。
- `fetch_address_internal_txs()`：获得某个地址的内部交易。
- `is_invoke_mixing_service()`：查询某个地址是否有相关混币器交易。

listen

- `monitor_mixing_service()`：监控存钱进混币器的用户，记录下来，他们可能是未来的黑客。
- `subscribe_address()`: 监听某个地址的所有交易。
- `subscribe_erc20_transfer()`: 监听某个地址的所有ERC20交易。

### utils

tools

- `get_contract_solidity_code()`：获取某个已经verify的合约的solidity源码，默认输出到项目根路径下的output文件夹，尚未完成。
- `send_email()`：发送邮件给用户。

### ai

chatgpt

- `chatgpt()`：向ChatGPT咨询安全问题，听取它的安全建议。


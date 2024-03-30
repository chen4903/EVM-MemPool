# SecHelper

## Brief

一个用于辅助监控、分析、预警区块链安全威胁的工具

## TODO

- [x] 查询某个地址的所有交易
- [x] 查询某个地址是否有相关混币器交易
- [ ] 监控某个合约的交互情况，如果有黑客交互(已经确认交易)，则发邮件通知用户，
  - [ ] 使用订阅的方式实现，也许是用subscribe_logs, subscribe某个合约地址...
  - [ ] 收集主流合约的地址：比如AAVE、Compound、1Inch等
  - [x] 收集黑客地址
  - [ ] 发邮件给用户
- [ ] 监控混币器发送给用户的地址，这些地址可能是将来用来发起攻击、部署钓鱼合约的地址
  - [ ] 将获取到的地址存放到`addressses._details.json`文件的`potential_hacker`字段

- [ ] 接入ChatGPT的API，用户可以询问来获取相关的安全建议
- [ ] 监控是否有黑客在给地址投毒
  - [ ] 订阅USDT的转账交易
  - [ ] 如果是转账0U，则是地址投毒
  - [ ] 记录下黑客的地址和被投毒的地址到utils文件夹下面的`addresses_poisoning.json`文件中



## Usage

### execute

- 

### listener

- `fetch_address_all_txs`()：获得某个地址的所有交易，包括普通交易、内部交易
- `fetch_address_normal_txs()`：获得某个地址的普通交易
- `fetch_address_internal_txs()`：获得某个地址的内部交易
- `is_invoke_mixing_service()`：查询某个地址是否有相关混币器交易

### utils

- `get_contract_solidity_code()`：获取某个已经verify的合约的solidity源码，默认输出到项目根路径下的output文件夹。TODO【这种[分页](https://etherscan.io/address/0x80d69e79258FE9D056c822461c4eb0B4ca8802E2#code)的合约尚未完成，拉下来需要进一步分开。像这种[单页](https://etherscan.io/address/0xB20bd5D04BE54f870D5C0d3cA85d82b34B836405#code)的可以正常拉取下来】




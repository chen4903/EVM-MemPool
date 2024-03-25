# EVM Pool

## Brief

监听EVM的MemPool，并做一些有趣的事情

## TODO

- [x] 查询某个地址的所有交易
- [ ] 实时监控Memory Pool，获取最新Pending中的交易，解析出来对比是不是目标地址的交易。注意：这需要用到高并发，因为获取Pending交易的速度比解析的速度快；
- [ ] 监控某个合约的交互情况，如果有黑客交互，则用发邮件通知用户
  - [ ] Memory Pool监控合约交互情况
  - [ ] 收集主流合约的地址：比如AAVE、Compound、1Inch等
  - [ ] 收集黑客地址
  - [ ] 发邮件给用户

- [ ] 监听已知的黑客地址，监控其动向，需要维护地址列表；
  - [ ] Memory Pool监控合约交互情况
  - [ ] 收集黑客地址


## Usage

### execute

- 

### listener

- `listen_analysis_all_pool()`：实时获取mempool池子的所有hash，并且解析hash。TODO【还存在问题】
- `fetch_address_all_txs`()：获得某个地址的所有交易，包括普通交易、内部交易
- `fetch_address_normal_txs()`：获得某个地址的普通交易
- `fetch_address_internal_txs()`：获得某个地址的内部交易

### utils

- `get_contract_solidity_code()`：获取某个已经verify的合约的solidity源码，默认输出到项目根路径下的output文件夹。TODO【这种[分页](https://etherscan.io/address/0x80d69e79258FE9D056c822461c4eb0B4ca8802E2#code)的合约尚未完成，拉下来需要进一步分开。像这种[单页](https://etherscan.io/address/0xB20bd5D04BE54f870D5C0d3cA85d82b34B836405#code)的可以正常拉取下来】




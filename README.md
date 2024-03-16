# EVM Pool

## Brief

Monitor EVM's MemPool and do some funny things

## TODO

- [ ] Monitor known hacker addresses, monitor their movements, and maintain an address list;
- [ ] Check if there are any front-run transactions;
- [ ] Always monitor your collateral assets, and if there is a significant fluctuation in their value, they will be liquidated. Our goal is to monitor the transaction that liquidating our assets, and then front-run the transaction that adds collateral before the liquidating transaction;

## Usage

cargo run
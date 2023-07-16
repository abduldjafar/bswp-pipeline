# BSWP-PIPELINE

## Use Case
spin up some kind of pipeline that will index all of the sweep for Stakehouse validators and produce a sum of accurate earnings

## Problem Solving
1. base on this[Documentation](https://docs.joinstakehouse.com/protocol/learn/StakehouseSubgraph) we can create simple query in graphql for getting all public keys.
```
    {
    stakehouseAccounts(first:1000,skip:0) {
        id
    }
    }
```
we can loop use this query until the result is empty.
2. After we get the public keys. we can use this [api](https://quicknode-private.com/token/eth/v1/beacon/states/finalized/validators/) for getting validators information.
3. next base on this[Documentation](https://kb.beaconcha.in/glossary#validator). we can use this formula ```total_eth_obtained = 32 - current_balance (in ETH)```

## Coding
I use rust because currently i'm learning this language since one week ago. the resut example already saved in ```data``` folders
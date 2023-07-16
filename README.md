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
   
4. next base on this [Documentation](https://kb.beaconcha.in/glossary#validator). we can use this formula ```total_eth_obtained = 32 - current_balance (in ETH)```
<img width="1018" alt="Screenshot 2023-07-16 at 18 05 55" src="https://github.com/kotekaman/bswp-pipeline/assets/26897306/277a4af2-65af-409b-a54a-bfd53c55d4f9">


## Coding
I use rust because currently i'm learning this language since one week ago. the resut example already saved in ```data``` folders

## How To use 
1. install rust with following this [instruction](https://www.rust-lang.org/tools/install)
2. Download this repo
3. enter this repo through terminal/command prompt
4. run ```cargo build```
5. run  ```./target/debug/bswp-pipeline --help``` for seing the details
6. run   ```./target/debug/bswp-pipeline``` if want to get all datas. but it will take time.
## Limitation
Currently because the access to sss still free plan so we can use multi threading for process the datas
<img width="1019" alt="Screenshot 2023-07-16 at 18 12 36" src="https://github.com/kotekaman/bswp-pipeline/assets/26897306/d2564b30-ba88-41ea-acf8-30e3f4786766">


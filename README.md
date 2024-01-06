# IDO-TIER

cargo wasm

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11

oraid query staking validators -> oraivaloper1f9judw4xg7d8k4d4ywgz8wsxvuesur739sr88g

RES=$(oraid tx wasm store artifacts/tier.wasm --from $KEY_NAME --gas auto --gas-adjustment 1.3 -y --home $ORAI_HOME_DIR)

TIER_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')

echo $TIER_CODE_ID

oraid tx wasm instantiate "$TIER_CODE_ID"                                  \
    '{
        "validators": [{  
          "address": "oraivaloper1f9judw4xg7d8k4d4ywgz8wsxvuesur739sr88g", 
          "weight": "100"  
        }],  
        "oraiswap_contract": {  
          "usdt_contract": "orai1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrsrpzcas",  
          "orai_contract": "orai1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuqawp9hn"
        },
        "deposits": ["100", "50", "10", "1"],
        "admin":"'"${WALLET_ADDRESS}"'"
    }'                                               \
    --gas auto                                    \
    --gas-adjustment 1.1          \
    --gas-prices 0.1orai    \
    --no-admin     \
    --from "$WALLET"                                 \
    --label "$TIER_LABEL"                            \
    --home $ORAI_HOME_DIR -y

orai1f8rp2vlg5s7pvyfcnpfms007uyl7856kpe2dcx8hjq2fkt90qfjslfal29

oraid q wasm contract-state smart "$TIER_CONTRACT" '{ "user_info": {"address":"'"$WALLET_ADDRESS"'"} }'

oraid tx wasm execute "$TIER_CONTRACT" '{ "deposit": {} }' \
  --gas auto                                    \
    --gas-adjustment 1.1          \
    --gas-prices 0.1orai     \
    --from "$WALLET"                                 \
    --amount 2000000orai                            \
    --yes
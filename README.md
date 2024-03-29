# IDO-TIER

cargo wasm

docker run --rm -v "$(pwd)":/code \
 --mount type=volume,source="$(basename "$(pwd)")\_cache",target=/code/target \
 --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
 cosmwasm/rust-optimizer:0.12.11

RES=$(oraid tx wasm store artifacts/tier.wasm --from $KEY_NAME --gas auto --gas-adjustment 1.3 -y --home $ORAI_HOME_DIR)

TIER_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')

echo $TIER_CODE_ID

RES=$(oraid tx wasm instantiate "$TIER_CODE_ID" \
 '{
"validators": [{
"address": "oraivaloper18hr8jggl3xnrutfujy2jwpeu0l76azprkxn29v",
"weight": "100"
}],  
 "oraiswap_contract": {  
 "usdt_contract": "orai1laj3d4zledty0r0vd7m3gem4cd7cyk09m608863p3t7p6sm6xmusru4l5p",  
 "orai_swap_router_contract": "orai1e4k9zhjpz3a0q6fgspwj3ug5fhc3t2emtxuvtra79hs70gqq7m0sg8kdd5"
},
"deposits": ["25000", "7500", "1500", "250"],
"admin":"'"${WALLET_ADDRESS}"'"
}' \
 --gas auto \
 --gas-adjustment 1.1 \
 --gas-prices 0.1orai \
 --no-admin \
 --from ttt \
 --label "YOUI_ORAI" \
 --home ~/.oraid -y)

TIER_CONTRACT=$(echo $RES | jq -r '.logs[0].events[0].attributes[0].value')

oraid q wasm contract-state smart "$TIER_CONTRACT" '{ "config": {}}' --home $ORAI_HOME_DIR

oraid tx wasm execute "$TIER_CONTRACT" '{ "deposit": {} }' \
  --gas auto                                    \
    --gas-adjustment 1.1          \
    --from "$KEY_NAME" \
 --amount 2000000orai \
 --home $ORAI_HOME_DIR --yes

oraid tx wasm execute "$TIER_CONTRACT" '{ "withdraw": {} }' \
  --gas auto                                    \
    --gas-adjustment 1.1          \
    --from "$KEY_NAME" \
 --home $ORAI_HOME_DIR --yes

oraid q wasm contract-state smart "$TIER_CONTRACT" '{ "user_info": {"address":"'"$WALLET_ADDRESS"'"} }' --home $ORAI_HOME_DIR

oraid q bank balances $(oraid keys show $KEY_NAME -a --home $ORAI_HOME_DIR ) --home $ORAI_HOME_DIR

oraid q wasm contract-state smart "orai1vhndln95yd7rngslzvf6sax6axcshkxqppr886ntelh28p9ghuqawp9hn" \
'{
"simulate_swap_operations": {
"offer_amount": "1000000",
"operations": [
{
"orai_swap": {
"offer_asset_info": {
"native_token": {
"denom": "orai"
}
},
"ask_asset_info": {
"token": {
"contract_addr": "orai1sthrn5ep8ls5vzz8f9gp89khhmedahhdqd244dh9uqzk3hx2pzrsrpzcas"
}
}
}
}
]
}
}' --home $ORAI_HOME_DIR

oraid query staking validators -> oraivaloper1f9judw4xg7d8k4d4ywgz8wsxvuesur739sr88g

oraid tx wasm execute orai1rmqthdyqs03faugttrt6kkm2u73pwfr86aqe6s6jvlwq4wqdp6vqsve3m4 \
'{
"mint": {
"owner": "orai1raykmpd688azm9zeagtytnal70fswvdp224qvy",
"token_id": "103",
"extension": {
"attributes": [
{
"trait_type": "id",
"value": "XYZB"
}
]
}
}
}' \
--from $KEY_NAME --gas auto --gas-adjustment 1.1 -y --home $ORAI_HOME_DIR

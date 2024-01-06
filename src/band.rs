use cosmwasm_std::{Decimal, StdResult};

use cosmwasm_std::DepsMut;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Config;

pub struct OraiPriceOracle {
    value: u128,
    flag: bool,
}

impl OraiPriceOracle {
    pub const DECIMALS: u8 = 18;
    pub const ONE_USD: u128 = 1_000_000_000_000_000_000;

    pub fn new(deps: &DepsMut) -> StdResult<Self> {
        let config = Config::load(deps.storage)?;
        let orai_swap_router_contract = config.oraiswap_contract.orai_swap_router_contract;
        let native_token = NativeToken::new("orai".to_string());
        let offer_asset_info = OfferAssetInfo::new(native_token);
        let usdt_contract_address = config.oraiswap_contract.usdt_contract;
        let msg = SwapContractMessage {
            simulate_swap_operations: SwapContractMessageContent {
                offer_amount: 1000000,
                operations: vec![Operation {
                    orai_swap: OraiSwap {
                        offer_asset_info: offer_asset_info,
                        ask_asset_info: AskAssetInfo {
                            token: UsdtContractAddr {
                                contract_addr: usdt_contract_address,
                            },
                        },
                    },
                }],
            },
        };
        let response: ExchangeRateResponse = deps
            .querier
            .query_wasm_smart(orai_swap_router_contract, &msg)?;
        let exchange_rate = response.amount;
        let mut flag = false;
        let mut value = exchange_rate;
        if exchange_rate < 1000000 {
            flag = true;
            value = (Decimal::raw(1000000u128) / Decimal::raw(exchange_rate))
            .to_uint_floor()
            .u128();
        } else {
            value = (Decimal::raw(exchange_rate) / Decimal::raw(1000000u128))
            .to_uint_floor()
            .u128();
        }
        
        Ok(OraiPriceOracle { value, flag })
    }

    pub fn usd_amount(&self, orai: u128) -> u128 {
        if self.flag == true {
            orai.checked_mul(self.value)
            .and_then(|v| v.checked_div(OraiPriceOracle::ONE_USD))
            .unwrap()
        } else {
            orai.checked_mul(OraiPriceOracle::ONE_USD)
            .and_then(|v: u128| v.checked_div(self.value))
            .unwrap()
        }
        
    }

    pub fn orai_amount(&self, usd: u128) -> u128 {
        if self.flag == true {
            usd.checked_mul(OraiPriceOracle::ONE_USD)
            .and_then(|v: u128| v.checked_div(self.value))
            .unwrap()
        } else {
            usd.checked_mul(self.value)
            .and_then(|v| v.checked_div(OraiPriceOracle::ONE_USD))
            .unwrap()
        }
        
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
// Define the struct for the innermost part "native_token"
pub struct NativeToken {
    denom: String,
}

impl NativeToken {
    pub fn new(native_token_denom: String) -> Self {
        NativeToken {
            denom: native_token_denom,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
// Define the struct associated with "offer_asset_info"
pub struct OfferAssetInfo {
    native_token: NativeToken,
}

impl OfferAssetInfo {
    pub fn new(native_token: NativeToken) -> Self {
        OfferAssetInfo {
            native_token: native_token,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsdtContractAddr {
    contract_addr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AskAssetInfo {
    pub token: UsdtContractAddr,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
// Define the struct represented by the "orai_swap" key
pub struct OraiSwap {
    offer_asset_info: OfferAssetInfo,
    ask_asset_info: AskAssetInfo,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Operation {
    pub orai_swap: OraiSwap,
}

impl Clone for Operation {
    fn clone(&self) -> Operation {
        Operation {
            orai_swap: self.orai_swap.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SwapContractMessageContent {
    pub offer_amount: u128,
    pub operations: Vec<Operation>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SwapContractMessage {
    pub simulate_swap_operations: SwapContractMessageContent,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Amount {
    amount: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExchangeRateResponse {
    pub amount: u128,
}

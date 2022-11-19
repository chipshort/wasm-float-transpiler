#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CalcResult, ExecuteMsg, InstantiateMsg, QueryMsg};
use std::str::FromStr;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:test-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Calculate { a, b } => {
            // reinterpret values as f32
            let a = f32::from_str(&a)
                .map_err(|_| StdError::generic_err("Error parsing a as a float"))?;
            let b = f32::from_str(&b)
                .map_err(|_| StdError::generic_err("Error parsing b as a float"))?;

            // do some float math with them
            let result = (a * b + 1.7234f32) / 4.8 - 5.6;

            // now convert back and return it
            let converted_result = result.to_bits();

            Ok(to_binary(&CalcResult {
                bits: converted_result,
                stringified: format!("{}", result),
            })?)
        }
    }
}

#[cfg(test)]
mod tests {}

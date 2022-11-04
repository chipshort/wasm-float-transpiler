use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CalcResult)]
    Calculate { a: String, b: String },
}

#[cw_serde]
pub struct CalcResult {
    pub bits: u32,
    pub stringified: String,
}

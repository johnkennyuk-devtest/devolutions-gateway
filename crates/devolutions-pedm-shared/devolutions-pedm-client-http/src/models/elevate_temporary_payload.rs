/*
 * Devolutions PEDM API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document:
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElevateTemporaryPayload {
    /// The number of seconds to elevate the user for.  This must be between 1 and `i32::MAX`.
    #[serde(rename = "Seconds")]
    pub seconds: u64,
}

impl ElevateTemporaryPayload {
    pub fn new(seconds: u64) -> ElevateTemporaryPayload {
        ElevateTemporaryPayload { seconds }
    }
}

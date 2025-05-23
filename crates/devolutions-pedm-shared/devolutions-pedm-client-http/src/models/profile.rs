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
pub struct Profile {
    #[serde(rename = "DefaultElevationKind")]
    pub default_elevation_kind: models::ElevationKind,
    #[serde(rename = "ElevationMethod")]
    pub elevation_method: models::ElevationMethod,
    #[serde(rename = "ElevationSettings")]
    pub elevation_settings: models::ElevationConfigurations,
    #[serde(rename = "Id")]
    pub id: uuid::Uuid,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PromptSecureDesktop")]
    pub prompt_secure_desktop: bool,
    #[serde(rename = "TargetMustBeSigned")]
    pub target_must_be_signed: bool,
}

impl Profile {
    pub fn new(
        default_elevation_kind: models::ElevationKind,
        elevation_method: models::ElevationMethod,
        elevation_settings: models::ElevationConfigurations,
        id: uuid::Uuid,
        name: String,
        prompt_secure_desktop: bool,
        target_must_be_signed: bool,
    ) -> Profile {
        Profile {
            default_elevation_kind,
            elevation_method,
            elevation_settings,
            id,
            name,
            prompt_secure_desktop,
            target_must_be_signed,
        }
    }
}

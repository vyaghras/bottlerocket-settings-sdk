//! Settings related to bootstrap containers.
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::Url;
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use std::convert::Infallible;

#[model(impl_default = true)]
pub struct BottlerocketBootstrapSettingsV1 {
    pub source: Url,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for BottlerocketBootstrapSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that parses as BottlerocketBootstrapSettingsV1.
        Ok(())
    }

    fn generate(
        existing_partial: Option<Self::PartialKind>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialKind, Self>> {
        Ok(GenerateResult::Complete(
            existing_partial.unwrap_or_default(),
        ))
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<()> {
        // Validate anything that parses as BottlerocketBootstrapSettingsV1.
        Ok(())
    }
}

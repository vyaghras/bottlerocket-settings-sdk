//! Settings related to bootstrap containers.
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{BootstrapMode, Identifier, Url, ValidBase64};
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, convert::Infallible};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BootstrapContainersSettingsV1 {
    pub bootstrap_containers: HashMap<Identifier, BootstrapContainer>,
}

// Custom serializer/deserializer added to maintain backwards
// compatibility with models created prior to settings extensions.
impl Serialize for BootstrapContainersSettingsV1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bootstrap_containers.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BootstrapContainersSettingsV1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bootstrap_containers = HashMap::deserialize(deserializer)?;
        Ok(Self {
            bootstrap_containers,
        })
    }
}

#[model(impl_default = true)]
struct BootstrapContainer {
    source: Url,
    mode: BootstrapMode,
    user_data: ValidBase64,
    essential: bool,
}

#[model(impl_default = true)]
pub struct BootstrapContainersSettingsV2 {
    pub default: Url, // Change this to Url
    pub definitions: HashMap<Identifier, BootstrapContainerV2>,
}

#[model(impl_default = true)]
struct BootstrapContainerV2 {
    source: Option<Url>,
    mode: BootstrapMode,
    user_data: ValidBase64,
    essential: bool,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for BootstrapContainersSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that parses as BootstrapContainersSettingsV1.
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
        // Validate anything that parses as BootstrapContainersSettingsV1.
        Ok(())
    }
}

impl SettingsModel for BootstrapContainersSettingsV2 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v2"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that parses as BootstrapContainersSettingsV2.
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
        // Validate anything that parses as BootstrapContainersSettingsV2.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_bootstrap_container_settings() {
        let generated = BootstrapContainersSettingsV1::generate(None, None).unwrap();

        assert_eq!(
            generated,
            GenerateResult::Complete(BootstrapContainersSettingsV1 {
                bootstrap_containers: HashMap::new(),
            })
        )
    }

    #[test]
    fn test_serde_bootstrap_container() {
        let test_json = json!({
            "mybootstrap": {
                "source": "uri.to.container.in.oci-compatible-registry.example.com/foo:1.0.0",
                "mode": "once",
                "user-data": "dXNlcmRhdGE=",
                "essential": true,
            }
        });

        let test_json_str = test_json.to_string();

        let bootstrap_containers: BootstrapContainersSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        let mut expected_bootstrap_container: HashMap<Identifier, BootstrapContainer> =
            HashMap::new();
        expected_bootstrap_container.insert(
            Identifier::try_from("mybootstrap").unwrap(),
            BootstrapContainer {
                source: Some(
                    Url::try_from(
                        "uri.to.container.in.oci-compatible-registry.example.com/foo:1.0.0",
                    )
                    .unwrap(),
                ),
                mode: Some(BootstrapMode::try_from("once").unwrap()),
                user_data: Some(ValidBase64::try_from("dXNlcmRhdGE=").unwrap()),
                essential: Some(true),
            },
        );

        assert_eq!(
            bootstrap_containers,
            BootstrapContainersSettingsV1 {
                bootstrap_containers: expected_bootstrap_container
            }
        );

        let serialized_json: serde_json::Value = serde_json::to_string(&bootstrap_containers)
            .map(|s| serde_json::from_str(&s).unwrap())
            .unwrap();

        assert_eq!(serialized_json, test_json);
    }
}

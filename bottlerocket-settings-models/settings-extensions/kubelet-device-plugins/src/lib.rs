//! Settings related to Kubelet Device Plugins
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::NvidiaDevicePluginSettings;
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use std::convert::Infallible;

#[model(impl_default = true)]
pub struct KubeletDevicePluginsV1 {
    nvidia: NvidiaDevicePluginSettings,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for KubeletDevicePluginsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that can be parsed as ECSSettingsV1.
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
        // KubeletDevicePluginsV1 is validated during deserialization.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bottlerocket_modeled_types::{NvidiaDeviceIdStrategy, NvidiaDeviceListStrategy};

    #[test]
    fn test_generate_kubelet_device_plugins() {
        let generated = KubeletDevicePluginsV1::generate(None, None).unwrap();
        assert_eq!(
            generated,
            GenerateResult::Complete(KubeletDevicePluginsV1 { nvidia: None })
        );
    }

    #[test]
    fn test_serde_kubelet_device_plugins() {
        let test_json = r#"{"nvidia":{"pass-device-specs":true,"device-id-strategy":"index","device-list-strategy":"volume-mounts"}}"#;

        let device_plugins: KubeletDevicePluginsV1 = serde_json::from_str(test_json).unwrap();
        assert_eq!(
            device_plugins,
            KubeletDevicePluginsV1 {
                nvidia: Some(NvidiaDevicePluginSettings {
                    pass_device_specs: Some(true),
                    device_id_strategy: Some(NvidiaDeviceIdStrategy::Index),
                    device_list_strategy: Some(NvidiaDeviceListStrategy::VolumeMounts),
                }),
            }
        );

        let results = serde_json::to_string(&device_plugins).unwrap();
        assert_eq!(results, test_json);
    }
}

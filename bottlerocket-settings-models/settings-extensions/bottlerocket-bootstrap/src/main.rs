use bottlerocket_settings_sdk::{BottlerocketSetting, NullMigratorExtensionBuilder};
use settings_extension_bottlerocket_bootstrap::BottlerocketBootstrapSettingsV1;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match NullMigratorExtensionBuilder::with_name("bottlerocket-bootstrap")
        .with_models(vec![
            BottlerocketSetting::<BottlerocketBootstrapSettingsV1>::model(),
        ])
        .build()
    {
        Ok(extension) => extension.run(),
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}

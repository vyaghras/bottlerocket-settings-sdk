/*!
This crate defines the FFI specification for Bottlerocket settings plugins, as well as some helper
functions.
*/

// Avoid empty doc comment warning that originates from the StableAbi derive macro.
#![allow(clippy::empty_docs)]
// Avoid false positive improper ctypes warnings for abi_stable's PhantomData markers. We rely on
// the StableAbi trait to catch any real problems.
#![allow(improper_ctypes_definitions)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as JsonValue;
use std::path::PathBuf;

use abi_stable::{
    erased_types::{DeserializeDyn, DynTrait, SerializeProxyType},
    external_types::{RawValueBox, RawValueRef},
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{RBox, RBoxError, RErr, ROk, RResult, RStr},
    StableAbi,
};

use lazy_static::lazy_static;

const SETTINGS: &str = "settings";

/// Plugins need to provide "default" and "deserialize" functions that return an instance of the
/// opaque BottlerocketSettingsProvider wrapper type. These are the only way for the host program
/// to construct new instances of the underlying concrete type.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = BottlerocketSettingsPluginRef)))]
#[sabi(missing_field(panic))]
pub struct BottlerocketSettingsPlugin {
    /// Returns a BottlerocketSettingsProvider that wraps a new instance of the underlying type
    /// which was created with default values.
    pub default_settings: extern "C" fn() -> BottlerocketSettingsProvider,

    #[sabi(last_prefix_field)]
    /// Returns a BottlerocketSettingsProvider that wraps a new instance of the underlying type
    /// which was created by deserializing the supplied string.
    pub deserialize_settings:
        for<'a> extern "C" fn(RStr<'a>) -> RResult<BottlerocketSettingsProvider, RBoxError>,
}

/// These values will be checked at runtime to ensure that the host program and the plugin agree
/// on the name and version of the expected interface.
impl RootModule for BottlerocketSettingsPluginRef {
    const BASE_NAME: &'static str = SETTINGS;
    const NAME: &'static str = SETTINGS;
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
    abi_stable::declare_root_module_statics! {BottlerocketSettingsPluginRef}
}

// Shared library plugins should only be loaded once, cannot be unloaded, and might not be safe to
// try loading again if the first try fails. Whatever result we get from this attempt is what we'll
// live with.
lazy_static! {
    static ref PLUGIN: BottlerocketSettingsPluginRef = {
        match BottlerocketSettingsPluginRef::load_from_file(&PathBuf::from(format!(
            "lib{}.{}",
            BottlerocketSettingsPluginRef::NAME,
            std::env::consts::DLL_EXTENSION,
        ))) {
            Ok(r) => r,
            Err(e) => {
                panic!("Fatal error when loading settings plugin: {e}");
            }
        }
    };
}

/// Provide an interface to load the settings plugin dynamically the first time it's required.
/// Panics if the plugin cannot be loaded. This simplifies loading the plugin since programs do
/// not need to arrange to call an initialization function before the first call to a nominally
/// infallible trait impl like Default.
impl BottlerocketSettingsPluginRef {
    pub fn load() {
        let _ = *PLUGIN;
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// Specify all required impls for the wrapped type that will be required by the plugin.
#[repr(C)]
#[derive(StableAbi)]
#[sabi(impl_InterfaceType(
    Sync,
    Send,
    Default,
    Eq,
    PartialEq,
    Clone,
    Debug,
    Deserialize,
    Serialize
))]
pub struct BottlerocketSettingsInterface;

/// Implement the proxy serialization trait for the wrapped type.
impl<'a> SerializeProxyType<'a> for BottlerocketSettingsInterface {
    // Serialize the type by way of a boxed serde_json raw value.
    type Proxy = RawValueBox;
    // There's no need to load the plugin to serialize the type, because the type can only be
    // instantiated by the default and deserialize functions, which trigger the plugin load.
}

/// Implement the proxy deserialization trait for the wrapped type.
impl<'a> DeserializeDyn<'a, BottlerocketSettingsProvider> for BottlerocketSettingsInterface {
    /// Deserialize the type by way of a serde_json raw value ref.
    type Proxy = RawValueRef<'a>;

    // Load the plugin, then pass the provided input to the deserialize function via FFI.
    fn deserialize_dyn(s: Self::Proxy) -> Result<BottlerocketSettingsProvider, RBoxError> {
        BottlerocketSettingsPluginRef::load();
        BottlerocketSettingsPluginRef::get_module()
            .unwrap()
            .deserialize_settings()(s.get_rstr())
        .into_result()
    }
}

/// Define the boxed wrapper type returned by FFI functions.
pub type BottlerocketSettingsProvider = DynTrait<'static, RBox<()>, BottlerocketSettingsInterface>;

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// The Default trait is already used for `DynTrait`, so add a custom trait to provide the same
/// behavior.
pub trait BottlerocketDefaults: Sized {
    fn defaults() -> Self;
}

/// Implement the custom default trait for the boxed wrapper type.
impl BottlerocketDefaults for BottlerocketSettingsProvider {
    // Load the plugin, then call the defaults function via FFI.
    fn defaults() -> Self {
        BottlerocketSettingsPluginRef::load();
        BottlerocketSettingsPluginRef::get_module()
            .unwrap()
            .default_settings()()
    }
}

// =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=   =^..^=

/// Helper function that plugins can use to implement the deserialize function.
/// This runs on the plugin side of the FFI boundary.
pub fn deserialize_json<'a, T>(s: RStr<'a>) -> RResult<T, RBoxError>
where
    T: serde::Deserialize<'a>,
{
    match serde_json::from_str::<T>(s.into()) {
        Ok(x) => ROk(x),
        Err(e) => RErr(RBoxError::new(e)),
    }
}

/// Helper function that plugins can use to implement the serialize function.
/// This runs on the plugin side of the FFI boundary.
pub fn serialize_json<T>(value: &T) -> Result<RawValueBox, RBoxError>
where
    T: serde::Serialize,
{
    match serde_json::value::to_raw_value::<T>(value) {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(RBoxError::new(e)),
    }
}

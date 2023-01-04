use serde::Deserialize;
use swc_core::plugin::metadata::TransformPluginProgramMetadata as Metadata;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PluginOptions {
    /// custom elements
    pub custom_element_patterns: Vec<String>,
    /// static JSXElementChild hoist threshold
    /// - default: 5
    #[serde(default = "default_static_threshold")]
    pub static_threshold: usize,
}

const fn default_true() -> bool {
    true
}

const fn default_static_threshold() -> usize {
    5
}

impl From<&str> for PluginOptions {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).expect("Error: Invalid Options")
    }
}

impl From<String> for PluginOptions {
    fn from(config: String) -> Self {
        Self::from(config.as_str())
    }
}

impl From<&Metadata> for PluginOptions {
    fn from(metadata: &Metadata) -> Self {
        metadata
            .get_transform_plugin_config()
            .map(Self::from)
            .unwrap_or_default()
    }
}

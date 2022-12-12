use serde::Deserialize;
use swc_core::plugin::metadata::TransformPluginProgramMetadata as Metadata;

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PluginOptions {
    /// transform `on:{click: xx}` to `onClick: xxx`
    pub transform_on: bool,
    /// enable optimization or not.
    pub optimize: bool,
    /// merge static and dynamic class / style attributes / onXXX handlers
    /// - default: true
    #[serde(default = "default_true")]
    pub merge_props: bool,
    /// configuring custom elements
    pub custom_element_patterns: Vec<String>,
    /// enable object slots syntax
    /// - default: true
    #[serde(default = "default_true")]
    pub enable_object_slots: bool,
    /// Replace the function used when compiling JSX expressions
    pub pragma: Option<String>,
}

impl From<&str> for PluginOptions {
    fn from(json: &str) -> Self {
        serde_json::from_str(json).expect("invalid options for Vue JSX")
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

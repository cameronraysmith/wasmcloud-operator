use k8s_openapi::api::core::v1::ResourceRequirements;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Default))]
#[kube(
    kind = "WasmCloudHostConfig",
    group = "k8s.wasmcloud.dev",
    version = "v1alpha1",
    shortname = "whc",
    namespaced,
    status = "WasmCloudHostConfigStatus",
    printcolumn = r#"{"name":"App Count", "type":"integer", "jsonPath":".status.app_count"}"#
)]
#[serde(rename_all = "camelCase")]
pub struct WasmCloudHostConfigSpec {
    /// The number of replicas to use for the wasmCloud host Deployment.
    #[serde(default = "default_host_replicas")]
    pub host_replicas: u32,
    /// A list of cluster issuers to use when provisioning hosts. See
    /// https://wasmcloud.com/docs/deployment/security/zero-trust-invocations for more information.
    pub issuers: Vec<String>,
    /// The lattice to use for these hosts.
    pub lattice: String,
    /// An optional set of labels to apply to these hosts.
    pub host_labels: Option<HashMap<String, String>>,
    /// The version of the wasmCloud host to deploy.
    pub version: String,
    /// The name of a secret containing the primary cluster issuer key along with an optional set
    /// of NATS credentials.
    pub secret_name: String,
    /// Enable structured logging for host logs.
    pub enable_structured_logging: Option<bool>,
    /// Name of a secret containing the registry credentials
    pub registry_credentials_secret: Option<String>,
    /// Kubernetes resources to allocate for the host. See
    /// https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/ for valid
    /// values to use here.
    pub resources: Option<WasmCloudHostConfigResources>,
    /// The control topic prefix to use for the host.
    pub control_topic_prefix: Option<String>,
    /// The leaf node domain to use for the NATS sidecar. Defaults to "leaf".
    #[serde(default = "default_leaf_node_domain")]
    pub leaf_node_domain: String,
    /// Enable the config service for this host.
    #[serde(default)]
    pub config_service_enabled: bool,
    /// The address of the NATS server to connect to. Defaults to "nats://nats.default.svc.cluster.local".
    #[serde(default = "default_nats_address")]
    pub nats_address: String,
    /// The Jetstream domain to use for the NATS sidecar. Defaults to "default".
    #[serde(default = "default_jetstream_domain")]
    pub jetstream_domain: String,
    /// The log level to use for the host. Defaults to "INFO".
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// Run hosts as a DaemonSet instead of a Deployment.
    #[serde(default)]
    pub daemonset: bool,
}

fn default_host_replicas() -> u32 {
    1
}

fn default_jetstream_domain() -> String {
    "default".to_string()
}

fn default_nats_address() -> String {
    "nats://nats.default.svc.cluster.local".to_string()
}

fn default_leaf_node_domain() -> String {
    "leaf".to_string()
}

fn default_log_level() -> String {
    "INFO".to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct WasmCloudHostConfigResources {
    pub nats: Option<ResourceRequirements>,
    pub wasmcloud: Option<ResourceRequirements>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct WasmCloudHostConfigStatus {
    pub apps: Vec<AppStatus>,
    pub app_count: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct AppStatus {
    pub name: String,
    pub version: String,
}

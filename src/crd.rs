use garde::Validate;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Struct corresponding to the Specification (`spec`) part of the `CDBootstrap` resource, directly
/// reflects context of the `cdbootstraps.example.com.yaml` file to be found in this repository.
/// The `CDBootstrap` struct will be generated by the `CustomResource` derive macro.
#[derive(CustomResource, Serialize, Deserialize, Debug, Validate, Clone, Default, JsonSchema)]
#[kube(
    group = "cncp.nl",
    version = "v1beta1",
    kind = "Tagger",
    plural = "taggers",
    namespaced
)]
#[kube(status = "TaggerStatus")]
pub struct TaggerSpec {
    #[garde(skip)]
    pub labels: Vec<Label>,
    #[garde(skip)]
    pub annotations: Vec<Annotation>,
    #[garde(skip)]
    #[serde(rename = "excludeList")]
    pub excludelist: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct Label {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, JsonSchema)]
pub struct Annotation {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, JsonSchema)]
pub struct TaggerStatus {
    pub succeeded: bool,
    pub tagged: Vec<String>,
}

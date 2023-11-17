#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Curve {
  pub fade_in_time: Option<f64>,
  pub fade_out_time: Option<f64>,
  pub id: String,
  pub segments: Vec<f64>,
  pub target: String,
}

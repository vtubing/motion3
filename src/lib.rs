#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Motion3 {
  pub curves: Vec<Curve>,
  pub meta: Meta,
  #[serde(default)]
  pub user_data: Vec<UserData>,
  pub version: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Meta {
  pub are_beziers_restricted: Option<bool>,
  pub curve_count: u64,
  pub duration: f64,
  pub fade_in_time: Option<f64>,
  pub fade_out_time: Option<f64>,
  pub fps: f64,
  #[serde(rename = "Loop")]
  pub loop_: Option<bool>,
  pub total_point_count: u64,
  pub total_segment_count: u64,
  pub total_user_data_size: Option<u64>,
  pub user_data_count: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct UserData {
  pub time: f64,
  pub value: String,
}

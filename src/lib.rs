mod curve;
mod meta;
mod user_data;

pub use curve::Curve;
pub use meta::Meta;
pub use user_data::UserData;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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

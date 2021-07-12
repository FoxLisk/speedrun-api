use serde::Deserialize;

use super::Link;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Genre {
    pub id: String,
    pub name: String,
    pub links: Vec<Link>,
}

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuthor {
    name: String,
    date_of_birth: NaiveDate,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAuthor {
    name: Option<String>,
    date_of_birth: Option<NaiveDate> 
}

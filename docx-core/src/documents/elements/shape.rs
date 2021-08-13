use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Default, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct Shape {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_data: Option<ImageData>,
}
// Experimental, For now reader only.

#[derive(Serialize, Debug, Clone, PartialEq, Default, ts_rs::TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct ImageData {
    pub id: String,
}

impl Shape {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn style(mut self, style: impl Into<String>) -> Self {

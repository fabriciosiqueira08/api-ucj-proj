use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub name: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardNode {
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardEdge {
    pub node: CardNode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cards {
    pub edges: Vec<CardEdge>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Phase {
    pub name: String,
    pub cards: Cards,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PipeResponse {
    pub pipe: Pipe,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pipe {
    pub phases: Vec<Phase>,
}

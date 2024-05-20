use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use crate::types::PipeResponse;

pub async fn fetch_pipefy_data(
    pipe_id: &str,
    cursor: Option<&str>,
    page_size: i32,
    config: &crate::definitions::Config,
) -> Result<PipeResponse, Box<dyn Error>> {
    let cursor_clause = if let Some(cursor) = cursor {
        format!(", after: \"{}\"", cursor)
    } else {
        String::new()
    };

    let query = format!(
        r#"
        query {{
            pipe(id: "{}") {{
                phases {{
                    name
                    cards(first: {}{}) {{
                        edges {{
                            node {{
                                title
                                createdAt
                                fields {{
                                    name
                                    value
                                }}
                            }}
                        }}
                        pageInfo {{
                            hasNextPage
                            endCursor
                        }}
                    }}
                }}
            }}
        }}
        "#,
        pipe_id, page_size, cursor_clause
    );

    let client = Client::new();
    let response = client
        .post(&config.pipefy_graphql_endpoint)
        .header("Authorization", format!("Bearer {}", config.pipefy_api_token))
        .json(&json!({ "query": query }))
        .send()
        .await?;

    if response.status().is_success() {
        let data: HashMap<String, PipeResponse> = response.json().await?;
        if let Some(pipe_response) = data.get("data") {
            Ok(pipe_response.clone())
        } else {
            Err("Erro na resposta da API".into())
        }
    } else {
        let status = response.status();
        let text = response.text().await?;
        println!("Erro na requisição: {} - {}", status, text);
        Err(format!("Erro na requisição: {} - {}", status, text).into())
    }
}

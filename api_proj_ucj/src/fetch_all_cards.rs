use crate::fetch_pipefy_data::fetch_pipefy_data;
use crate::definitions::Config;
use crate::types::{Phase, CardEdge};
use std::error::Error;

pub async fn fetch_all_cards(pipe_id: &str, config: &Config) -> Result<Vec<Phase>, Box<dyn Error>> {
    let mut all_phases: Vec<Phase> = Vec::new();
    let initial_data = fetch_pipefy_data(pipe_id, None, 30, config).await?;

    for mut phase_data in initial_data.pipe.phases {
        let phase_name = phase_data.name.clone();
        let card_count = phase_data.cards.edges.len();
        let mut cards: Vec<CardEdge> = phase_data.cards.edges.clone();
        let mut page_info = phase_data.cards.page_info.clone();
        let mut cursor = page_info.end_cursor.clone();

        while page_info.has_next_page {
            if let Some(cursor_value) = cursor.clone() {
                let more_data = fetch_pipefy_data(pipe_id, Some(&cursor_value), 30, config).await?;
                if let Some(more_phase_data) = more_data.pipe.phases.iter().find(|p| p.name == phase_name) {
                    let more_cards = more_phase_data.cards.edges.clone();
                    cards.extend(more_cards);
                    page_info = more_phase_data.cards.page_info.clone();
                    cursor = page_info.end_cursor.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        phase_data.cards.edges = cards.clone();
        all_phases.push(phase_data);
        println!("Fase '{}' processada com {} cartões.", phase_name, card_count);
    }

    Ok(all_phases)
}

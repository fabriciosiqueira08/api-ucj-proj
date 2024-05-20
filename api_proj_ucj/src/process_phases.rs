use crate::process_card::process_card;
use crate::types::Phase;
use xlsxwriter::Worksheet;
use chrono::NaiveDate;

pub fn process_phases(
    worksheet: &mut Worksheet,
    headers: &[&str],
    all_phases: &[Phase],
    mut row_num: u32,
) -> u32 {
    // Limpar as linhas antigas
    for row in 1..row_num {
        for col in 0..headers.len() as u16 {
            worksheet.write_blank(row as u32, col as u16, None).unwrap();
        }
    }

    // Processar todas as fases
    for phase in all_phases {
        for card_edge in &phase.cards.edges {
            let card = &card_edge.node;
            let field_values = process_card(card, headers);

            let created_at_str = &card.created_at;
            let created_at_formatted = if let Ok(created_at) = NaiveDate::parse_from_str(created_at_str, "%Y-%m-%dT%H:%M:%SZ") {
                created_at.format("%d/%m/%Y").to_string()
            } else {
                created_at_str.to_string()
            };

            worksheet.write_string(row_num, 0, &created_at_formatted, None).unwrap();

            for (col_num, header) in headers.iter().enumerate().skip(1) {
                let cell_value = field_values.get(*header).unwrap_or(&String::new()).clone();
                worksheet.write_string(row_num, col_num as u16, &cell_value, None).unwrap();
            }

            row_num += 1;
        }
    }

    row_num
}

use crate::process_phases::process_phases;
use crate::types::Phase;
use calamine::{open_workbook, Reader};
use chrono::NaiveDate;
use xlsxwriter::prelude::*; // Use a prelude para importar tudo necessário de uma vez

pub fn update_excel(
    workbook: &mut Workbook,
    all_phases: &[Phase],
    sheet_name: &str,
) {
    // Obter ou criar a aba (worksheet) no workbook
    let mut worksheet = workbook.add_worksheet(Some(sheet_name)).unwrap();

    let headers = [
        "Criação do Card", "Nome da Empresa", "Nome do Cliente", "Data de Chegada", "Classificação do Cliente",
        "Canal de Chegada", "Data de Atendimento", "Status Conversão para Diagnóstico",
        "Status Conformidade", "Motivo Inconformidade", "Data do Diagnóstico",
        "Status conversão para proposta", "Data de Entrega da Proposta", "Valor oferecido do Projeto",
        "Valor da Hora do Projeto", "Status Orientação Proposta", "Data de Resposta do Cliente",
        "Resposta do Cliente", "Data de Assinatura do Contrato", "Preço Vendido",
        "Preço da Hora Vendida", "Etiqueta Indicação", "Subcanal de Chegada - Indicação"
    ];

    // Criação de um formato para os cabeçalhos
    let mut header_format = Format::new();
    header_format.set_bold();
    header_format.set_align(FormatAlignment::CenterAcross); // Use apenas o alinhamento horizontal disponível

    // Aplicando os cabeçalhos e seus estilos
    for (col_num, header) in headers.iter().enumerate() {
        worksheet.write_string(0, col_num as u16, header, Some(&header_format)).unwrap();
    }

    let mut row_num = 1;
    row_num = process_phases(&mut worksheet, &headers, all_phases, row_num);

    // Lendo os dados existentes da planilha usando calamine
    let mut reader: calamine::Xlsx<_> = open_workbook("path_to_existing_file.xlsx").unwrap();
    let range = reader
        .worksheet_range(sheet_name)
        .expect("Cannot find the sheet")
        .expect("Cannot read the sheet");

    // Extraindo os dados das células para ordenar
    let mut data_rows: Vec<Vec<String>> = Vec::new();
    for row in range.rows().skip(1) {
        let row_data: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
        data_rows.push(row_data);
    }

    // Ordenar os dados pela primeira coluna ("Criação do Card"), em ordem decrescente
    data_rows.sort_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a[0], "%d/%m/%Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        let date_b = NaiveDate::parse_from_str(&b[0], "%d/%m/%Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1900, 1, 1).unwrap());
        date_b.cmp(&date_a)
    });

    // Limpar as linhas antigas
    for row in 1..row_num {
        for col in 0..headers.len() {
            worksheet.write_blank(row as u32, col as u16, None).unwrap();
        }
    }

    // Escrever os dados ordenados de volta na planilha
    for (i, row) in data_rows.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            worksheet.write_string(i as u32 + 1, j as u16, value, None).unwrap();
        }
    }

    // Ajuste das colunas
    for col in 0..headers.len() {
        let max_length = data_rows.iter().map(|row| row[col].len()).max().unwrap_or(0);
        let adjusted_width = max_length as f64 + 2.0;
        worksheet.set_column(col as u16, col as u16, adjusted_width, None).unwrap();
    }

    println!("Dados atualizados na aba '{}'.", sheet_name);
}

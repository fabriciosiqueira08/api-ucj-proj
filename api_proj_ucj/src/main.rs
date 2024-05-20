mod definitions;
mod fetch_pipefy_data;
mod fetch_all_cards;
mod clean_value;
mod process_card;
mod process_phases;
mod update_excel;
mod types;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use definitions::Config;
use fetch_all_cards::fetch_all_cards;
use update_excel::update_excel;
use calamine::{open_workbook, Xlsx};
use serde::{Deserialize, Serialize};
use xlsxwriter::prelude::*;
use xlsxwriter::{Workbook, Format};

#[derive(Serialize, Deserialize)]
struct UpdateRequest {
    directory: String,
}

async fn update_excel_files(req: web::Json<UpdateRequest>, config: web::Data<Config>) -> impl Responder {
    let directory = &req.directory;

    for (pipe_name, pipe_file) in &config.pipe_to_file {
        let filename = format!("{}/{}", directory, pipe_file.filename);
        
        // Tente abrir o arquivo existente com calamine
        let reader: Option<Xlsx<_>> = match open_workbook(&filename) {
            Ok(wb) => {
                println!("Arquivo '{}' carregado com sucesso.", filename);
                Some(wb)
            }
            Err(_) => {
                println!("Arquivo '{}' não encontrado, criando novo arquivo.", filename);
                None
            }
        };

        // Obtenha os dados da API
        let pipe_id = &config.pipe_ids[pipe_name].id;
        let all_phases = fetch_all_cards(pipe_id, &config).await.unwrap();

        // Crie um novo workbook com xlsxwriter
        let mut workbook_writer = Workbook::new(&filename).expect("Erro ao criar o workbook");
        let mut sheet = workbook_writer.add_worksheet(Some(&pipe_file.sheet_name)).unwrap();

        // Adicione cabeçalhos à nova planilha
        let headers = [
            "Criação do Card", "Nome da Empresa", "Nome do Cliente", "Data de Chegada", "Classificação do Cliente",
            "Canal de Chegada", "Data de Atendimento", "Status Conversão para Diagnóstico",
            "Status Conformidade", "Motivo Inconformidade", "Data do Diagnóstico",
            "Status conversão para proposta", "Data de Entrega da Proposta", "Valor oferecido do Projeto",
            "Valor da Hora do Projeto", "Status Orientação Proposta", "Data de Resposta do Cliente",
            "Resposta do Cliente", "Data de Assinatura do Contrato", "Preço Vendido",
            "Preço da Hora Vendida", "Etiqueta Indicação", "Subcanal de Chegada - Indicação"
        ];

        let mut header_format = Format::new();
        header_format.set_bold().set_align(FormatAlignment::CenterAcross).set_fg_color(FormatColor::Yellow);

        for (col_num, header) in headers.iter().enumerate() {
            sheet.write_string(0, col_num as u16, header, Some(&header_format)).unwrap();
        }

        // Atualize a planilha com os dados retornados
        update_excel(&mut workbook_writer, &all_phases, &pipe_file.sheet_name);

        // Feche o workbook do xlsxwriter
        workbook_writer.close().expect("Erro ao fechar o workbook");

        println!("Arquivo '{}' salvo com sucesso com a aba atualizada.", filename);
    }

    HttpResponse::Ok().json("Atualização concluída com sucesso!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/update_excel_files", web::post().to(update_excel_files))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

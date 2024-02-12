use crate::{errors::HttpError, models::NewTransacao, services::insere_transacao};
use chrono::Local;
use ntex::web;
use serde::Deserialize;

use crate::PgPool;

#[derive(Deserialize)]
struct Transacao {
    valor: i32,
    tipo: char,
    descricao: String,
}

#[web::post("/clientes/{cliente_id}/transacoes")]
pub async fn create_transaction(
    pool: web::types::State<PgPool>,
    path: web::types::Path<u32>,
    transacao: web::types::Json<Transacao>,
) -> Result<String, web::Error> {
    let cliente_id = path.into_inner();
    let current_date = Local::now().naive_local();

    if cliente_id <= 0 || cliente_id > 5 {
        return Err(HttpError::BadClientData.into());
    }
    let new_transacao = NewTransacao {
        cliente_id,
        descricao: transacao.descricao.to_string(),
        tipo: transacao.tipo.to_string(),
        realizada_em: current_date,
        valor: transacao.valor,
    };

    insere_transacao(pool, new_transacao)
        .await
        .expect("Error ao inserir transação");

    Ok(format!("user_id {} transiction complete", cliente_id))
}

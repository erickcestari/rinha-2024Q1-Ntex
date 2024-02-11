use crate::services::insere_transicao;
use chrono::{Local, NaiveDate};
use ntex::web;
use serde::Deserialize;

use crate::PgPool;

#[derive(Deserialize)]
struct Transacao {
    valor: i32,
    tipo: char,
    descricao: String,
}

#[web::post("/clientes/{id}/transacoes")]
pub async fn create_transaction(
    pool: web::types::State<PgPool>,
    path: web::types::Path<i32>,
    transacao: web::types::Json<Transacao>,
) -> Result<String, web::Error> {
    let id = path.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let current_date = Local::now().naive_local();

    insere_transicao(
        &mut conn,
        transacao.valor,
        transacao.tipo,
        &transacao.descricao,
        &id,
        &current_date,
    );
    Ok(format!(
        "user_id {} transiction {}!",
        id, transacao.descricao
    ))
}

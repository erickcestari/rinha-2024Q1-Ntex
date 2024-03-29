use crate::{
    errors::HttpError,
    models::{ClienteData, CreateTransacao, NewTransacao, Saldo, UpdatedClient},
    services::{get_cliente, get_transacoes, insere_transacao, update_cliente_saldo},
};
use chrono::Local;
use ntex::web;

use crate::PgPool;

#[web::get("/clientes/{cliente_id}/extrato")]
pub async fn get_extrato(
    pool: web::types::State<PgPool>,
    path: web::types::Path<i32>,
) -> Result<impl web::Responder, web::Error> {
    let cliente_id = path.into_inner();
    let current_date = Local::now().naive_local();

    if cliente_id <= 0 || cliente_id > 5 {
        return Err(HttpError::NotFound.into());
    }

    let cliente = get_cliente(&pool, cliente_id)
        .await
        .expect("Erro ao pegar as informações do cliente");

    let transacoes = get_transacoes(&pool, cliente_id)
        .await
        .expect("Erro ao pergar as ultimas transacoes");

    let client_saldo = Saldo {
        total: cliente.saldo,
        data_extrato: current_date,
        limite: cliente.limite,
    };

    let client_data = ClienteData {
        saldo: client_saldo,
        ultimas_transacoes: transacoes,
    };
    Ok(web::HttpResponse::Ok().json(&client_data))
}

#[web::post("/clientes/{cliente_id}/transacoes")]
pub async fn create_transaction(
    pool: web::types::State<PgPool>,
    path: web::types::Path<i32>,
    transacao: web::types::Json<CreateTransacao>,
) -> Result<impl web::Responder, web::Error> {
    let cliente_id = path.into_inner();
    let current_date = Local::now().naive_local();

    if cliente_id <= 0 || cliente_id > 5 {
        return Err(HttpError::NotFound.into());
    }

    if transacao.descricao.len() > 10 || transacao.descricao.is_empty() || transacao.valor < 1 {
        return Err(HttpError::UnprocessableEntity.into());
    }

    let transacao_valor = match transacao.tipo {
        'd' => -transacao.valor,
        'c' => transacao.valor,
        _ => return Err(HttpError::UnprocessableEntity.into()),
    };

    let new_transacao = NewTransacao {
        cliente_id,
        descricao: transacao.descricao.to_string(),
        tipo: transacao.tipo.to_string(),
        realizada_em: current_date,
        valor: transacao.valor,
    };

    let (limite, saldo) = match update_cliente_saldo(&pool, cliente_id, transacao_valor).await {
        Ok((limite, saldo)) => (limite, saldo),
        Err(_) => return Err(HttpError::UnprocessableEntity.into()),
    };

    if let Err(_) = insere_transacao(&pool, new_transacao).await {
        return Err(HttpError::UnprocessableEntity.into());
    }

    Ok(web::HttpResponse::Ok().json(&UpdatedClient { limite, saldo }))
}

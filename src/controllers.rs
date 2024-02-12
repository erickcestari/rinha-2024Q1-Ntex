use crate::{
    errors::HttpError,
    models::{CreateTransacao, NewTransacao, UpdatedClient},
    services::{get_cliente, insere_transacao, update_cliente_saldo},
};
use chrono::Local;
use ntex::web;

use crate::PgPool;

#[web::post("/clientes/{cliente_id}/transacoes")]
pub async fn create_transaction(
    pool: web::types::State<PgPool>,
    path: web::types::Path<u32>,
    transacao: web::types::Json<CreateTransacao>,
) -> Result<impl web::Responder, web::Error> {
    let cliente_id = path.into_inner();
    let current_date = Local::now().naive_local();

    if cliente_id <= 0 || cliente_id > 5 {
        return Err(HttpError::BadClientData.into());
    }

    let cliente = get_cliente(&pool, cliente_id as i32)
        .await
        .expect("Erro ao pegar as informações do cliente");

    let saldo = match transacao.tipo {
        'd' => cliente.saldo - transacao.valor,
        'c' => cliente.saldo + transacao.valor,
        _ => return Err(HttpError::BadClientData.into()),
    };

    if saldo < -cliente.limite {
        return Err(HttpError::UnprocessableEntity.into());
    }

    let new_transacao = NewTransacao {
        cliente_id,
        descricao: transacao.descricao.to_string(),
        tipo: transacao.tipo.to_string(),
        realizada_em: current_date,
        valor: transacao.valor,
    };

    insere_transacao(&pool, new_transacao)
        .await
        .expect("Error ao inserir transação");

    update_cliente_saldo(&pool, cliente_id as i32, saldo).await;

    let udpated_client = UpdatedClient {
        limite: cliente.limite,
        saldo,
    };

    Ok(web::HttpResponse::Ok().json(&udpated_client))
}

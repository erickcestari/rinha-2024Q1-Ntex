use chrono::NaiveDateTime;
use ntex::web::types::State;
use sqlx::PgPool;
use std::result::Result;

use crate::models::NewTransacao;

pub async fn insere_transacao(
    conn: State<PgPool>,
    new_transacao: NewTransacao,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO transacoes (valor, tipo, descricao, cliente_id, realizada_em)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&new_transacao.valor)
    .bind(&new_transacao.tipo)
    .bind(&new_transacao.descricao)
    .bind(new_transacao.cliente_id as i32)
    .bind(&new_transacao.realizada_em)
    .execute(conn.get_ref())
    .await?;

    Ok(result.rows_affected())
}

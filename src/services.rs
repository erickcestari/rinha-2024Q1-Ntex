use ntex::web::types::State;
use sqlx::PgPool;
use std::result::Result;

use crate::models::{Cliente, NewTransacao, Transacao};

pub async fn insere_transacao(
    conn: &State<PgPool>,
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
    .bind(new_transacao.cliente_id)
    .bind(&new_transacao.realizada_em)
    .execute(conn.get_ref())
    .await?;

    Ok(result.rows_affected())
}

pub async fn get_cliente(conn: &State<PgPool>, id: i32) -> Result<Cliente, sqlx::Error> {
    let cliente = sqlx::query_as::<_, Cliente>(
        r#"
        SELECT * FROM clientes WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(conn.get_ref())
    .await?;

    Ok(cliente)
}

pub async fn update_cliente_saldo(conn: &State<PgPool>, id: i32, saldo: i32) {
    let _ = sqlx::query(
        r#"
        UPDATE clientes SET saldo = $1 WHERE id = $2
        "#,
    )
    .bind(saldo)
    .bind(id)
    .execute(conn.get_ref())
    .await;
}

pub async fn get_transacoes(conn: &State<PgPool>, id: i32) -> Result<Vec<Transacao>, sqlx::Error> {
    let transacoes = sqlx::query_as::<_, Transacao>(
        r#"
        SELECT tipo, descricao, valor, realizada_em FROM transacoes WHERE cliente_id = $1
        "#,
    )
    .bind(id)
    .fetch_all(conn.get_ref())
    .await
    .expect("Erro ao inserir");

    Ok(transacoes)
}

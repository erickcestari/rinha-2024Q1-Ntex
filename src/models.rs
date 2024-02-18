use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};

#[derive(Debug)]
pub struct NewTransacao {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub cliente_id: i32,
    pub realizada_em: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTransacao {
    pub valor: i32,
    pub tipo: char,
    pub descricao: String,
}

#[derive(Serialize)]
pub struct UpdatedClient {
    pub limite: i32,
    pub saldo: i32,
}

#[derive(Debug, FromRow)]
pub struct Cliente {
    pub id: i32,
    pub nome: String,
    pub limite: i32,
    pub saldo: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Saldo {
    pub total: i32,
    pub data_extrato: NaiveDateTime,
    pub limite: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, FromRow)]
pub struct Transacao {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: NaiveDateTime,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ClienteData {
    pub saldo: Saldo,
    pub ultimas_transacoes: Vec<Transacao>,
}

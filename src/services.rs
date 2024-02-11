use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;

mod schema {
    use diesel::table;

    table! {
        transacao (id) {
            id -> Integer,
            valor -> Integer,
            tipo -> Char,
            descricao -> VarChar,
            cliente_id -> Integer,
            realizada_em -> Timestamp,
        }
    }
}

use schema::transacao;

use self::schema::transacao::realizada_em;

#[derive(Debug, Insertable)]
#[table_name = "transacao"]
struct NewTransacao<'a> {
    valor: &'a i32,
    tipo: &'a str,
    descricao: &'a str,
    cliente_id: &'a i32,
    realizada_em: &'a NaiveDateTime,
}

pub fn insere_transicao(
    conn: &mut PgConnection,
    valor: i32,
    tipo: char,
    descricao: &str,
    cliente_id: &i32,
    timestamp: &NaiveDateTime,
) -> QueryResult<usize> {
    // Create insertion model
    let new_transacao = NewTransacao {
        valor: &valor,
        tipo: &tipo.to_string(),
        descricao: &descricao,
        cliente_id,
        realizada_em: timestamp,
    };

    let result = diesel::insert_into(transacao::table)
        .values(&new_transacao)
        .execute(conn)?;

    Ok(result)
}

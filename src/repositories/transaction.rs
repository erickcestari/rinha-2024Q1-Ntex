use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema {
    use diesel::table;

    table! {
        transacao (id) {
            id -> Integer,
            valor -> Integer,
            tipo -> Char,
            descricao -> VarChar,
        }
    }
}

use schema::transacao;

#[derive(Debug, Insertable)]
#[table_name = "transacao"]
struct NewTransacao<'a> {
    valor: &'a i32,
    tipo: &'a str,
    descricao: &'a str,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn insere_transicao(
    conn: &mut PgConnection,
    valor: i32,
    tipo: &str,
    descricao: &str,
) -> QueryResult<schema::transacao::SqlType> {
    // Create insertion model
    let new_transacao = NewTransacao {
        valor: &valor,
        tipo: &tipo,
        descricao: &descricao,
    };

    // normal diesel operations
    diesel::insert_into(transacao::table)
        .values(&new_transacao)
        .execute(conn)?;

    Ok(transacao)
}

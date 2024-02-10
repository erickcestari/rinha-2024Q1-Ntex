use ntex::web;
use serde::Deserialize;

#[derive(Deserialize)]
struct Transacao {
    valor: u32,
    tipo: char,
    descricao: String,
}

#[web::post("/clientes/{id}/transacoes")]
pub async fn create_transaction(
    path: web::types::Path<u32>,
    transacao: web::types::Json<Transacao>,
) -> Result<String, web::Error> {
    let id = path.into_inner();
    Ok(format!(
        "user_id {} transiction {}!",
        id, transacao.descricao
    ))
}

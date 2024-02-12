use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct NewTransacao {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub cliente_id: u32,
    pub realizada_em: NaiveDateTime,
}

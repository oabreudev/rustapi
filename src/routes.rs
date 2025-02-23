// src/routes.rs
use rocket::serde::json::Json;
use rocket::response::status::NotFound;
use diesel::prelude::*;
use crate::{DbConn, schema::*, models::*, db};

#[get("/cpf/<cpf>")]
pub async fn get_by_cpf(conn: DbConn, cpf: String) -> Result<Json<Vec<CompleteData>>, NotFound<String>> {
    let contatos_ids = conn.run(move |c| {
        DADOS::dsl::DADOS
            .filter(DADOS::dsl::cpf.eq(&cpf))
            .select(DADOS::dsl::contatos_id)
            .load::<String>(c)
    }).await.map_err(|_| NotFound("CPF not found".to_string()))?;

    Ok(Json(db::get_complete_data(conn, contatos_ids).await))
}

#[get("/email/<email_query>")]
pub async fn get_by_email(conn: DbConn, email_query: String) -> Result<Json<Vec<CompleteData>>, NotFound<String>> {
    let contatos_ids = conn.run(move |c| {
        EMAIL::dsl::EMAIL
            .filter(EMAIL::dsl::email.eq(&email_query))
            .select(EMAIL::dsl::contatos_id)
            .load::<i32>(c)
    }).await.map_err(|_| NotFound("Email not found".to_string()))?;

    let contatos_ids_str = contatos_ids.into_iter().map(|id| id.to_string()).collect();
    Ok(Json(db::get_complete_data(conn, contatos_ids_str).await))
}

#[get("/telefone/<numero_completo>")]
pub async fn get_by_telefone(conn: DbConn, numero_completo: String) -> Result<Json<Vec<CompleteData>>, NotFound<String>> {
    if numero_completo.len() < 10 || numero_completo.len() > 11 {
        return Err(NotFound("Número de telefone inválido".to_string()));
    }

    let ddd_str = &numero_completo[0..2];
    let telefone_str = &numero_completo[2..];
    let ddd_num: i32 = ddd_str.parse().map_err(|_| NotFound("DDD inválido".to_string()))?;
    let telefone_num: i32 = telefone_str.parse().map_err(|_| NotFound("Número inválido".to_string()))?;

    let contatos_ids = conn.run(move |c| {
        TELEFONE::dsl::TELEFONE
            .filter(TELEFONE::dsl::ddd.eq(ddd_num))
            .filter(TELEFONE::dsl::telefone.eq(telefone_num))
            .select(TELEFONE::dsl::contatos_id)
            .load::<i32>(c)
    }).await.map_err(|_| NotFound("Telefone not found".to_string()))?;

    let contatos_ids_str = contatos_ids.into_iter().map(|id| id.to_string()).collect();
    Ok(Json(db::get_complete_data(conn, contatos_ids_str).await))
}

#[get("/nome/<nome_query>")]
pub async fn get_by_nome(conn: DbConn, nome_query: String) -> Result<Json<Vec<CompleteData>>, NotFound<String>> {
    let contatos_ids = conn.run(move |c| {
        DADOS::dsl::DADOS
            .filter(DADOS::dsl::nome.like(format!("%{}%", nome_query)))
            .limit(100)
            .select(DADOS::dsl::contatos_id)
            .load::<String>(c)
    }).await.map_err(|_| NotFound("Nome not found".to_string()))?;

    Ok(Json(db::get_complete_data(conn, contatos_ids).await))
}

#[get("/cep/<cep_query>?<page>&<per_page>")]
pub async fn get_cpfs_by_cep(
    conn: DbConn,
    cep_query: String,
    page: Option<i32>,
    per_page: Option<i32>
) -> Result<Json<CepResponse>, NotFound<String>> {
    let page = page.unwrap_or(1).max(1);
    let per_page = per_page.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let results = conn.run(move |c| {
        let base_query = ENDERECOS::dsl::ENDERECOS
            .inner_join(DADOS::dsl::DADOS.on(
                ENDERECOS::dsl::contatos_id.eq(DADOS::dsl::contatos_id)
            ))
            .filter(ENDERECOS::dsl::cep.eq(&cep_query));

        let total = base_query
            .clone()
            .count()
            .get_result::<i64>(c)?;

        let cpfs = base_query
            .select(DADOS::dsl::cpf)
            .offset(offset.into())
            .limit(per_page.into())
            .load::<Option<String>>(c)?
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<String>>();

        let error: Result<_, diesel::result::Error> = Ok(CepResponse {
            cpfs,
            total,
            page,
            per_page,
            total_pages: ((total as f64) / (per_page as f64)).ceil() as i32,
        });
        error
    }).await.map_err(|_| NotFound("CEP not found".to_string()))?;

    Ok(Json(results))
}
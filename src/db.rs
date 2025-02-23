use diesel::prelude::*;
use rayon::prelude::*;
use crate::{DbConn, schema::*, models::*};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
    static ref CACHE: Arc<RwLock<HashMap<String, Vec<CompleteData>>>> = Arc::new(RwLock::new(HashMap::with_capacity(1000)));
}

pub async fn get_complete_data(conn: DbConn, contatos_ids: Vec<String>) -> Vec<CompleteData> {
    // Verifica cache primeiro
    let cache_key = contatos_ids.join(",");
    {
        let cache_read = CACHE.read();
        if let Some(cached_data) = cache_read.get(&cache_key) {
            return cached_data.clone();
        }
    }

    let result = conn.run(move |c| {
        // Configura o SQLite para performance
        configure_database(c);

        // Busca dados principais
        let dados: HashMap<String, Dados> = DADOS::dsl::DADOS
            .filter(DADOS::dsl::contatos_id.eq_any(&contatos_ids))
            .load::<Dados>(c)
            .unwrap_or_default()
            .into_iter()
            .map(|d| (d.contatos_id.clone(), d))
            .collect();

        let ids_i32: Vec<i32> = contatos_ids
            .iter()
            .filter_map(|id| id.parse::<i32>().ok())
            .collect();

        // Busca emails
        let emails: HashMap<i32, Vec<Email>> = EMAIL::dsl::EMAIL
            .filter(EMAIL::dsl::contatos_id.eq_any(&ids_i32))
            .load::<Email>(c)
            .unwrap_or_default()
            .into_iter()
            .fold(HashMap::new(), |mut acc, email| {
                acc.entry(email.contatos_id)
                    .or_insert_with(Vec::new)
                    .push(email);
                acc
            });

        // Busca telefones
        let telefones: HashMap<i32, Vec<Telefone>> = TELEFONE::dsl::TELEFONE
            .filter(TELEFONE::dsl::contatos_id.eq_any(&ids_i32))
            .load::<Telefone>(c)
            .unwrap_or_default()
            .into_iter()
            .fold(HashMap::new(), |mut acc, tel| {
                acc.entry(tel.contatos_id)
                    .or_insert_with(Vec::new)
                    .push(tel);
                acc
            });

        // Busca endereços
        let enderecos: HashMap<String, Vec<Endereco>> = ENDERECOS::dsl::ENDERECOS
            .filter(ENDERECOS::dsl::contatos_id.eq_any(&contatos_ids))
            .load::<Endereco>(c)
            .unwrap_or_default()
            .into_iter()
            .fold(HashMap::new(), |mut acc, end| {
                acc.entry(end.contatos_id.clone())
                    .or_insert_with(Vec::new)
                    .push(end);
                acc
            });

        // Busca poder aquisitivo
        let poder_aquisitivo: HashMap<i32, PoderAquisitivo> = PODER_AQUISITIVO::table
            .filter(PODER_AQUISITIVO::contatos_id.eq_any(&ids_i32))
            .load::<PoderAquisitivo>(c)
            .unwrap_or_default()
            .into_iter()
            .map(|pa| (pa.contatos_id, pa))
            .collect();

        // Busca scores
        let scores: HashMap<i32, Score> = SCORE::table
            .filter(SCORE::contatos_id.eq_any(&ids_i32))
            .load::<Score>(c)
            .unwrap_or_default()
            .into_iter()
            .map(|s| (s.contatos_id, s))
            .collect();

        // Busca PIS
        let pis_data: HashMap<i32, Pis> = PIS::table
            .filter(PIS::contatos_id.eq_any(&ids_i32))
            .load::<Pis>(c)
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p.contatos_id, p))
            .collect();

        // Busca TSE
        let tse_data: HashMap<i32, Tse> = TSE::table
            .filter(TSE::contatos_id.eq_any(&ids_i32))
            .load::<Tse>(c)
            .unwrap_or_default()
            .into_iter()
            .map(|t| (t.contatos_id, t))
            .collect();

        // Processa em paralelo
        let result: Vec<CompleteData> = contatos_ids
            .par_iter()
            .map(|id| {
                let id_i32 = id.parse::<i32>().unwrap_or_default();
                let telefones = telefones.get(&id_i32).cloned().unwrap_or_default();

                CompleteData::new(
                    dados.get(id).cloned(),
                    emails.get(&id_i32).cloned().unwrap_or_default(),
                    telefones,
                    enderecos.get(id).cloned().unwrap_or_default(),
                    poder_aquisitivo.get(&id_i32).cloned(),
                    scores.get(&id_i32).cloned(),
                    pis_data.get(&id_i32).cloned(),
                    tse_data.get(&id_i32).cloned()
                )
            })
            .collect();

        result
    }).await;

    // Atualiza o cache
    CACHE.write().insert(cache_key, result.clone());

    result
}

pub fn clear_cache() {
    CACHE.write().clear();
}

fn configure_database(conn: &mut SqliteConnection) {
    let _ = diesel::sql_query("PRAGMA journal_mode = WAL").execute(conn);
    let _ = diesel::sql_query("PRAGMA synchronous = NORMAL").execute(conn);
    let _ = diesel::sql_query("PRAGMA cache_size = -2000000").execute(conn);
    let _ = diesel::sql_query("PRAGMA temp_store = MEMORY").execute(conn);
    let _ = diesel::sql_query("PRAGMA mmap_size = 30000000000").execute(conn);
}
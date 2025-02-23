// models.rs
use serde::Serialize;
use diesel::prelude::*;
use crate::schema::*;

#[derive(Debug, Clone, Serialize)]
pub struct CompleteData {
    pub dados: Option<Dados>,
    pub emails: Vec<Email>,
    pub telefones_fixos: Vec<TelefoneFormatado>,
    pub telefones_celulares: Vec<TelefoneFormatado>,
    pub enderecos: Vec<Endereco>,
    pub poder_aquisitivo: Option<PoderAquisitivo>,
    pub score: Option<Score>,
    pub pis: Option<Pis>,
    pub tse: Option<Tse>
}

#[derive(Debug, Clone, Serialize)]
pub struct CepResponse {
    pub cpfs: Vec<String>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = DADOS)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dados {
    pub contatos_id: String,
    pub cpf: Option<String>,
    pub nome: Option<String>,
    pub sexo: Option<String>,
    pub nasc: Option<String>,
    pub nome_mae: Option<String>,
    pub nome_pai: Option<String>,
    pub cadastro_id: Option<String>,
    pub estciv: Option<String>,
    pub rg: Option<String>,
    pub nacionalid: Option<String>,
    pub contatos_id_conjuge: Option<String>,
    pub so: Option<String>,
    pub cd_sit_cad: Option<String>,
    pub dt_sit_cad: Option<String>,
    pub dt_informacao: Option<String>,
    pub cbo: Option<String>,
    pub orgao_emissor: Option<String>,
    pub uf_emissao: Option<String>,
    pub dt_ob: Option<String>,
    pub cd_mosaic: Option<String>,
    pub renda: Option<String>,
    pub faixa_renda_id: Option<String>,
    pub titulo_eleitor: Option<String>,
    pub cd_mosaic_novo: Option<String>,
    pub cd_mosaic_secundario: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = EMAIL)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Email {
    pub contatos_id: i32,
    pub email: Option<String>,
    pub prioridade: Option<i32>,
    pub email_score: Option<String>,
    pub email_pessoal: Option<String>,
    pub email_duplicado: Option<String>,
    pub blacklist: Option<String>,
    pub estrutura: Option<String>,
    pub status_vt: Option<String>,
    pub dominio: Option<String>,
    pub mapas: Option<i32>,
    pub peso: Option<i32>,
    pub cadastro_id: Option<i32>,
    pub dt_inclusao: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = PODER_AQUISITIVO)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PoderAquisitivo {
    pub contatos_id: i32,
    pub cod_poder_aquisitivo: Option<i32>,
    pub poder_aquisitivo: Option<String>,
    pub renda_poder_aquisitivo: Option<String>,
    pub fx_poder_aquisitivo: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = ENDERECOS)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Endereco {
    pub contatos_id: String,
    pub logr_tipo: Option<String>,
    pub logr_nome: Option<String>,
    pub logr_numero: Option<String>,
    pub logr_complemento: Option<String>,
    pub bairro: Option<String>,
    pub cidade: Option<String>,
    pub uf: Option<String>,
    pub cep: Option<String>,
    pub dt_atualizacao: Option<String>,
    pub dt_inclusao: Option<String>,
    pub tipo_endereco_id: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = PARENTES)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Parente {
    pub cpf_completo: Option<i32>,
    pub nome: Option<String>,
    pub cpf_vinculo: Option<i32>,
    pub nome_vinculo: Option<String>,
    pub vinculo: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = PIS)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Pis {
    pub contatos_id: i32,
    #[diesel(sql_type = Double)]
    pub pis: Option<f64>,
    pub cadastro_id: Option<String>,
    pub dt_inclusao: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = SCORE)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Score {
    pub contatos_id: i32,
    pub csb8: Option<String>,
    pub csb8_faixa: Option<String>,
    pub csba: Option<i32>,
    pub csba_faixa: Option<String>
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = TELEFONE)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Telefone {
    pub contatos_id: i32,
    pub ddd: Option<i32>,
    pub telefone: Option<i32>,
    pub tipo_telefone: Option<i32>,
    pub dt_inclusao: Option<String>,
    pub dt_informacao: Option<String>,
    pub sigilo: Option<i32>,
    pub nsu: Option<String>,
    pub classificacao: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub struct TelefoneFormatado {
    pub contatos_id: i32,
    pub ddd: Option<i32>,
    pub telefone: Option<i32>,
    pub tipo_telefone: Option<i32>,
    pub dt_inclusao: Option<String>,
    pub dt_informacao: Option<String>,
    pub sigilo: Option<i32>,
    pub nsu: Option<String>,
    pub classificacao: Option<String>,
    pub numero_completo: Option<String>
}

impl From<Telefone> for TelefoneFormatado {
    fn from(tel: Telefone) -> Self {
        let numero_completo = match (tel.ddd, tel.telefone) {
            (Some(ddd), Some(numero)) => {
                let numero_str = numero.to_string();
                let numero_formatado = if numero_str.len() < 8 {
                    format!("{:08}", numero)
                } else {
                    numero_str
                };
                Some(format!("{}{}", ddd, numero_formatado))
            },
            _ => None
        };

        Self {
            contatos_id: tel.contatos_id,
            ddd: tel.ddd,
            telefone: tel.telefone,
            tipo_telefone: tel.tipo_telefone,
            dt_inclusao: tel.dt_inclusao,
            dt_informacao: tel.dt_informacao,
            sigilo: tel.sigilo,
            nsu: tel.nsu,
            classificacao: tel.classificacao,
            numero_completo
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = TSE)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tse {
    pub contatos_id: i32,
    #[diesel(sql_type = Double)]
    pub titulo_eleitor: Option<f64>,
    pub zona: Option<String>,
    pub secao: Option<String>
}

impl CompleteData {
    pub fn new(
        dados: Option<Dados>,
        emails: Vec<Email>,
        telefones: Vec<Telefone>,
        enderecos: Vec<Endereco>,
        poder_aquisitivo: Option<PoderAquisitivo>,
        score: Option<Score>,
        pis: Option<Pis>,
        tse: Option<Tse>
    ) -> Self {
        // Converter telefones para o formato com número completo
        let telefones_formatados: Vec<TelefoneFormatado> = telefones.into_iter()
            .map(TelefoneFormatado::from)
            .collect();

        // Separar em fixos e celulares
        let (telefones_fixos, telefones_celulares): (Vec<TelefoneFormatado>, Vec<TelefoneFormatado>) = 
            telefones_formatados.into_iter()
            .partition(|tel| tel.tipo_telefone.unwrap_or(0) == 1);

        Self {
            dados,
            emails,
            telefones_fixos,
            telefones_celulares,
            enderecos,
            poder_aquisitivo,
            score,
            pis,
            tse
        }
    }
}
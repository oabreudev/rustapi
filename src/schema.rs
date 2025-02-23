// schema.rs
use diesel::prelude::*;

table! {
    DADOS (contatos_id) {
        contatos_id -> Text,
        cpf -> Nullable<Text>,
        nome -> Nullable<Text>,
        sexo -> Nullable<Text>,
        nasc -> Nullable<Text>,
        nome_mae -> Nullable<Text>,
        nome_pai -> Nullable<Text>,
        cadastro_id -> Nullable<Text>,
        estciv -> Nullable<Text>,
        rg -> Nullable<Text>,
        nacionalid -> Nullable<Text>,
        contatos_id_conjuge -> Nullable<Text>,
        so -> Nullable<Text>,
        cd_sit_cad -> Nullable<Text>,
        dt_sit_cad -> Nullable<Text>,
        dt_informacao -> Nullable<Text>,
        cbo -> Nullable<Text>,
        orgao_emissor -> Nullable<Text>,
        uf_emissao -> Nullable<Text>,
        dt_ob -> Nullable<Text>,
        cd_mosaic -> Nullable<Text>,
        renda -> Nullable<Text>,
        faixa_renda_id -> Nullable<Text>,
        titulo_eleitor -> Nullable<Text>,
        cd_mosaic_novo -> Nullable<Text>,
        cd_mosaic_secundario -> Nullable<Text>
    }
}

table! {
    EMAIL (contatos_id) {
        contatos_id -> Integer,
        email -> Nullable<Text>,
        prioridade -> Nullable<Integer>,
        email_score -> Nullable<Text>,
        email_pessoal -> Nullable<Text>,
        email_duplicado -> Nullable<Text>,
        blacklist -> Nullable<Text>,
        estrutura -> Nullable<Text>,
        status_vt -> Nullable<Text>,
        dominio -> Nullable<Text>,
        mapas -> Nullable<Integer>,
        peso -> Nullable<Integer>,
        cadastro_id -> Nullable<Integer>,
        dt_inclusao -> Nullable<Text>
    }
}

table! {
    PODER_AQUISITIVO (contatos_id) {
        contatos_id -> Integer,
        cod_poder_aquisitivo -> Nullable<Integer>,
        poder_aquisitivo -> Nullable<Text>,
        renda_poder_aquisitivo -> Nullable<Text>,
        fx_poder_aquisitivo -> Nullable<Text>
    }
}

table! {
    ENDERECOS (contatos_id) {
        contatos_id -> Text,
        logr_tipo -> Nullable<Text>,
        logr_nome -> Nullable<Text>,
        logr_numero -> Nullable<Text>,
        logr_complemento -> Nullable<Text>,
        bairro -> Nullable<Text>,
        cidade -> Nullable<Text>,
        uf -> Nullable<Text>,
        cep -> Nullable<Text>,
        dt_atualizacao -> Nullable<Text>,
        dt_inclusao -> Nullable<Text>,
        tipo_endereco_id -> Nullable<Text>
    }
}

table! {
    PARENTES (cpf_completo) {
        cpf_completo -> Nullable<Integer>,
        nome -> Nullable<Text>,
        cpf_vinculo -> Nullable<Integer>,
        nome_vinculo -> Nullable<Text>,
        vinculo -> Nullable<Text>
    }
}

table! {
    PIS (contatos_id) {
        contatos_id -> Integer,
        pis -> Nullable<Double>,
        cadastro_id -> Nullable<Text>,
        dt_inclusao -> Nullable<Text>
    }
}

table! {
    SCORE (contatos_id) {
        contatos_id -> Integer,
        csb8 -> Nullable<Text>,
        csb8_faixa -> Nullable<Text>,
        csba -> Nullable<Integer>,
        csba_faixa -> Nullable<Text>
    }
}

table! {
    TELEFONE (contatos_id) {
        contatos_id -> Integer,
        ddd -> Nullable<Integer>,
        telefone -> Nullable<Integer>,
        tipo_telefone -> Nullable<Integer>,
        dt_inclusao -> Nullable<Text>,
        dt_informacao -> Nullable<Text>,
        sigilo -> Nullable<Integer>,
        nsu -> Nullable<Text>,
        classificacao -> Nullable<Text>
    }
}

table! {
    TSE (contatos_id) {
        contatos_id -> Integer,
        titulo_eleitor -> Nullable<Double>,
        zona -> Nullable<Text>,
        secao -> Nullable<Text>
    }
}

allow_tables_to_appear_in_same_query!(
    DADOS, 
    EMAIL,
    PODER_AQUISITIVO,
    ENDERECOS,
    PARENTES,
    PIS,
    SCORE,
    TELEFONE,
    TSE
);
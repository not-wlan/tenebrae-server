table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    api_keys (id) {
        id -> Int4,
        name -> Varchar,
        key -> Varchar,
        state -> Api_key_state,
        message -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    signatures (id) {
        id -> Int4,
        owner -> Int4,
        signature -> Varchar,
        file -> Nullable<Varchar>,
        state -> Signature_state,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(api_keys, signatures,);

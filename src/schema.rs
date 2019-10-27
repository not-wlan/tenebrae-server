table! {
    api_keys (id) {
        id -> Int4,
        name -> Varchar,
        key -> Varchar,
        state -> Api_key_state,
        message -> Nullable<Varchar>,
    }
}

table! {
    signatures (id) {
        id -> Int4,
        owner -> Int4,
        signature -> Varchar,
        file -> Nullable<Varchar>,
        state -> Nullable<Signature_state>,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    api_keys,
    signatures,
);

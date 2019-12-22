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

    signature_history (hid) {
        hid -> Int4,
        id -> Int4,
        owner -> Int4,
        signature -> Varchar,
        filename -> Varchar,
        filehash -> Varchar,
        name -> Varchar,
        created -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        deleted -> Nullable<Timestamp>,
        deleted_by -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::sql_types::*;

    signatures (id) {
        id -> Int4,
        owner -> Int4,
        signature -> Varchar,
        filename -> Varchar,
        filehash -> Varchar,
        name -> Varchar,
    }
}

joinable!(signature_history -> api_keys (owner));
joinable!(signatures -> api_keys (owner));

allow_tables_to_appear_in_same_query!(api_keys, signature_history, signatures,);

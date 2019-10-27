use super::schema::signatures;
use diesel::{self, prelude::*, PgConnection};

// Thanks to https://atsuzaki.com/blog/diesel-enums/

#[PgType = "api_key_state"]
#[DieselType = "Api_key_state"]
#[derive(Debug, PartialEq, DbEnum, Serialize, Deserialize)]
pub enum ApiKeyState {
    #[db_rename = "enabled"]
    Enabled,
    #[db_rename = "disabled"]
    Disabled,
}

#[PgType = "signature_state"]
#[DieselType = "Signature_state"]
#[derive(Debug, PartialEq, DbEnum, Serialize, Deserialize)]
pub enum SignatureState {
    #[db_rename = "unverified"]
    Unverified,
    #[db_rename = "outdated"]
    Outdated,
    #[db_rename = "normal"]
    Normal,
}

#[table_name = "signatures"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Signature {
    id: i32,
    owner: i32,
    signature: String,
    file: Option<String>,
    state: SignatureState,
    name: String,
}

impl Signature {
    pub fn count(connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        signatures.count().get_result(connection)
    }
}

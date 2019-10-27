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
    pub signature: String,
    file: Option<String>,
    state: SignatureState,
    pub name: String,
}

impl Signature {
    /// Returns the total number of all signatures known to tenebrae on success
    pub fn count(connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        signatures.count().get_result(connection)
    }

    /// Persists a Signature to the Database. Returns the new id on success.
    pub fn persist(&self, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        diesel::insert_into(super::schema::signatures::table)
            .values(self)
            .returning(id)
            .execute(connection)
    }

    #[cfg(debug_assertions)]
    /// Fetch a signature directly by id.
    pub fn fetch(id: i32, connection: &PgConnection) -> Result<Signature, diesel::result::Error> {
        signatures::table.find(id).first(connection)
    }

    pub fn search(sigs: &Vec<String>, connection: &PgConnection) -> Result<Vec<Signature>, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        use diesel::dsl::*;
        signatures.filter(signature.eq(any(sigs))).load::<Signature>(connection)
    }
}

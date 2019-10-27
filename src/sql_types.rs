use super::schema::{api_keys, signatures};
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
    #[db_rename = "admin"]
    Admin,
    #[db_rename = "moderator"]
    Moderator,
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
    pub filehash: String,
    pub filename: String,
    state: SignatureState,
    pub name: String,
}

#[table_name = "api_keys"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct ApiKey {
    id: i32,
    pub name: String,
    pub key: String,
    state: ApiKeyState,
    message: Option<String>,
}

impl ApiKey {
    pub fn new(name: &str, state: ApiKeyState, message: Option<String>) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let key = format!("{:x}", rng.gen::<u128>());
        ApiKey {
            id: 0,
            name: name.to_string(),
            key,
            state,
            message,
        }
    }

    pub fn count_master_keys(connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        use super::schema::api_keys::dsl::*;
        api_keys
            .filter(state.eq(ApiKeyState::Admin))
            .count()
            .get_result(connection)
    }

    pub fn persist(&self, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        use super::schema::api_keys::dsl::*;
        diesel::insert_into(super::schema::api_keys::table)
            .values(self)
            .returning(id)
            .execute(connection)
    }
}

impl Signature {
    pub fn new(
        owner: i32,
        name: &str,
        signature: &str,
        filename: &str,
        filehash: &str,
        state: SignatureState,
    ) -> Self {
        Signature {
            id: 0,
            owner,
            signature: signature.to_string(),
            filename: filename.to_string(),
            filehash: filehash.to_string(),
            state,
            name: name.to_string(),
        }
    }

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

    /// Fetch a signature directly by id.
    pub fn fetch(id: i32, connection: &PgConnection) -> Result<Signature, diesel::result::Error> {
        signatures::table.find(id).first(connection)
    }

    pub fn search(
        sigs: &[String],
        connection: &PgConnection,
    ) -> Result<Vec<Signature>, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        use diesel::dsl::*;
        signatures
            .filter(signature.eq(any(sigs)))
            .load::<Signature>(connection)
    }
}

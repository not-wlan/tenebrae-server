use super::{
    database::PostgresPool,
    schema::{api_keys, signatures},
};
use diesel::{
    self,
    pg::upsert::{excluded, on_constraint},
    prelude::*,
    PgConnection,
};
use rocket::{
    http::Status, outcome::IntoOutcome, request, request::FromRequest, Outcome, Request, State,
};

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

#[table_name = "signatures"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Signature {
    id: i32,
    owner: i32,
    pub signature: String,
    pub filehash: String,
    pub filename: String,
    pub name: String,
}

#[table_name = "api_keys"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct ApiKey {
    pub id: i32,
    pub name: String,
    pub key: String,
    state: ApiKeyState,
    message: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;

        if let Ok(conn) = pool.get() {
            return request
                .cookies()
                .get("apikey")
                .map(|cookie| cookie.value())
                .and_then(|key| ApiKey::get_api_key(&key, &conn).ok())
                .into_outcome((Status::Forbidden, ()));
        }

        Outcome::Failure((Status::Forbidden, ()))
    }
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

    pub fn get_api_key(
        rawkey: &str,
        connection: &PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use super::schema::api_keys::dsl::*;
        api_keys.filter(key.eq(rawkey)).first(connection)
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
    pub fn new<S: Into<String>>(
        owner: i32,
        name: S,
        signature: S,
        filename: S,
        filehash: S,
    ) -> Self {
        Signature {
            id: 0,
            owner,
            signature: signature.into(),
            filename: filename.into(),
            filehash: filehash.into(),
            name: name.into(),
        }
    }

    /// Returns the total number of all signatures known to tenebrae on success
    pub fn count(connection: &PgConnection) -> Result<i64, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        signatures.count().get_result(connection)
    }

    pub fn persist_data(
        sigs: &[Signature],
        connection: &PgConnection,
    ) -> Result<usize, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        diesel::insert_into(super::schema::signatures::table)
            .values(sigs)
            .on_conflict(on_constraint("unique_signature"))
            .do_update()
            .set(name.eq(excluded(name)))
            .execute(connection)
    }

    pub fn by_hash(hash: &str, connection: &PgConnection) -> Result<Vec<Signature>, diesel::result::Error> {
        use super::schema::signatures::dsl::*;
        signatures
            .filter(filehash.eq(hash))
            .load::<Signature>(connection)
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

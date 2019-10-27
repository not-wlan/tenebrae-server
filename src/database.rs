use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};
use std::ops::Deref;

/// Our connection pool type
type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

/// Initialize the connection pool
pub fn connect() -> PostgresPool {
    // TODO: Example for a connection URL here (?)
    let url =
        dotenv::var("DATABASE_URL").expect("Please set the DATABASE_URL environment variable!");
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::new(manager)
        .expect("Failed to connect to the database. Please check the connection string!")
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    /// Inject our database connection into every request.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

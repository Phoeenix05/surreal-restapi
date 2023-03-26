use rocket::{
    fairing::{Fairing, Info, Kind, Result},
    serde::Deserialize,
    Build, Rocket,
};
use surrealdb::{sql::Value, Datastore, Error, Session};

pub struct Db {
    session: Session,
    datastore: Datastore,
}

impl Db {
    pub async fn new(ns: &str, db: &str, ds: &str) -> Self {
        Self {
            session: Session::for_db(ns, db),
            datastore: Datastore::new(&ds).await.unwrap(),
        }
    }

    pub async fn query(&self, q: &str) -> Result<Vec<Value>, Error> {
        let responses = self
            .datastore
            .execute(q, &self.session, None, false)
            .await?;

        let mut results = Vec::new();

        for response in responses {
            results.push(response.result?.first())
        }

        Ok(results)
    }
}

pub struct DbFairing;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct DbConfig {
    /// Database namespace
    ns: String,
    /// Database name
    db: String,
    /// Data store name
    ds: String,
}

#[rocket::async_trait]
impl Fairing for DbFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result {
        let figment = rocket.figment().clone();
        let db_config: DbConfig = figment.select("database").extract().unwrap();
        let db = Db::new(&db_config.ns, &db_config.db, &db_config.ds).await;

        Ok(rocket.manage(db))
    }
}

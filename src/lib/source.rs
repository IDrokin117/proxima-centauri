use anyhow::{anyhow, Result};
use csv::Reader;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::registry::Limits;

pub(crate) enum DBConnection {
    Csv(CSVConnection),
    Sql(SQLConnection),
}

impl Connection for DBConnection {
    type Record = UserRecord;

    fn establish(&self) -> Result<()> {
        match self {
            DBConnection::Csv(conn) => conn.establish(),
            DBConnection::Sql(conn) => conn.establish()
        }
    }

    fn fetch(&self, username: &str) -> Result<Option<UserRecord>> {
        match self {
            DBConnection::Csv(conn) => conn.fetch(username),
            DBConnection::Sql(conn) => conn.fetch(username)
        }
    }
}

trait Connection {
    type Record;
    fn establish(&self) -> Result<()>;
    fn fetch(&self, username: &str) -> Result<Option<Self::Record>>;
}

pub(crate) struct CSVConnectionParameters {
    file_path: PathBuf,
}
impl CSVConnectionParameters {
    pub(crate) fn new(path_buf: PathBuf) -> Self {
        CSVConnectionParameters {
            file_path: path_buf,
        }
    }
}
impl Default for CSVConnectionParameters{
    fn default() -> Self {
        CSVConnectionParameters::new(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("files/db.csv"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct UserRecord {
    username: String,
    password: String,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
    concurrency_limit: Option<u16>,
    traffic_limit: Option<u128>,
    status: String,
}

impl UserRecord{
    pub(crate) fn is_authenticated(&self, password: &str) -> bool {
        self.password == password
    }
}

impl From<UserRecord> for Limits{
    fn from(value: UserRecord) -> Self {
        Limits::new(value.concurrency_limit, value.traffic_limit)
    }
}
pub(crate) struct CSVConnection {
    params: CSVConnectionParameters,
    data: Mutex<Vec<UserRecord>>,
}

impl CSVConnection {
    pub(crate) fn new(params: CSVConnectionParameters) -> Self {
        CSVConnection {
            params,
            data: Mutex::new(Vec::new()),
        }
    }
}

impl Connection for CSVConnection {
    type Record = UserRecord;

    fn establish(&self) -> Result<()> {
        const MAX_FILE_SIZE: u64 = 10_000;
        if std::fs::metadata(&self.params.file_path).map(|m| m.len())? > MAX_FILE_SIZE {
            Err(anyhow!(format!(
                "CSV file must less than {MAX_FILE_SIZE} bytes"
            )))
        } else {
            let mut reader = Reader::from_path(&self.params.file_path)?;
            let mut data: Vec<UserRecord> = Vec::new();

            for record in reader.deserialize() {
                let user: UserRecord = record?;
                data.push(user);
            }
            *self.data.lock() = data;
            Ok(())
        }
    }

    fn fetch(&self, username: &str) -> Result<Option<UserRecord>> {
        Ok(self.data.lock()
            .iter()
            .find(|el| el.username == username)
            .cloned())
    }
}

pub(crate) struct SQLConnection {}

impl Connection for SQLConnection {
    type Record = UserRecord;

    fn establish(&self) -> Result<()> {
        todo!("SQL connection not implemented")
    }

    fn fetch(&self, _username: &str) -> Result<Option<UserRecord>> {
        todo!()
    }
}

pub(crate) struct Backend {
    connection: DBConnection,
    cache: Mutex<HashMap<String, UserRecord>>,
}

impl Backend {
    pub(crate) fn new(connection: DBConnection) -> Self {
        Self {
            connection,
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub(crate) fn fetch_user(&self, username: &str) -> Result<Option<UserRecord>> {
        let mut guard = self.cache.lock();
        if guard.contains_key(username) {
            return Ok(guard.get(username).cloned());
        }

        self.connection.establish()?;
        if let Some(user) = self.connection.fetch(username)? {
            guard.insert(username.to_string(), user);
        }
        Ok(guard.get(username).cloned())
    }

}

use rusqlite::{params, Connection};

use crate::{
    backup::{Backup, Id, Timestamp},
    filesystem::database_path,
    error::Result,
};

/// Abstraction over the SQLite database.
pub struct Database(Connection);

impl Database {
    /// Open a new in-memory database.
    pub fn open_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()?;
        Self::with_connection(connection)
    }

    /// Open a database at the given path.
    ///
    /// This will create the database if it does not exist.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let connection = Connection::open(path)?;
        Self::with_connection(connection)
    }

    /// Open the default database.
    pub fn open_default() -> Result<Self> {
        Self::open(database_path()?)
    }

    /// Open a database with the given connection.
    pub fn with_connection(connection: Connection) -> Result<Self> {
        let db = Self(connection);
        Ok(db)
    }

    /// Returns the underlying connection.
    pub fn connection(&self) -> &Connection {
        &self.0
    }

    /// Returns a proxy to the backup table.
    pub fn backup_table<'a>(&'a self, profile: &str) -> Result<BackupTable<'a>> {
        BackupTable::open(&self.0, profile)
    }
}

/// Proxy to the backup table for some profile.
pub struct BackupTable<'a> {
    /// The underlying connection.
    connection: &'a Connection,
    /// The name of the profile this table is for.
    profile: String,
}

impl<'a> BackupTable<'a> {
    /// Open the backup table, or create it if necessary.
    fn open(connection: &'a Connection, profile: &str) -> Result<Self> {
        let table = Self {
            connection,
            profile: profile.to_owned(),
        };
        table.create_table()?;
        Ok(table)
    }

    /// Drop the backup table.
    pub fn drop(self) -> Result<()> {
        let sql = format!("DROP TABLE IF EXISTS {}", self.profile);
        self.connection.execute(&sql, params![])?;
        Ok(())
    }

    /// Create the backup table if it does not exist.
    fn create_table(&self) -> Result<()> {
        let sql = &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                tag TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            self.profile
        );
        self.connection.execute(sql, params![])?;
        Ok(())
    }

    /// Insert a new backup into the table.
    pub fn insert(&self, tag: &str, timestamp: &Timestamp) -> Result<Backup> {
        let sql = format!(
            "INSERT INTO {} (tag, timestamp) VALUES (?, ?)",
            self.profile
        );
        self.connection.execute(&sql, params![tag, timestamp])?;
        Ok(Backup::new(
            self.last_id(),
            tag.to_owned(),
            timestamp.to_owned(),
        ))
    }

    /// Select a backup with the given ID
    pub fn select_id(&self, id: Id) -> Option<Backup> {
        let sql = format!(
            "SELECT id, tag, timestamp FROM {} WHERE id = ?",
            self.profile
        );
        let mut stmt = self.connection.prepare(&sql).expect("query failed");
        let mut iter = stmt
            .query_map(params![id], |row| {
                Ok(Backup::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .ok()?;
        match iter.next() {
            Some(Ok(backup)) => Some(backup),
            _ => None,
        }
    }

    /// Retrieve all backups.
    pub fn select_all(&self) -> Vec<Backup> {
        let sql = format!("SELECT id, tag, timestamp FROM {}", self.profile);
        let mut stmt = self.connection.prepare(&sql).expect("query failed");
        stmt.query_map(params![], |row| {
            Ok(Backup::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .expect("query failed")
        .filter_map(|res| res.ok())
        .collect()
    }

    /// Remove a backup with the given ID.
    pub fn remove(&self, id: Id) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE id = ?", self.profile);
        self.connection.execute(&sql, params![id])?;
        Ok(())
    }

    pub fn latest(&self) -> Option<Backup> {
        self.select_all()
            .into_iter()
            .max_by_key(|b| b.timestamp())
    }

    /// Returns the last inserted ID.
    fn last_id(&self) -> Id {
        self.connection
            .last_insert_rowid()
            .try_into()
            .expect("id overflow")
    }
}

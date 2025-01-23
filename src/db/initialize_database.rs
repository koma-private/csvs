use crate::db::add_regexp_function::add_regexp_function;
use rusqlite::config::DbConfig;
use tracing::debug;

/// Initialize SQLite database connection
pub fn initialize_database(
    out_database: &Option<String>,
) -> anyhow::Result<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>> {
    let manager = match out_database {
        None => {
            debug!("Using in-memory SQLite database.");
            r2d2_sqlite::SqliteConnectionManager::memory()
        }
        Some(path) => {
            debug!("Using SQLite database at path: {}", path);
            r2d2_sqlite::SqliteConnectionManager::file(path)
        }
    }
    .with_init(|conn| {
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER, true)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_VIEW, true)?;

        add_regexp_function(conn)
            .map_err(|err| rusqlite::Error::UserFunctionError(Box::from(err)))?;
        Ok(())
    });

    let pool = r2d2::Pool::new(manager)?;
    Ok(pool)
}

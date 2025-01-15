use crate::db::sqlite_quoted::SqliteQuoted;
use std::fmt::{Display, Formatter};

pub fn table_info(
    pool: &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>,
    table_name: &str,
) -> anyhow::Result<Vec<TableInfo>> {
    let conn = pool.get()?;

    let query = format!(
        "PRAGMA table_info({})",
        SqliteQuoted::Field(table_name.to_string()).get()
    );
    let mut stmt = conn.prepare(&query)?;
    let column_index_name = stmt.column_index("name")?;
    let column_index_data_type = stmt.column_index("type")?;
    let column_index_notnull = stmt.column_index("notnull")?;
    let column_index_dflt_value = stmt.column_index("dflt_value")?;
    let column_index_pk = stmt.column_index("pk")?;

    let mut table_infos: Vec<TableInfo> = Vec::new();
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let name: String = row.get(column_index_name)?;
        let data_type: String = row.get(column_index_data_type)?;
        let notnull: u8 = row.get(column_index_notnull)?;
        let dflt_value: Option<String> = row.get(column_index_dflt_value)?;
        let pk: u8 = row.get(column_index_pk)?;

        table_infos.push(TableInfo {
            name,
            data_type,
            notnull: notnull > 0,
            dflt_value,
            pk: pk > 0,
        });
    }

    Ok(table_infos)
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct TableInfo {
    pub name: String,
    pub data_type: String,
    pub notnull: bool,
    pub dflt_value: Option<String>,
    pub pk: bool,
}

impl Display for TableInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = format!(
            "{} {}{}{}{}",
            self.name,
            self.data_type,
            if self.notnull { " NOT NULL" } else { "" },
            if let Some(dflt_value) = &self.dflt_value {
                format!(" DEFAULT {}", dflt_value)
            } else {
                "".to_string()
            },
            if self.pk { " PK" } else { "" }
        );
        f.write_str(&text)
    }
}

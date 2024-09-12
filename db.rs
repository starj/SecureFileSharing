use std::env;
use rusqlite::{params, Connection, Result};

pub mod db_manager {
    use super::*;

    pub fn init_db() -> Result<Connection> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Connection::open(database_url)
    }

    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS items (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                value TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert_item(conn: &Connection, name: &str, value: &str) -> Result<()> {
        conn.execute(
            "INSERT INTO items (name, value) VALUES (?1, ?2)",
            params![name, value],
        )?;
        Ok(())
    }

    pub fn get_item(conn: &Connection, id: i32) -> Result<Option<(i32, String, String)>> {
        let mut stmt = conn.prepare("SELECT * FROM items WHERE id = ?1")?;
        let mut item_iter = stmt.query_map(params![id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        if let Some(result) = item_iter.next() {
            result
        } else {
            Ok(None)
        }
    }

    pub fn update_item(conn: &Connection, id: i32, new_value: &str) -> Result<()> {
        conn.execute(
            "UPDATE items SET value = ?1 WHERE id = ?2",
            params![new_value, id],
        )?;
        Ok(())
    }

    pub fn delete_item(conn: &Connection, id: i32) -> Result<()> {
        conn.execute(
            "DELETE FROM items WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let conn = db_manager::init_db()?;
    
    db_manager::create_table(&conn)?;
    db_manager::insert_item(&conn, "ExampleItem", "This is a test")?;
    
    if let Some(item) = db_manager::get_item(&conn, 1)? {
        println!("Retrieved item: {:?}", item);
    }

    Ok(())
}
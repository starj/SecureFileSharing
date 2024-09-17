use rusqlite::{params, Connection, Result};
use std::env;

mod db_manager {
    use rusqlite::{params, Connection, Result};
    use std::env;

    pub fn initialize_database_connection() -> Result<Connection> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Connection::open(database_url)
    }

    pub fn create_files_table(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                value TEXT
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert_file_record(connection: &Connection, file_name: &str, file_value: &str) -> Result<()> {
        connection.execute(
            "INSERT INTO files (name, value) VALUES (?1, ?2)",
            params![file_name, file_value],
        )?;
        Ok(())
    }

    pub fn get_file_by_id(connection: &Connection, file_id: i32) -> Result<Option<(i32, String, String)>> {
        let mut statement = connection.prepare("SELECT * FROM files WHERE id = ?1")?;
        
        let file_iter = statement.query_map(params![file_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        file_iter.next().transpose()
    }

    pub fn update_file_value(connection: &Connection, file_id: i32, new_file_value: &str) -> Result<()> {
        connection.execute(
            "UPDATE files SET value = ?1 WHERE id = ?2",
            params![new_file_value, file_id],
        )?;
        Ok(())
    }

    pub fn delete_file_by_id(connection: &Connection, file_id: i32) -> Result<()> {
        connection.execute(
            "DELETE FROM files WHERE id = ?1",
            params![file_id],
        )?;
        Ok(())
    }
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let database_connection = db_manager::initialize_database_connection()?;
    
    db_manager::create_files_table(&database_connection)?;
    db_manager::insert_file_record(&database_connection, "ExampleFile", "This is a test content")?;
    
    match db_manager::get_file_by_id(&database_connection, 1)? {
        Some(file) => println!("Retrieved file: {:?}", file),
        None => println!("File not found."),
    }

    Ok(())
}
mod models {
    use std::collections::HashMap;
    use std::env;

    pub struct User {
        pub id: u32,
        pub name: String,
        pub email: String,
        pub files: HashMap<u32, FileMetadata>,
    }

    pub struct FileMetadata {
        pub id: u32,
        pub name: String,
        pub size: u64,
        pub file_type: String,
        pub owner_id: u32,
    }

    impl User {
        pub fn new(id: u32, name: &str, email: &str) -> User {
            User { 
                id, 
                name: name.to_string(), 
                email: email.to_string(),
                files: HashMap::new(),
            }
        }

        pub fn add_file(&mut self, file: FileMetadata) {
            self.files.insert(file.id, file);
        }

        pub fn remove_file(&mut self, file_id: u32) {
            self.files.remove(&file_id);
        }
    }

    impl FileMetadata {
        pub fn new(id: u32, name: &str, size: u64, file_type: &str, owner_id: u32) -> FileMetadata {
            FileMetadata { 
                id, 
                name: name.to_string(), 
                size, 
                file_type: file_type.to_string(),
                owner_id,
            }
        }
    }

    pub fn db_connection_string() -> String {
        dotenv::dotenv().ok();

        env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".into())
    }
}
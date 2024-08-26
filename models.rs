mod models {
    use std::collections::HashMap;
    use std::env;

    pub struct User {
        pub id: u32,
        pub username: String, // Changed from 'name' to 'username' for clarity
        pub email: String,
        pub owned_files: HashMap<u32, FileMetadata>, // Changed from 'files' to 'owned_files' for clarity
    }

    pub struct FileMetadata {
        pub id: u32,
        pub filename: String, // Changed from 'name' to 'filename' for clarity
        pub byte_size: u64, // Changed from 'size' to 'byte_size' for clarity about the unit of measurement
        pub content_type: String, // Changed from 'file_type' to 'content_type' for clarity and to avoid confusion with Rust's type system
        pub owner_user_id: u32, // Changed from 'owner_id' to 'owner_user_id' for explicit relation to User
    }

    impl User {
        // Constructor method now more clearly names parameters
        pub fn new(user_id: u32, username: &str, user_email: &str) -> User {
            User { 
                id: user_id, 
                username: username.to_string(), 
                email: user_email.to_string(),
                owned_files: HashMap::new(),
            }
        }

        // Method name remains unchanged but the parameter name is clearer
        pub fn add_file(&mut self, new_file: FileMetadata) {
            self.owned_files.insert(new_file.id, new_file);
        }

        // Parameter name changed for clarity
        pub fn remove_file_by_id(&mut self, target_file_id: u32) {
            self.owned_files.remove(&target_file_id);
        }
    }

    impl FileMetadata {
        // Constructor method with parameters named for clarity
        pub fn new(file_id: u32, filename: &str, file_size: u64, content_type: &str, file_owner_user_id: u32) -> FileMetadata {
            FileMetadata { 
                id: file_id, 
                filename: filename.to_string(), 
                byte_size: file_size, 
                content_type: content_type.to_string(),
                owner_user_id: file_owner_user_id,
            }
        }
    }

    pub fn get_database_connection_string() -> String { // Changed for clarity on the action performed
        dotenv::dotenv().ok();

        env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".into())
    }
}
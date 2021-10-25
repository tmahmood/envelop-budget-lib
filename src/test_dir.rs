

// Creates temporary directory and deletes when exits scope
#[cfg(test)]
pub mod test_dir {
    use std::fs;
    use std::path::Path;
    use fs::remove_dir_all;
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    pub struct TestDir {
        // private field to enforce use of Context::new()
        pub folder_name: String,
    }

    impl TestDir {
        pub fn new() -> Self {
            // Add initialization code here.
            let folder_name = loop {
                let folder_name: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(7)
                    .map(char::from)
                    .collect();
                let path = format!("test_dirs/{}", folder_name);
                if !Path::exists(Path::new(&path)) {
                    fs::create_dir(path).unwrap();
                    break folder_name;
                }
            };
            TestDir { folder_name: folder_name.to_string() }
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            // Add cleanup code here
            println!("Deleting folder: test_dirs/{}", self.folder_name);
            remove_dir_all(Path::new(&format!("test_dirs/{}", self.folder_name)));
        }
    }
}

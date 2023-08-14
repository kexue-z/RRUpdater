#[cfg(test)]

mod tests {
    #[test]
    fn test_new_find() {
        use std::path::PathBuf;

        #[derive(Debug, Clone)]
        pub struct FileData {
            pub name: String,
            pub path: PathBuf,
            pub sha1: String,
        }

        fn find_differences(a: &[FileData], b: &[FileData]) -> (Vec<FileData>, Vec<FileData>) {
            let mut missing_files: Vec<FileData> = Vec::new();
            let mut extra_files: Vec<FileData> = Vec::new();

            // Find missing files in b compared to a
            for file_a in a {
                let mut found = false;
                for file_b in b {
                    if file_a.name == file_b.name
                        && file_a.path == file_b.path
                        && file_a.sha1 == file_b.sha1
                    {
                        found = true;
                        break;
                    }
                }
                if !found {
                    missing_files.push(file_a.clone());
                }
            }

            // Find extra files in b compared to a
            for file_b in b {
                let mut found = false;
                for file_a in a {
                    if file_b.name == file_a.name
                        && file_b.path == file_a.path
                        && file_b.sha1 == file_a.sha1
                    {
                        found = true;
                        break;
                    }
                }
                if !found {
                    extra_files.push(file_b.clone());
                }
            }

            (missing_files, extra_files)
        }

        let a = vec![
            FileData {
                name: "a.txt".to_string(),
                path: PathBuf::from("a.txt"),
                sha1: "a".to_string(),
            },
            FileData {
                name: "b.txt".to_string(),
                path: PathBuf::from("dir/b.txt"),
                sha1: "b".to_string(),
            },
        ];

        let b = vec![
            FileData {
                name: "b.txt".to_string(),
                path: PathBuf::from("dir/b.txt"),
                sha1: "b".to_string(),
            },
            FileData {
                name: "c.txt".to_string(),
                path: PathBuf::from("dir/c.txt"),
                sha1: "c".to_string(),
            },
        ];

        let (missing_files, extra_files) = find_differences(&a, &b);

        println!("Missing Files:");
        for file in missing_files {
            println!("{:?}", file);
        }

        println!("Extra Files:");
        for file in extra_files {
            println!("{:?}", file);
        }
    }
}

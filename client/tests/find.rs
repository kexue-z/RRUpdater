#[cfg(test)]

mod tests {
    #[test]
    fn test_new_find() {
        use client::utils::find_differences;
        use file_patcher::FileData;
        use std::path::PathBuf;

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

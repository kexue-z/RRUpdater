#[cfg(test)]

mod tests {
    #[test]
    fn test_find_misssing_items() {
        use client::utils::find_missing_items;
        use file_patcher::{FileData, FilePatcher};

        use std::path::Path;

        let f1 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![
                FileData {
                    name: "ff1.txt".to_string(),
                    path: Path::new("ff1.txt").to_path_buf(),
                    sha1: "abc".to_string(),
                },
                FileData {
                    name: "ff2.txt".to_string(),
                    path: Path::new("ff2.txt").to_path_buf(),
                    sha1: "cba".to_string(),
                },
            ],
        };

        let missing_item = FileData {
            name: "ff1.txt".to_string(),
            path: Path::new("ff1.txt").to_path_buf(),
            sha1: "abc".to_string(),
        };

        let f2 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![FileData {
                name: "ff2.txt".to_string(),
                path: Path::new("ff2.txt").to_path_buf(),
                sha1: "cba".to_string(),
            }],
        };

        let missing_items = find_missing_items(&f1, &f2);

        assert_eq!(missing_item.sha1, missing_items.iter().next().unwrap().sha1);
    }

    #[test]
    fn test_find_surplu_items() {
        use client::utils::find_surplus_items;
        use file_patcher::{FileData, FilePatcher};

        use std::path::Path;

        let f1 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![FileData {
                name: "ff2.txt".to_string(),
                path: Path::new("ff2.txt").to_path_buf(),
                sha1: "cba".to_string(),
            }],
        };

        let surplus_item = FileData {
            name: "ff1.txt".to_string(),
            path: Path::new("ff1.txt").to_path_buf(),
            sha1: "abc".to_string(),
        };

        let f2 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![
                FileData {
                    name: "ff1.txt".to_string(),
                    path: Path::new("ff1.txt").to_path_buf(),
                    sha1: "abc".to_string(),
                },
                FileData {
                    name: "ff2.txt".to_string(),
                    path: Path::new("ff2.txt").to_path_buf(),
                    sha1: "cba".to_string(),
                },
            ],
        };

        let surplus_items = find_surplus_items(&f1, &f2);

        assert_eq!(surplus_item.sha1, surplus_items.iter().next().unwrap().sha1);
    }

    #[test]
    fn test_find_items() {
        use client::utils::find_items;
        use file_patcher::{FileData, FilePatcher};

        use std::path::Path;

        let f1 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![
                FileData {
                    name: "ff2.txt".to_string(),
                    path: Path::new("ff2.txt").to_path_buf(),
                    sha1: "cba".to_string(),
                },
                FileData {
                    name: "ff3.txt".to_string(),
                    path: Path::new("ff3.txt").to_path_buf(),
                    sha1: "dfg".to_string(),
                },
            ],
        };

        let surplus_item = FileData {
            name: "ff1.txt".to_string(),
            path: Path::new("ff1.txt").to_path_buf(),
            sha1: "abc".to_string(),
        };

        let missing_item = FileData {
            name: "ff3.txt".to_string(),
            path: Path::new("ff3.txt").to_path_buf(),
            sha1: "dfg".to_string(),
        };

        let f2 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![
                FileData {
                    name: "ff1.txt".to_string(),
                    path: Path::new("ff1.txt").to_path_buf(),
                    sha1: "abc".to_string(),
                },
                FileData {
                    name: "f2.txt".to_string(),
                    path: Path::new("f2.txt").to_path_buf(),
                    sha1: "cba".to_string(),
                },
            ],
        };

        let items = find_items(f1, f2);

        assert_eq!(missing_item.sha1, items.missing.iter().next().unwrap().sha1);
        assert_eq!(surplus_item.sha1, items.surplus.iter().next().unwrap().sha1);
    }
}

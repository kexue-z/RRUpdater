#[cfg(test)]

mod tests {
    #[test]
    fn test_find_misssing_itmes() {
        use client::utils::find_missing_items;
        use file_patcher::{FileData, FilePatcher};

        use std::path::Path;

        let f1 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![
                FileData {
                    name: "ff1".to_string(),
                    path: Path::new("a/1").to_path_buf(),
                    sha1: "abc".to_string(),
                },
                FileData {
                    name: "ff2".to_string(),
                    path: Path::new("a/2").to_path_buf(),
                    sha1: "cba".to_string(),
                },
            ],
        };

        let missing_item = FileData {
            name: "ff1".to_string(),
            path: Path::new("a/1").to_path_buf(),
            sha1: "abc".to_string(),
        };

        let f2 = FilePatcher {
            name: "1".to_string(),
            path: Path::new("a").to_path_buf(),
            file_data: vec![FileData {
                name: "ff2".to_string(),
                path: Path::new("a/2").to_path_buf(),
                sha1: "cba".to_string(),
            }],
        };

        let missing_items = find_missing_items(&f1, &f2);

        assert_eq!(missing_item.sha1, missing_items.iter().next().unwrap().sha1);
    }
}

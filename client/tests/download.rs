#[cfg(test)]

mod tests {
    #[test]
    fn test_download() {
        let urls = vec![
            Url::parse("https://example.com/file1.txt").unwrap(),
            Url::parse("https://example.com/file2.txt").unwrap(),
            Url::parse("https://example.com/file3.txt").unwrap(),
        ];

        let download_dir = PathBuf::from("/path/to/download/directory");

        download_files(urls, download_dir);
    }
}

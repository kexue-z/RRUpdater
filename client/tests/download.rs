#[cfg(test)]

mod tests {

    #[test]
    fn test_download() {
        use client::downloader::download_file;
        use reqwest::Url;
        use std::path::Path;

        let urls = vec![Url::parse("http://127.0.0.1:8520/files/cellphone.svg").unwrap()];

        let download_dir = Path::new("./abc.svg");
        urls.into_iter().for_each(|url| {
            download_file(url, download_dir);
        });
    }
}

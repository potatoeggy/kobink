use epub::doc::EpubDoc;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::kobo::library::models::DownloadUrl;

const DEFAULT_PROTOCOL: &'static str = "http";
const DEFAULT_HOST: &'static str = "192.168.2.65:3000";

#[derive(Debug, Clone)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>, // are multiple authors allowed?
    pub description: String,
    pub path: PathBuf,
    pub size: u64,
}

impl Book {
    pub fn download_url(&self) -> DownloadUrl {
        DownloadUrl {
            Format: "EPUB".to_string(),
            Size: self.size,
            Url: format!(
                "{}://{}/download/{book_id}/{book_format}/{filename}",
                DEFAULT_PROTOCOL,
                DEFAULT_HOST,
                book_id = self.id,
                book_format = "EPUB",
                filename = self.path.file_name().unwrap().to_str().unwrap()
            ),
            Platform: "Generic".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LibraryState {
    pub path: String,
    pub books: Vec<Book>,
}

impl LibraryState {
    pub fn new(library_path: &str) -> Self {
        let mut library = Self {
            path: library_path.to_string(),
            books: vec![],
        };

        library.load_books();
        library
    }

    pub fn find_book_by_id(&self, id: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.id.to_string() == id)
    }

    // TODO: watch the library path for changes
    pub fn load_books<'a>(&mut self) {
        // recursively walk the library path and load all the books
        for path in WalkDir::new(&self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "epub"))
        {
            let Ok(epub) = EpubDoc::new(path.path()) else {
                tracing::info!("failed to load epub at {:?}", path.path());
                continue;
            };

            let title = epub.mdata("title").unwrap_or("Untitled".to_string());

            // we can use metadata.get("creator") to get multiple authors
            let author = epub
                .mdata("creator")
                .unwrap_or("Unknown author".to_string());
            let description = epub.mdata("description").unwrap_or("".to_string());

            let mut seed = [0u8; 16];
            let seed_key = format!("{}{}", title, author);
            let seed_source = seed_key.as_bytes();
            let copy_len = std::cmp::min(seed.len(), seed_source.len());
            seed[..copy_len].copy_from_slice(&seed_source[..copy_len]);

            self.books.push(Book {
                id: Uuid::from_bytes(seed),
                title,
                authors: vec![author],
                description,
                path: path.path().to_path_buf(),
                size: path.metadata().unwrap().len(),
            });
        }
        println!("loaded {:?} books", self.books);
    }
}

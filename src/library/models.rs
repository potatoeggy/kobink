use epub::doc::EpubDoc;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<String>, // are multiple authors allowed?
    pub description: String,
    pub path: PathBuf,
}

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

    // TODO: watch the library path for changes
    pub fn load_books(&mut self) {
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
            self.books.push(Book {
                id: Uuid::new_v4(),
                title,
                authors: vec![author],
                description,
                path: path.path().to_path_buf(),
            });
        }
        println!("loaded {:?} books", self.books);
    }
}
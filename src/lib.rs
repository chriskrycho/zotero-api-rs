//! Handle Zotero's API data structures.
//!
//! **Note:** this does not implement Zotero's API itself, *only* handling for
//! taking its responses and converting them from JSON into Rust representations
//! of those types.

use std::collections::HashMap;

struct Library {}

struct CollectionId {}

struct Url(String);

struct ISBN(String);

struct Date(String);

enum Name {
    Split {
        first: Option<String>,
        last: Option<String>,
    },
    Merged(String),
}

enum Creator {
    Author(Name),
    Contributor(Name),
    Editor(Name),
    SeriesEditor(Name),
    Translator(Name),
}

enum Item {
    Artwork {
        title: String,
        creators: Vec<Creator>,
        abstract_note: Option<String>,
        artwork_medium: Option<String>, // ?
        artwork_size: Option<String>, // ?
        date: Option<String>,
        language: Option<String>,
        short_title: Option<String>,
        archive: Option<String>,
        archive_location: Option<String>,
        library_catalog: Option<String>,
        call_number: Option<String>,
        url: Option<Url>,
        access_date: Option<Date>,
        rights: Option<String>,
        extra: Option<String>,
        tags: Vec<String>,
        collections: Vec<CollectionId>,
        relations: HashMap<String, Box<dyn AsRef<str>>>,
    },
    AudioRecording,
    Bill,
    BlogPost,
    Book {
        title: String,
        creators: Vec<Creator>,
        abstract_note: Option<String>,
        series: Option<String>,
        series_number: Option<String>,
        volume: Option<String>,
        number_of_volumes: u16,
        edition: Option<String>,
        place: Option<String>,
        publisher: Option<String>,
        date: Option<String>,
        num_pages: Option<u16>,
        language: Option<String>,
        isbn: Option<ISBN>,
        short_title: Option<String>,
        url: Option<Url>,
        access_date: Option<Date>,
        archive: Option<String>,
        archive_location: Option<String>,
        library_catalog: Option<String>,
        call_number: Option<String>,
        rights: Option<String>,
        extra: Option<String>,
        tags: Vec<String>,
        collections: Vec<CollectionId>,
        relations: HashMap<String, Box<dyn AsRef<str>>>,
    },
    BookSection {
        title: String,
        book_title: String,
        creators: Vec<Creator>,
        abstract_note: Option<String>,
        series: Option<String>,
        series_number: Option<String>,
        volume: Option<String>,
        number_of_volumes: u16,
        edition: Option<String>,
        place: Option<String>,
        publisher: Option<String>,
        date: Option<String>,
        num_pages: Option<u16>,
        language: Option<String>,
        isbn: Option<ISBN>,
        short_title: Option<String>,
        url: Option<Url>,
        access_date: Option<Date>,
        archive: Option<String>,
        archive_location: Option<String>,
        library_catalog: Option<String>,
        call_number: Option<String>,
        rights: Option<String>,
        extra: Option<String>,
        tags: Vec<String>,
        collections: Vec<CollectionId>,
        relations: HashMap<String, Box<dyn AsRef<str>>>,
    },
    Case,
    ComputerProgram,
    ConferencePaper,
    DictionaryEntry,
    Document,
    Email,
    EncyclopediaArticle,
    Film,
    ForumPost,
    Hearing,
    InstantMessage,
    Interview,
    JournalArticle,
    Letter,
    MagazineArticle,
    Manuscript,
    Map,
    NewspaperArticle,
    Note,
    Patent,
    Podcast,
    Presentation,
    RadioBroadcast,
    Report,
    Statute,
    TvBroadcast,
    Thesis,
    VideoRecording,
    Webpage,
}

impl Item {
    // TODO: actually use locale! This is just for en-US.
    fn localized(&self, _locale: &str) -> String {
        match self {
            Self::Artwork => "Artwork",
            Self::AudioRecording => "Audio Recording",
            Self::Bill => "Bill",
            Self::BlogPost => "Blog Post",
            Self::Book => "Book",
            Self::BookSection => "Book Section",
            Self::Case => "Case",
            Self::ComputerProgram => "Computer Program",
            Self::ConferencePaper => "Conference Paper",
            Self::DictionaryEntry => "Dictionary Entry",
            Self::Document => "Document",
            Self::Email => "E-mail",
            Self::EncyclopediaArticle => "Encyclopedia Article",
            Self::Film => "Film",
            Self::ForumPost => "Forum Post",
            Self::Hearing => "Hearing",
            Self::InstantMessage => "Instant Message",
            Self::Interview => "Interview",
            Self::JournalArticle => "Journal Article",
            Self::Letter => "Letter",
            Self::MagazineArticle => "Magazine Article",
            Self::Manuscript => "Manuscript",
            Self::Map => "Map",
            Self::NewspaperArticle => "Newspaper Article",
            Self::Note => "Note",
            Self::Patent => "Patent",
            Self::Podcast => "Podcast",
            Self::Presentation => "Presentation",
            Self::RadioBroadcast => "Radio Broadcast",
            Self::Report => "Report",
            Self::Statute => "Statute",
            Self::TvBroadcast => "TV Broadcast",
            Self::Thesis => "Thesis",
            Self::VideoRecording => "Video Recording",
            Self::Webpage => "Web Page",
        }.into()
    }
}

struct Item {
    /// The Zotero identifier for the item.
    key: String,

    /// The version (revision count) of the item.
    version: u64,

    /// The ID of the library to which the item belongs.
    library_id: u64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

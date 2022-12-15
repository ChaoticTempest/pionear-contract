use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::lookup_map::Entry;
use near_sdk::store::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

type Isbn = String;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct Book {
    title: String,
    author: String,
    desc: String,
    user_submitted_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct BookDatabase {
    books: LookupMap<Isbn, Book>,
    manager: AccountId,
}

#[near_bindgen]
impl BookDatabase {
    #[init]
    pub fn new() -> Self {
        Self {
            books: LookupMap::new(b"b"),
            manager: env::predecessor_account_id(),
        }
    }

    // adds a new book to the tracker
    pub fn add_book(
        &mut self,
        isbn: String,
        title: String,
        author: String,
        desc: String,
    ) -> String {
        match self.books.entry(isbn) {
            Entry::Occupied(entry) => {
                format!("Book for isbn={} already exists", entry.key())
            }
            Entry::Vacant(entry) => {
                let msg = format!("Successfully added book with isbn={}", entry.key());
                entry.insert(Book {
                    title,
                    author,
                    desc,
                    user_submitted_id: env::predecessor_account_id(),
                });

                msg
            }
        }
    }

    /// get a book based on the isbn
    pub fn get(&self, isbn: String) -> Option<Book> {
        self.books.get(&isbn).cloned()
    }
}

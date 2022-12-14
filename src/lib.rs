use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::lookup_map::Entry;
use near_sdk::store::{LookupMap, Vector};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BookTracker {
    books: LookupMap<AccountId, Vector<Book>>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Book {
    title: String,
    author: String,
    desc: String,
}

impl Default for BookTracker {
    fn default() -> Self {
        Self {
            books: LookupMap::new(b"b"),
        }
    }
}

#[near_bindgen]
impl BookTracker {
    // adds a new book to the tracker
    pub fn add_book(&mut self, title: String, author: String, desc: String) {
        let id = env::predecessor_account_id();
        let new_book = Book {
            title,
            author,
            desc,
        };

        match self.books.entry(id) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(new_book);
            }
            Entry::Vacant(entry) => {
                entry.insert({
                    let prefix = env::predecessor_account_id().to_string();
                    let mut vec = Vector::new(prefix.as_bytes());
                    vec.push(new_book);
                    vec
                });
            }
        }
    }
}

# Pionear -- Book DB

Minimal database to keep track of books on chain via an ISBN.

### Change Methods
```
add_book(isbn: String, title: String, author: String, desc: String)
```

### View Methods
```
get(isbn: String) -> Option<Book>
```


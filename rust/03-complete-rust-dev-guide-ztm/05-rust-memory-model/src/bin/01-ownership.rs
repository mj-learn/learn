struct Book {
    pages: i32,
    rating: f64,
}

fn print_book_pages(book: &Book) {
    println!("pages = {}", book.pages);
}

fn print_book_rating(book: &Book) {
    println!("rating = {}", book.rating);
}

fn main() {
    let my_book = Book {
        pages: 12,
        rating: 9.8,
    };

    print_book_pages(&my_book);
    print_book_rating(&my_book);
}

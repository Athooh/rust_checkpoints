pub mod library {
    pub mod writers {
        use super::books::Book;

        #[derive(Debug)]
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }

    pub mod books {
        #[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
        pub struct Book {
            pub title: String,
            pub year: u64,
        }
    }
}

pub fn order_books(writer: &mut library::writers::Writer) {
    writer.books.sort();
}
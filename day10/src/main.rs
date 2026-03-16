struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

impl Book {
    fn show_info(&self) {
        println!("《{}》 by {}", self.title, self.author);
        println!("Pages: {}, Available: {}", self.pages, self.available);
        println!("---");
        if self.available {
            println!("《{}》is available", self.title);
        } else {
            println!("《{}》is borrowed", self.title);
        }
    }
}

fn main() {
    let mut book = Book {
        title: String::from("Serial Experiments Lain"),
        author: String::from("Yoshitoshi ABe"),
        pages: 256,
        available: true,
    };

    book.show_info();

    println!("\n=======Borrowing the book...=======\n");

    book.available = false;
    book.show_info();
}

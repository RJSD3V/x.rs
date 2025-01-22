#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            //Good ! We have something to return
            Some(&self.items[index])
        } else {
            // Bad! We don't have anything to return !!
            None
        }
    }
}

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {} ", title)
        // } else {
        //     format!("Media Format Invalid")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }

            Media::Audiobook { title } => {
                format!("Audiobook {} ", title)
            }

            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }

            Media::Podcast(id) => {
                format!("Podcast {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Indistractible"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let good_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    let podcast = Media::Podcast(10);

    let placeholder = Media::Placeholder;
    //value is not moved if you call a function on it. ONly happens if you pass it as an argument to a method.
    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", good_book.description());
    //
    let mut catalog = Catalog::new();

    catalog.add(good_movie);
    catalog.add(audiobook);
    catalog.add(good_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(110);
    let placeholder = Media::Placeholder;
    println!("{:#?}", item.unwrap_or(&placeholder));
}

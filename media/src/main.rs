mod content;

use content::catalog::Catalog;
use content::media::Media;

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

    let item = catalog.get_by_index(1);
    let placeholder = Media::Placeholder;
    println!("{:#?}", item.unwrap_or(&placeholder));
}

use std::collections::HashMap;

fn main(){

    let mut book_reviews = HashMap::new();
    //let mut book_reviews = HashMap<String,String>::new(); 会自动类型推导，可以省略范型参数
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );

    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );

    //contains_key(key)   remove(key)  get(key)

    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    for key in book_reviews.keys() {
        println!("{}", key);
    }

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }

    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);

    // The easiest way to use HashMap with a custom key type is to derive Eq and Hash. We must also derive PartialEq.

}


// Ownership example
fn ownership_example() {
    let x = 40; // 'x' is the owner of the value '40'
    let y = x; // Ownership of the value is moved from 'x' to 'y'
    println!("x = {}", x); // Error: borrow of moved value: `x`
}

// Ownership Transfer with Functions
fn take_ownership(s: String) {
    println!("The string is: {}", s);
}


fn ownership_transfer_func() {
    let string_1 = String::from("hello");
    let string_2 = string_1; // Ownership of the string is moved from 'string_1' to 'string_2'
    take_ownership(string_2); // In this case, the ownership of `string_2` is transferred to `takes_ownership`. After `takes_ownership` is done, `string_2 ` is no longer valid.
    // println!("The string is: {}", string_2); // Error: borrow of moved value: `string_2`
}

// Immutable Borrowing and Mutable Borrowing

struct Book {
    title: String,
    price: f64,
}

// Immutable Borrowing
fn immutable_borrowing() {
    let book1 = Book {
        title: String::from("The Great Gatsby"),
        price: 12.99,
    };

    let book_price = &book1.price; // Immutable borrow of the book's price
    println!("The price of '{}' is ${}", book1.title, book_price);
}

// Mutable Borrowing
fn mutable_borrowing() {
    let mut book1 = Book {
        title: String::from("The Great Gatsby"),
        price: 12.99,
    };

    {
        let book_price = &mut book1.price; // Mutable borrow of the book's price
        *book_price = 14.99; // Modifying the price through the mutable borrow
    } // The mutable borrow is dropped here

    println!("The new price of '{}' is ${}", book1.title, book1.price);
}

// Lifetimes
fn lifetimes() {
    let x = 40;
    let y = &x;
    println!("x = {}, y = {}", x, y);
}

fn main() {
    // Copying occurs when the value is simple and cheap to duplicate.
    // Ownership transfer occurs when the value is more complex and cannot be easily copied.
    
    let a = 11;
    let b = a;
    println!("a = {}", a); // Error: borrow of moved value: `a`

    ownership_example();
    ownership_transfer();
    immutable_borrowing_example();
    mutable_borrowing_example();
    lifetimes_example();
}
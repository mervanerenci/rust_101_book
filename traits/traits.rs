// Defining a Trait
// Traits are a way to define shared behavior in Rust. 
//They allow you to define a set of methods that a type must implement to be considered a member of that trait.

// To define a trait, you use the trait keyword followed by the name of the trait and a block of method signatures.
// The `Animal` trait defines a contract that any type implementing this trait must follow.
// It includes two methods: `make_sound` and `get_num_legs`.
trait Animal {
    fn make_sound(&self);
    fn get_num_legs(&self) -> u32;
}

// Implementing a Trait
// To implement a trait for a type, you use the impl keyword followed by the trait name and a block of method implementations.
// The `Dog` and `Cat` structs implement the `Animal` trait, providing their own
// implementations of the required methods.
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says: Woof!", self.name);
    }

    fn get_num_legs(&self) -> u32 {
        4
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("{} says: Meow!", self.name);
    }

    fn get_num_legs(&self) -> u32 {
        4
    }
}

// Trait Bounds
// Trait bounds are a way to constrain the types that can be used with generic functions or structs. 
// They ensure that the type passed as a parameter or used in a generic context implements the required traits
 
// The `print_animal_info` function demonstrates the use of trait bounds.
// The `<T: Animal>` syntax ensures that the type `T` implements the `Animal` trait,
// allowing us to call the trait methods on the `animal` parameter.
fn print_animal_info<T: Animal>(animal: &T) {
    animal.make_sound();
    println!("{} has {} legs.", animal.get_num_legs());
}

// Default Trait Methods
// Traits can include default method implementations, which provide a default behavior for types implementing the trait.
//  

// The `Vehicle` trait includes a default implementation of the `display_info` method.
// Implementing types can choose to override this method or use the default implementation.
trait Vehicle {
    fn get_make(&self) -> &str;
    fn get_model(&self) -> &str;

    fn display_info(&self) {
        println!("This is a {} {}", self.get_make(), self.get_model());
    }
}

struct Car {
    make: String,
    model: String,
}

// Implementing the `Vehicle` trait for the `Car` struct.
// The `get_make` and `get_model` methods are required by the trait,
// while the `display_info` method is optional and uses the default implementation.
impl Vehicle for Car {
    fn get_make(&self) -> &str {
        &self.make
    }

    fn get_model(&self) -> &str {
        &self.model
    }

    // The `display_info` method is optional and uses the default implementation.
    // If a type implementing the `Vehicle` trait does not provide its own implementation,
    // this default method will be used.
    // If a type does provide its own implementation, that implementation will be used instead. For example below code,

    // fn display_info(&self) {
    //     // Custom implementation
    //     println!("This is a custom implementation for the Car struct.");
    // }

}



// Trait Objects
// Trait objects are a way to work with different types that implement the same trait through a single reference.
// They allow you to store different types in a collection or pass them as function parameters without knowing their concrete types.

fn main() {
    // Creating instances of `Dog` and `Cat`, which implement the `Animal` trait.
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };

    // Calling the `print_animal_info` function with the `Dog` and `Cat` instances.
    // The function can work with any type that implements the `Animal` trait.
    print_animal_info(&dog);
    print_animal_info(&cat);

    // Creating a vector of trait objects that can hold any type implementing the `Animal` trait.
    let mut animals: Vec<&dyn Animal> = Vec::new();
    animals.push(&dog);
    animals.push(&cat);

    // Iterating over the vector and calling the `make_sound` method on each element.
    // We can then call the make_sound() method on each animal, even though they are of different underlying types.
    for animal in animals {
        animal.make_sound();
    }


    // Creating an instance of `Car`, which implements the `Vehicle` trait.
    let car = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
    };

    // Calling the default `display_info` method from the `Vehicle` trait.
    car.display_info();
}
// Define a trait that requires implementing a method to render the item
trait Renderable {
    fn render(&self) -> String;
}

// Define a struct for a User
struct User {
    username: String,
    email: String,
}

impl Renderable for User {
    fn render(&self) -> String {
        format!("User: {} ({})", self.username, self.email)
    }
}

// Define a struct for a Product
struct Product {
    name: String,
    price: f64,
}

impl Renderable for Product {
    fn render(&self) -> String {
        format!("Product: {} - ${:.2}", self.name, self.price)
    }
}

// This generic function accepts any item that implements the Renderable trait
fn print_rendered<T: Renderable>(item: &T) {
    println!("{}", item.render());
}

// You can also use the `impl Trait` syntax for conciseness
fn print_rendered_alt(item: &impl Renderable) {
    println!("[ALT] {}", item.render());
}

fn main() {
    // Create some instances
    let user = User {
        username: "alice123".to_string(),
        email: "alice@example.com".to_string(),
    };

    let product = Product {
        name: "Rust Book".to_string(),
        price: 29.99,
    };

    // Call the generic functions
    print_rendered(&user);
    print_rendered(&product);

    // Or use the impl Trait version
    print_rendered_alt(&user);
    print_rendered_alt(&product);
}

// Trait defining a common interface for renderable items
trait Renderable {
    fn render(&self) -> String;
}

// Struct representing a system user
struct User {
    username: String,
    email: String,
}

impl Renderable for User {
    fn render(&self) -> String {
        format!("User: {} <{}>", self.username, self.email)
    }
}

// Struct representing a product
struct Product {
    name: String,
    price: f64,
}

impl Renderable for Product {
    fn render(&self) -> String {
        format!("Product: {} - ${:.2}", self.name, self.price)
    }
}

// Struct representing a system log message
struct SystemLog {
    level: String,
    message: String,
}

impl Renderable for SystemLog {
    fn render(&self) -> String {
        format!("[{}] {}", self.level, self.message)
    }
}

// Generic function to print any slice of Renderable items
fn print_all<T: Renderable>(items: &[T]) {
    for item in items {
        println!("{}", item.render());
    }
}

// Generic function for single items using `impl Trait` syntax
fn print_rendered(item: &impl Renderable) {
    println!("{}", item.render());
}

fn main() {
    // Sample users and products
    let user1 = User {
        username: "alice123".to_string(),
        email: "alice@example.com".to_string(),
    };

    let product1 = Product {
        name: "Rust Book".to_string(),
        price: 29.99,
    };

    let product2 = Product {
        name: "USB Keyboard".to_string(),
        price: 49.95,
    };

    // Sample log messages
    let logs = vec![
        SystemLog {
            level: "INFO".to_string(),
            message: "System started successfully".to_string(),
        },
        SystemLog {
            level: "ERROR".to_string(),
            message: "Failed to connect to database".to_string(),
        },
        SystemLog {
            level: "WARN".to_string(),
            message: "Low disk space".to_string(),
        },
    ];

    println!("--- Single Items ---");
    print_rendered(&user1);
    print_rendered(&product1);

    println!("\n--- Logs ---");
    print_all(&logs);

    println!("\n--- Products ---");
    print_all(&[product1, product2]);
}

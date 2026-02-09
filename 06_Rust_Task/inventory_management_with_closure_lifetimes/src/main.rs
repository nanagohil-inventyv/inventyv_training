use std::collections::HashMap;
use std::fmt;

pub trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Debug)]
pub struct Product<'a> {
    name: &'a str,
    price: f64,
}

impl<'a> DisplayItem for Product<'a> {
    fn display(&self) -> String {
        format!("Product : {} , Price ₹ {:.2}", self.name, self.price)
    }
}

pub struct Inventory<'a, T>
where
    T: DisplayItem + 'a,
{
    items: HashMap<String, &'a T>,
}

#[derive(Debug)]
pub enum InventoryError {
    DuplicateId(String),
    InvalidId,
    ItemNotFound(String),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::DuplicateId(id) => {
                write!(f, "Item with id {id} already exists")
            }
            InventoryError::InvalidId => write!(f, "Invalid item Id"),
            InventoryError::ItemNotFound(id) => {
                write!(f, "Item with id {id} not found")
            }
        }
    }
}

impl<'a, T> Inventory<'a, T>
where
    T: DisplayItem + 'a,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, id: String, item: &'a T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }

        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId(id));
        }

        self.items.insert(id, item);
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Result<&'a T, InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }

        self.items
            .get(id)
            .copied()
            .ok_or_else(|| InventoryError::ItemNotFound(id.to_string()))
    }

    pub fn display_all<F>(&self, f: F) -> String
    where
        F: Fn(&String, &T) -> String,
    {
        if self.items.is_empty() {
            return "Inventory is Empty".to_string();
        }

        let mut output = String::new();

        for (id, item) in &self.items {
            output.push_str(&f(id, item));
            output.push_str("\n-------------------\n");
        }

        output
    }
}

fn main() {
    let p1 = Product {
        name: "Laptop",
        price: 60000.0,
    };

    let p2 = Product {
        name: "KeyBoard",
        price: 1000.0,
    };

    let p3 = Product {
        name: "Mouse",
        price: 500.0,
    };

    let mut inventory = Inventory::new();

    inventory.add_item("P01".to_string(), &p1).unwrap();
    inventory.add_item("P02".to_string(), &p2).unwrap();

    // duplicate check
    match inventory.add_item("P01".to_string(), &p3) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }

    // get item not exists
    match inventory.get_item("P03") {
        Ok(item) => println!("{}", item.display()),
        Err(e) => println!("Error: {}", e),
    }

    // get item success
    match inventory.get_item("P01") {
        Ok(item) => println!("{:?}", item),
        Err(e) => println!("Error: {}", e),
    }

    let all_items = inventory.display_all(|id, item| {
        format!("ID: {}\n{}", id, item.display())
    });

    println!("{}", all_items);
}


use std::fmt;

use std::collections::HashMap;

pub trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Product {
    name: String,
    price: f64,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("Product : {} , Price ₹ {:.2}", self.name, self.price)
    }
}

pub struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}

#[derive(Debug)]
pub enum InventoryError {
    Duplicatid(String),
    InvalidId,
    ItemNotFound(String),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::Duplicatid(id) => {
                write!(f, "Item with id {id} already exists")
            }
            InventoryError::InvalidId => {
                write!(f, "Invalid item Id")
            }

            InventoryError::ItemNotFound(id) => {
                write!(f, "Item with id {id} not found")
            }
        }
    }
}

impl<T> Inventory<T>
where
    T: DisplayItem + Clone,
{
    // Add with duplicate check.

    pub fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }
        if self.items.contains_key(&id) {
            return Err(InventoryError::Duplicatid(id));
        }

        self.items.insert(id, item);

        Ok(())
    }

    pub fn get_item(&self, id: String) -> Result<T, InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }
        self.items
            .get(&id)
            .cloned()
            .ok_or_else(|| InventoryError::ItemNotFound(id))
    }

    pub fn display_all(&self) -> String {
        if self.items.is_empty() {
            return "Inventory is Empty".to_string();
        }

        let mut output = String::new();

        for (id, item) in &self.items {
            output.push_str(&format!(
                "ID: {}\n{}\n-------------------\n",
                id,
                item.display()
            ));
        }

        output
    }
}

fn main() {
    let p1 = Product {
        name: "Laptop".to_string(),
        price: 60000.0,
    };

    let p2 = Product {
        name: "KeyBoard".to_string(),
        price: 1000.0,
    };

    let p3 = Product {
        name: "mouse".to_string(),
        price: 500.0,
    };

    let mut inventory = Inventory {
        items: HashMap::new(),
    };

    inventory.add_item("P01".to_string(), p1).unwrap();
    inventory.add_item("P02".to_string(), p2).unwrap();

    // duplicate check
    match inventory.add_item("P01".to_string(), p3.clone()) {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    };

    //get item where id not exits
    match inventory.get_item("P03".to_string()) {
        Ok(item) => {
            item.display();
        }
        Err(e) => println!("Error: {}", e),
    };

    // get item(success)
    match inventory.get_item("P01".to_string()) {
        Ok(item) => {
            println!("{:?}", item)
        }
        Err(e) => println!("Error: {}", e),
    };

    // add item with ivalid id

    match inventory.add_item("     ".to_string(), p3.clone()) {
        Ok(_) => {}
        Err(e) => println!("Erro: {}", e),
    };

    // get all the item

    let all_items = inventory.display_all();

    println!("{}", all_items);
}

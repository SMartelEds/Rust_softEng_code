// Function to process an order and update inventory
fn process_order(inventory: &mut Vec<(String, u32)>, product_name: &str, quantity: u32) {
    if let Some((_, stock)) = inventory.iter_mut().find(|(name, _)| name == product_name) {
        if *stock >= quantity {
            *stock -= quantity;
        } else {
            println!(
                "Order for {} {} cannot be fulfilled. Insufficient stock.",
                quantity, product_name
            );
        }
    } else {
        println!("Product {} not found in inventory.", product_name);
    }
}

// Function to generate a restock report
fn generate_restock_report(inventory: &[(String, u32)]) -> Vec<String> {
    inventory
        .iter()
        .filter(|(_, stock)| *stock < 3)
        .map(|(name, _)| name.clone())
        .collect()
}

pub fn exercise_4() {
    let mut inventory: Vec<(String, u32)> = vec![
        ("apple".to_string(), 10),
        ("banana".to_string(), 5),
        ("orange".to_string(), 8),
        ("pear".to_string(), 2),
    ];

    // Process the following orders
    process_order(&mut inventory, "apple", 4);
    process_order(&mut inventory, "banana", 6);
    process_order(&mut inventory, "orange", 3);
    process_order(&mut inventory, "pear", 2);

    // Print the updated inventory
    println!("Updated Inventory:");
    for (product, stock) in &inventory {
        println!("{}: {}", product, stock);
    }

    // Generate and print the restock report
    let restock_report = generate_restock_report(&inventory);
    println!("Restock Report: {:?}", restock_report);
}

#![allow(unused)]
pub mod utils;

use cargo_supply_order_tracker::{
    pre_add_orders, handle_menu_choice,
};
use std::io::{self, Write};
use crate::utils::get_user_choice;

fn main() {
    // pre-add orders
    if let Err(e) = pre_add_orders() {
        println!("Failed to pre-add orders: {}", e);
    } else {
        println!("Pre-added orders successfully");
    }

    loop {
        println!("\n===== Main Menu =====");
        println!("1. Add Order");
        println!("2. View Orders");
        println!("3. Remove Fullfilled Orders");
        println!("4. Edit Order");
        println!("5. Cancel Order");
        println!("6. Exit");

        // Prompt for input and ensure it's displayed
        print!("\nEnter your choice (1-6): ");
        io::stdout().flush().unwrap();

        match utils::get_user_choice(1, 6) {
            Ok(choice) => {
                if let Err(e) = handle_menu_choice(choice) {
                    println!("Error: {}", e);
                }
            }
            Err(e) => println!("Error: {}", e),
        }

        // // Process menu selection with validation
        // match get_user_choice(1, 6) {
        //     Ok(1) => {
        //         if let Err(e) = add_items() {
        //             println!("Failed to add item: {}", e);
        //         }
        //     }
        //     Ok(2) => {
        //         if let Err(e) = get_orders() {
        //             println!("Failed to get orders: {}", e);
        //         }
        //     }
        //     Ok(3) => {
        //         if let Err(e) = remove_fullfilled_orders() {
        //             println!("Failed to remove fullfilled orders: {}", e);
        //         }
        //     }
        //     Ok(6) => {
        //         println!("Thank you for using our system!");
        //         break;
        //     }
        //     Ok(_) => unreachable!(), //
        //     Err(e) => println!("Error: {}", e),
        // }
    }
}


pub mod utils;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    PendingPayment,
    Processing,
    Fullfilled,
    Cancelled,
    Refunded,
}

#[derive(Debug)]
pub enum OrderType {
    Regular,
    Express,
}

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub name: String,
    pub amount: f64,
    pub quantity: u32,
    pub supplier: String,
}

// pub struct OrderTracker {
//   pub orders: Vec<Order>,
//   pub next_id: u32,
// }

// // Global static OrderTracker
// static ORDER_TRACKER: Lazy<Arc<Mutex<OrderTracker>>> = Lazy::new(|| {
//     Arc::new(Mutex::new(OrderTracker::new()))
// });

// impl OrderTracker {
//   pub fn new() -> Self {
//     Self { orders: Vec::new(), next_id: 1 }
//   }

//   pub fn add_order(&mut self, order_type: OrderType, name: String, amount: f64, quantity: u32, supplier: String) {
//     let order = Order {
//       id: self.next_id,
//       order_type: order_type,
//       status: OrderStatus::PendingPayment,
//       name: name,
//       amount: amount,
//       quantity: quantity,
//       supplier: supplier,
//     };
//     self.orders.push(order);
//     self.next_id += 1;
//   }

//   pub fn get_orders(&self) -> &Vec<Order> {
//     &self.orders
//   }
// }
pub struct OrderTracker {
    pub orders: HashMap<u32, Order>,
    pub next_id: u32,
}

// Global static OrderTracker
static ORDER_TRACKER: Lazy<Arc<Mutex<OrderTracker>>> =
    Lazy::new(|| Arc::new(Mutex::new(OrderTracker::new())));

impl OrderTracker {
    pub fn new() -> OrderTracker {
        OrderTracker {
            orders: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_order(
        &mut self,
        order_type: OrderType,
        name: String,
        amount: f64,
        quantity: u32,
        supplier: String,
        status: OrderStatus,
    ) {
        let order = Order {
            id: self.next_id,
            order_type: order_type,
            status: status,
            name: name,
            amount: amount,
            quantity: quantity,
            supplier: supplier,
        };
        self.orders.insert(order.id, order);
        self.next_id += 1;
    }

    pub fn get_orders(&self) -> &HashMap<u32, Order> {
        &self.orders
    }

    pub fn get_order(&self, id: u32) -> Option<&Order> {
        self.orders.get(&id)
    }

    pub fn cancel_order(&mut self, order: &mut Order) {
        order.status = OrderStatus::Cancelled;
    }
}

pub fn print_order(order: Order) {
    println!("{:?} is {:?}", order.id, order.name);
}

// pre add three orders
pub fn pre_add_orders() -> Result<(), String> {
    let mut order_tracker = ORDER_TRACKER.lock().unwrap();
    order_tracker.add_order(
        OrderType::Regular,
        String::from("Item 1"),
        100.0,
        1,
        String::from("Supplier 1"),
        OrderStatus::PendingPayment,
    );
    order_tracker.add_order(
        OrderType::Regular,
        String::from("Item 2"),
        200.0,
        2,
        String::from("Supplier 2"),
        OrderStatus::PendingPayment,
    );
    order_tracker.add_order(
        OrderType::Regular,
        String::from("Item 3"),
        300.0,
        3,
        String::from("Supplier 3"),
        OrderStatus::Fullfilled,
    );
    order_tracker.add_order(
        OrderType::Regular,
        String::from("Item 4"),
        400.0,
        4,
        String::from("Supplier 4"),
        OrderStatus::Cancelled,
    );
    Ok(())
}

pub fn add_items() -> Result<(), String> {
    let mut order_tracker = ORDER_TRACKER.lock().unwrap();
    order_tracker.add_order(
        OrderType::Regular,
        utils::get_user_input("Enter the name of the item: "),
        utils::get_user_input_amount("Enter the amount of the item: "),
        utils::get_user_input_number("Enter the quantity of the item: "),
        utils::get_user_input("Enter the supplier of the item: "),
        OrderStatus::PendingPayment,
    );
    println!("Item added successfully");
    println!("All orders: {:?}", order_tracker.get_orders());
    Ok(())
}

pub fn get_orders() -> Result<(), String> {
    let order_tracker = ORDER_TRACKER.lock().unwrap();
    println!("All orders: {:?}", order_tracker.get_orders());
    println!(
        "All orders: {:?}",
        order_tracker.get_orders().values().len()
    );
    Ok(())
}

// remove_fullfilled_orders
pub fn remove_fullfilled_orders() -> Result<(), String> {
    let mut order_tracker = ORDER_TRACKER.lock().unwrap();
    order_tracker
        .orders
        .retain(|_, order| order.status != OrderStatus::Fullfilled);
    println!("Fullfilled orders removed successfully");
    println!(
        "All orders count: {:?}",
        order_tracker.get_orders().values().len()
    );
    Ok(())
}

pub fn cancel_order() -> Result<(), String> {
    let mut order_tracker = ORDER_TRACKER.lock().unwrap();
    let id = utils::get_user_input_number("Enter the id of the order to cancel: ");

    // get the order from the order tracker and check if it is not cancelled
    if let Some(order) = order_tracker.orders.get_mut(&id) {
      if order.status != OrderStatus::Cancelled {
        order.status = OrderStatus::Cancelled;
        println!("Order cancelled successfully");
        println!("All orders count after cancellation: {:?}", order_tracker.get_orders().values().len());
      } else {
        println!("Order already cancelled");
      }
    } else {
      println!("Order not found");
    }

    Ok(())
}


pub fn handle_menu_choice(choice: u32) -> Result<(), String> {
    match choice {
        1 => add_items(),
        2 => get_orders(),
        3 => remove_fullfilled_orders(),
        4 => println!("Edit order feature is not implemented yet"),
        5 => cancel_order(),
        6 => {
            println!("Thank you for using our system!");
            std::process::exit(0); // Or return Ok(()) and break in main
        }
        _ => unreachable!(),
    }
}
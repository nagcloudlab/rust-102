use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Clone)]
struct Customer {
    id: u32,
    name: String,
    email: String,
    phone: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Account {
    id: u32,
    customer_id: u32,
    account_type: String,
    balance: f64,
}

// --- Application State ---
struct AppState {
    customers: Mutex<HashMap<u32, Customer>>,
    accounts: Mutex<HashMap<u32, Account>>,
}

// --- Handlers ---

// Fetch customer details by ID
async fn get_customer(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner();
    let customers = data.customers.lock().unwrap();

    if let Some(customer) = customers.get(&id) {
        HttpResponse::Ok().json(customer)
    } else {
        HttpResponse::NotFound().body(format!("Customer with ID {} not found", id))
    }
}

// Fetch account details by ID
async fn get_account(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner();
    let accounts = data.accounts.lock().unwrap();

    if let Some(account) = accounts.get(&id) {
        HttpResponse::Ok().json(account)
    } else {
        HttpResponse::NotFound().body(format!("Account with ID {} not found", id))
    }
}

// Create a new customer
async fn create_customer(
    customer: web::Json<Customer>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut customers = data.customers.lock().unwrap();
    let customer = customer.into_inner();

    customers.insert(customer.id, customer.clone());
    HttpResponse::Created().json(customer)
}

// Create a new account
async fn create_account(account: web::Json<Account>, data: web::Data<AppState>) -> impl Responder {
    let mut accounts = data.accounts.lock().unwrap();
    let account = account.into_inner();

    accounts.insert(account.id, account.clone());
    HttpResponse::Created().json(account)
}

// Fetch all data (example of over-fetching)
async fn get_all_data(data: web::Data<AppState>) -> impl Responder {
    let customers = data.customers.lock().unwrap();
    let accounts = data.accounts.lock().unwrap();

    let response = serde_json::json!({
        "customers": customers.values().collect::<Vec<_>>(),
        "accounts": accounts.values().collect::<Vec<_>>()
    });

    HttpResponse::Ok().json(response)
}

// --- Main Function ---

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let customers = Mutex::new(HashMap::new());
    let accounts = Mutex::new(HashMap::new());

    let data = web::Data::new(AppState {
        customers,
        accounts,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/customers/{id}", web::get().to(get_customer)) // Get customer by ID
            .route("/accounts/{id}", web::get().to(get_account)) // Get account by ID
            .route("/customers", web::post().to(create_customer)) // Create customer
            .route("/accounts", web::post().to(create_account)) // Create account
            .route("/all-data", web::get().to(get_all_data)) // Fetch all data (over-fetching)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

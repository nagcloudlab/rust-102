use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_graphql::{InputObject, Object, Schema, SimpleObject};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

// Define the data types
#[derive(SimpleObject)]
struct Customer {
    id: u32,
    name: String,
    email: String,
    phone: String,
}

#[derive(SimpleObject)]
struct Account {
    id: u32,
    customer_id: u32,
    account_type: String,
    balance: f64,
}

#[derive(InputObject)]
struct NewCustomerInput {
    id: u32,
    name: String,
    email: String,
    phone: String,
}

#[derive(InputObject)]
struct NewAccountInput {
    id: u32,
    customer_id: u32,
    account_type: String,
    balance: f64,
}

// Define the resolvers
struct Query;

#[Object]
impl Query {
    async fn get_customer(&self, id: u32) -> Option<Customer> {
        Some(Customer {
            id,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "123-456-7890".to_string(),
        })
    }

    async fn get_account(&self, id: u32) -> Option<Account> {
        Some(Account {
            id,
            customer_id: 1,
            account_type: "Savings".to_string(),
            balance: 5000.0,
        })
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn add_customer(&self, input: NewCustomerInput) -> Customer {
        Customer {
            id: input.id,
            name: input.name,
            email: input.email,
            phone: input.phone,
        }
    }

    async fn add_account(&self, input: NewAccountInput) -> Account {
        Account {
            id: input.id,
            customer_id: input.customer_id,
            account_type: input.account_type,
            balance: input.balance,
        }
    }
}

// Actix Web integration
async fn graphql_handler(
    schema: web::Data<Schema<Query, Mutation, async_graphql::EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_schema(
    schema: web::Data<Schema<Query, Mutation, async_graphql::EmptySubscription>>,
) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body(schema.sdl())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, Mutation, async_graphql::EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
            .route("/graphql-schema", web::get().to(graphql_schema)) // Endpoint for SDL
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

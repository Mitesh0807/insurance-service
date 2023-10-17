use axum::{extract, http};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Customer {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    aadhar_number: i64,
    date_of_birth: String,
    gender: String,
    address: String,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Customer {
    fn new(
        first_name: String,
        last_name: String,
        aadhar_number: i64,
        date_of_birth: String,
        gender: String,
        address: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            first_name,
            last_name,
            aadhar_number,
            date_of_birth,
            gender,
            address,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct CreateCustomer {
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: i64,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
}

pub async fn health_check() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn get_all_customers(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Customer>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Customer>("SELECT * FROM customers")
        .fetch_all(&pool)
        .await;
    match res {
        Ok(customers) => Ok(axum::Json(customers)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
pub async fn create_customer(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(customer): axum::Json<CreateCustomer>,
) -> Result<(http::StatusCode, axum::Json<Customer>), http::StatusCode> {
    let new_customer = Customer::new(
        customer.first_name,
        customer.last_name,
        customer.aadhar_number,
        customer.date_of_birth,
        customer.gender,
        customer.address,
    );
    let res = sqlx::query(
        r#"
        INSERT INTO customers (
            id,    
            first_name,
            last_name,
            aadhar_number,
            date_of_birth,
            gender,
            address,
            is_active,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(&new_customer.id)
    .bind(&new_customer.first_name)
    .bind(&new_customer.last_name)
    .bind(&new_customer.aadhar_number)
    .bind(&new_customer.date_of_birth)
    .bind(&new_customer.gender)
    .bind(&new_customer.address)
    .bind(&new_customer.is_active)
    .bind(&new_customer.created_at)
    .bind(&new_customer.updated_at)
    .execute(&pool)
    .await;

    println!("{:?}", res);

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(new_customer))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// async fn get_customer(
//     extract::State(pool): extract::State<PgPool>,
//     extract::Path(id): extract::Path<i32>,
// ) -> (axum::Json<Customer>, http::StatusCode) {
//     let res = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = $1")
//         .bind(id)
//         .fetch_one(&pool);
//     match res {
//         Ok(customer) => (axum::Json(customer), http::StatusCode::OK),
//         Err(_) => (axum::Json(Customer::default()), http::StatusCode::NOT_FOUND),
//     }
// }
//
// async fn update_customer(
//     extract::State(pool): extract::State<PgPool>,
//     extract::Path(id): extract::Path<i32>,
//     axum::Json(customer): Json<CreateCustomer>,
// ) -> (axum::Json<Customer>, http::StatusCode) {
//     let now = chrono::Utc::now();
//     let res = sqlx::query(
//         r#"
//         UPDATE customers
//         SET first_name = $1,
//             last_name = $2,
//             aadhar_number = $3,
//             date_of_birth = $4,
//             gender = $5,
//             address = $6,
//             updated_at = $7
//         WHERE id = $8
//         "#,
//     )
//     .bind(customer.first_name)
//     .bind(customer.last_name)
//     .bind(customer.aadhar_number)
//     .bind(customer.date_of_birth)
//     .bind(customer.gender)
//     .bind(customer.address)
//     .bind(now)
//     .bind(id)
//     .execute(&pool)
//     .await
//     .map(|res| match res.rows_affected() {
//         0 => (axum::Json(Customer::default()), http::StatusCode::NOT_FOUND),
//         _ => (axum::Json(Customer::default()), http::StatusCode::OK),
//     });
//     res.unwrap()
// }
//
// async fn delete_customer(
//     extract::State(pool): extract::State<PgPool>,
//     extract::Path(id): extract::Path<i32>,
// ) -> http::StatusCode {
//     let res = sqlx::query(
//         r#"
//         DELETE FROM customers
//         WHERE id = $1
//         "#,
//     )
//     .bind(id)
//     .execute(&pool)
//     .await;
//     match res {
//         Ok(_) => http::StatusCode::NO_CONTENT,
//         Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }
//

// use axum::{extract, http};
// use serde::{Deserialize, Serialize};
// use sqlx::{FromRow, PgPool};
//
// #[derive(Serialize, FromRow)]
// pub struct Quote {
//     id: uuid::Uuid,
//     book: String,
//     quote: String,
//     inserted_at: chrono::DateTime<chrono::Utc>,
//     updated_at: chrono::DateTime<chrono::Utc>,
// }
//
// impl Quote {
//     fn new(book: String, quote: String) -> Self {
//         let now = chrono::Utc::now();
//         Self {
//             id: uid::Uuid::new_v4(),
//             book,
//             quote,
//             inserted_at: now,
//             updated_at: now,
//         }
//     }
// }
//
// #[derive(Debug, Deserialize)]
// pub struct CreateQuote {
//     book: String,
//     quote: String,
// }
//
// pub async fn health() -> http::StatusCode {
//     http::StatusCode::OK
// }
//
// pub async fn create_quote(
//     extract::State(pool): extract::State<PgPool>,
//     axum::axum::Json(payload): axum::Json<CreateQuote>,
// ) -> Result<(http::StatusCode, axum::axum::Json<Quote>), http::StatusCode> {
//     let quote = Quote::new(payload.book, payload.quote);
//
//     let res = sqlx::query(
//         r#"
//         INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
//         VALUES ($1, $2, $3, $4, $5)
//         "#,
//     )
//     .bind(&quote.id)
//     .bind(&quote.book)
//     .bind(&quote.quote)
//     .bind(&quote.inserted_at)
//     .bind(&quote.updated_at)
//     .execute(&pool)
//     .await;
//
//     match res {
//         Ok(_) => Ok((http::StatusCode::CREATED, axum::axum::Json(quote))),
//         Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }
//
// pub async fn read_quotes(
//     extract::State(pool): extract::State<PgPool>,
// ) -> Result<axum::axum::Json<Vec<Quote>>, http::StatusCode> {
//     let res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
//         .fetch_all(&pool)
//         .await;
//
//     match res {
//         Ok(quotes) => Ok(axum::axum::Json(quotes)),
//         Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }
//
// pub async fn update_quote(
//     extract::State(pool): extract::State<PgPool>,
//     extract::Path(id): extract::Path<uuid::Uuid>,
//     axum::axum::Json(payload): axum::Json<CreateQuote>,
// ) -> http::StatusCode {
//     let now = chrono::Utc::now();
//
//     let res = sqlx::query(
//         r#"
//         UPDATE quotes
//         SET book = $1, quote = $2, updated_at = $3
//         WHERE id = $4
//         "#,
//     )
//     .bind(&payload.book)
//     .bind(&payload.quote)
//     .bind(now)
//     .bind(id)
//     .execute(&pool)
//     .await
//     .map(|res| match res.rows_affected() {
//         0 => http::StatusCode::NOT_FOUND,
//         _ => http::StatusCode::OK,
//     });
//
//     match res {
//         Ok(status) => status,
//         Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }
//
// pub async fn delete_quote(
//     extract::State(pool): extract::State<PgPool>,
//     extract::Path(id): extract::Path<uuid::Uuid>,
// ) -> http::StatusCode {
//     let res = sqlx::query(
//         r#"
//         DELETE FROM quotes
//         WHERE id = $1
//         "#,
//     )
//     .bind(id)
//     .execute(&pool)
//     .await
//     .map(|res| match res.rows_affected() {
//         0 => http::StatusCode::NOT_FOUND,
//         _ => http::StatusCode::OK,
//     });
//
//     match res {
//         Ok(status) => status,
//         Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
//     }
// }

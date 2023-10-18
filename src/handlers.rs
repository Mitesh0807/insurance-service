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
            is_active: false,
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Dependents {
    id: uuid::Uuid,
    customer_id: uuid::Uuid,
    first_name: String,
    last_name: String,
    aadhar_number: i64,
    date_of_birth: String,
    gender: String,
    address: String,
    relation: String,
    relationship: String,
    is_active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}
impl Dependents {
    fn new(
        first_name: String,
        last_name: String,
        aadhar_number: i64,
        customer_id: uuid::Uuid,
        date_of_birth: String,
        gender: String,
        address: String,
        relation: String,
        relationship: String,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            customer_id,
            first_name,
            last_name,
            aadhar_number,
            date_of_birth,
            gender,
            address,
            relation,
            relationship,
            is_active: false,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateDependents {
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: i64,
    pub customer_id: uuid::Uuid,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
    pub relation: String,
    pub relationship: String,
}
#[derive(Debug, Deserialize)]
pub struct CreateCustomerWithDependents {
    pub first_name: String,
    pub last_name: String,
    pub aadhar_number: i64,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
    pub dependents: Vec<CreateDependents>,
}

pub async fn create_customer_with_dependents(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateCustomerWithDependents>,
) -> http::StatusCode {
    println!("payload {:?}", payload);
    let customer = Customer::new(
        payload.first_name,
        payload.last_name,
        payload.aadhar_number,
        payload.date_of_birth,
        payload.gender,
        payload.address,
    );
    println!("customer {:?}", customer);
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
            updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
    )
    .bind(&customer.id)
    .bind(&customer.first_name)
    .bind(&customer.last_name)
    .bind(&customer.aadhar_number)
    .bind(&customer.date_of_birth)
    .bind(&customer.gender)
    .bind(&customer.address)
    .bind(&customer.is_active)
    .bind(&customer.created_at)
    .bind(&customer.updated_at)
    .execute(&pool)
    .await;
    println!("res {:?}", res);
    if res.is_err() {
        return http::StatusCode::INTERNAL_SERVER_ERROR;
    }
    if !payload.dependents.is_empty() {
        for dependents in payload.dependents {
            let new_dependents = Dependents::new(
                dependents.first_name,
                dependents.last_name,
                dependents.aadhar_number,
                customer.id,
                dependents.date_of_birth,
                dependents.gender,
                dependents.address,
                dependents.relation,
                dependents.relationship,
            );
            let res = sqlx::query(
                r#"
                INSERT INTO dependents (
                    id,
                    customer_id,
                    first_name,
                    last_name,
                    aadhar_number,
                    date_of_birth,
                    gender,
                    address,
                    relation,
                    relationship,
                    is_active,
                    created_at,
                    updated_at
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                "#,
            )
            .bind(&new_dependents.id)
            .bind(&new_dependents.customer_id)
            .bind(&new_dependents.first_name)
            .bind(&new_dependents.last_name)
            .bind(&new_dependents.aadhar_number)
            .bind(&new_dependents.date_of_birth)
            .bind(&new_dependents.gender)
            .bind(&new_dependents.address)
            .bind(&new_dependents.relation)
            .bind(&new_dependents.relationship)
            .bind(&new_dependents.is_active)
            .bind(&new_dependents.created_at)
            .bind(&new_dependents.updated_at)
            .execute(&pool)
            .await;
            println!("res {:?}", res);
            if res.is_err() {
                return http::StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }
    match res {
        Ok(_) => http::StatusCode::CREATED,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
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
    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(new_customer))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
pub async fn create_dependents(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(dependents): axum::Json<CreateDependents>,
) -> Result<(http::StatusCode, axum::Json<Dependents>), http::StatusCode> {
    let new_dependents = Dependents::new(
        dependents.first_name,
        dependents.last_name,
        dependents.aadhar_number,
        dependents.customer_id,
        dependents.date_of_birth,
        dependents.gender,
        dependents.address,
        dependents.relation,
        dependents.relationship,
    );
    let res = sqlx::query(
        r#"
        INSERT INTO dependents (
            id,
            customer_id,
            first_name,
            last_name,
            aadhar_number,
            date_of_birth,
            gender,
            address,
            relation,
            relationship,
            is_active,
            created_at,
            updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#,
    )
    .bind(&new_dependents.id)
    .bind(&new_dependents.customer_id)
    .bind(&new_dependents.first_name)
    .bind(&new_dependents.last_name)
    .bind(&new_dependents.aadhar_number)
    .bind(&new_dependents.date_of_birth)
    .bind(&new_dependents.gender)
    .bind(&new_dependents.address)
    .bind(&new_dependents.relation)
    .bind(&new_dependents.relationship)
    .bind(&new_dependents.is_active)
    .bind(&new_dependents.created_at)
    .bind(&new_dependents.updated_at)
    .execute(&pool)
    .await;
    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(new_dependents))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
// pub async fn get_customer(
//     extract::State(pool): extract::State<PgPool>,
//     extract::Path(id): extract::Path<i32>,
// ) -> Result<(axum::Json<Customer>, http::StatusCode), http::StatusCode> {
//     let res = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = $1")
//         .bind(id)
//         .fetch_one(&pool)
//         .await;
//     match res {
//         Ok(customer) => Ok((axum::Json(customer), http::StatusCode::OK)),
//         Err(_) => Err(http::StatusCode::NOT_FOUND),
//     }
// }
pub async fn delete_customer(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<String>,
) -> Result<http::StatusCode, http::StatusCode> {
    let uuid = uuid::Uuid::parse_str(&id);
    let res = sqlx::query(
        r#"
        DELETE FROM customers
        WHERE id = $1
        "#,
    )
    .bind(&uuid.unwrap())
    .execute(&pool)
    .await;
    match res {
        Ok(_) => Ok(http::StatusCode::OK),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
pub async fn update_customer(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<String>,
    axum::Json(customer): axum::Json<CreateCustomer>,
) -> http::StatusCode {
    let now = chrono::Utc::now();
    let uuid = uuid::Uuid::parse_str(&id);
    let res = sqlx::query(
        r#"
        UPDATE customers
        SET first_name = $1,
            last_name = $2,
            aadhar_number = $3,
            date_of_birth = $4,
            gender = $5,
            address = $6,
            updated_at = $7
        WHERE id = $8
        "#,
    )
    .bind(customer.first_name)
    .bind(customer.last_name)
    .bind(customer.aadhar_number)
    .bind(customer.date_of_birth)
    .bind(customer.gender)
    .bind(customer.address)
    .bind(now)
    .bind(uuid.unwrap())
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::user_model::{NewUser, UpdateUser, User, LoginRequest};

pub async fn register_user(
    db: web::Data<PgPool>,
    form: web::Json<NewUser>,
) -> impl Responder {
    let hashed_pwd = hash(&form.password, DEFAULT_COST).unwrap();
    let id = Uuid::new_v4();

    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (id, name, phone, password) VALUES ($1, $2, $3, $4) RETURNING id, name, phone, password",
        id,
        form.name,
        form.phone,
        hashed_pwd
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn login_user(
    db: web::Data<PgPool>,
    form: web::Json<LoginRequest>,
) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE phone = $1",
        form.phone
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(user) => {
            if verify(&form.password, &user.password).unwrap() {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::Unauthorized().body("Invalid password")
            }
        }
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

pub async fn get_users(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn update_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    form: web::Json<UpdateUser>,
) -> impl Responder {
    let user_id = user_id.into_inner();

    // Fetch existing user from DB
    let existing_user = sqlx::query!(
        "SELECT * FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool.get_ref())
    .await;

    let user = match existing_user {
        Ok(u) => u,
        Err(_) => return HttpResponse::NotFound().body("User not found"),
    };

    // Update fields or fallback to existing
    let name = form.name.clone().unwrap_or(user.name);
    let phone = form.phone.clone().unwrap_or(user.phone);

    // Hash new password if provided, else keep old hashed password
    let password = match &form.password {
        Some(pwd) => {
            match hash(pwd, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
            }
        }
        None => user.password,
    };

    // Perform update query
    let result = sqlx::query!(
        "UPDATE users SET name = $1, phone = $2, password = $3 WHERE id = $4",
        name,
        phone,
        password,
        user_id
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User updated successfully"),
        Err(e) => {
            eprintln!("Database error during update: {}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}
pub async fn delete_user(
    db: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        *user_id
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

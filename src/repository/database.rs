use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::users::User;
use crate::models::schema::users::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_users(&self) -> Result<User, Error> {
        let user: User = users
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading user");
        Ok(user)
    }

    pub fn create_user(&self, user: User) -> Result<User, Error> {
        let user: User = User {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..user
        };

        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new user");

        Ok(user)
    }

    pub fn get_user_by_id(&self, user_id: &str) -> Option<User> {
        let user: User = users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading user");
        Some(user)
    }

    pub fn update_user(&self, user_id: &str, mut user: User) -> Option<User> {
        user.updated_at = Some(Utc::now().naive_utc());
        let user: User = diesel::update(users.find(user_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error updating user");
        Some(user)
    }

    pub fn delete_user(&self, user_id: &str) -> Option<usize> {
        let count: usize = diesel::delete(users.find(user_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting todo");
        Some(count)
    }
}

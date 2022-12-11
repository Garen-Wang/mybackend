use crate::{error::AppError, schema::users};
use crate::token;
use chrono::Utc;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub is_admin: bool,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub fn generate_token(&self) -> Result<String, AppError> {
        let now = Utc::now().timestamp();
        let token = token::generate_token(self.id, now)?;
        Ok(token)
    }
}

// static methods
impl User {
    pub fn register(
        conn: &mut PgConnection,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<(User, String), AppError> {
        let encrypted_password = bcrypt::hash(naive_password, bcrypt::DEFAULT_COST)?;

        let insert_user = InsertUser {
            email: email,
            username: username,
            password: &encrypted_password,
        };

        let user: User = diesel::insert_into(users::table)
            .values(insert_user)
            .get_result(conn)?;

        let token = user.generate_token()?;
        Ok((user, token))
    }

    pub fn login(
        conn: &mut PgConnection,
        email: &str,
        naive_password: &str,
    ) -> Result<(User, String), AppError> {
        let user: User = users::table
            .filter(users::email.eq(email))
            .limit(1)
            .first(conn)?;
        let a = bcrypt::verify(naive_password, &user.password)?;
        if a {
            let token = user.generate_token()?;
            Ok((user, token))
        } else {
            Err(AppError::InternalServerError)
        }
    }

    pub fn update(
        conn: &mut PgConnection,
        user_id: i32,
        changeset: UpdateUser,
    ) -> Result<User, AppError> {
        let target = users::table.filter(users::id.eq(user_id));
        let user = diesel::update(target).set(changeset).get_result(conn)?;
        Ok(user)
    }

    pub fn find(conn: &mut PgConnection, user_id: i32) -> Result<User, AppError> {
        let user = users::table.find(user_id).first(conn)?;
        Ok(user)
    }

    pub fn find_by_username(conn: &mut PgConnection, username: &str) -> Result<User, AppError> {
        let user: User = users::table
            .filter(users::username.eq(username))
            .limit(1)
            .first(conn)?;
        Ok(user)
    }
}

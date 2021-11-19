use anyhow::{Context, Error, Result};
use argon2::{self, Config};
use chrono::{DateTime, NaiveDateTime, Utc}; // https://rust-lang-nursery.github.io/rust-cookbook/datetime/duration.html
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{FromRow, Row};
use std::env;

/// The Profile model
/// Database table:
/// CREATE TABLE IF NOT EXISTS profiles (
///    profile_id serial PRIMARY KEY,
///    nickname VARCHAR (50),
///    avatar VARCHAR (255),
///    bibo VARCHAR(24),
///    reputation INT DEFAULT 0 NOT NULL,
///    account_id INT UNIQUE NOT NULL,
///    FOREIGN KEY(account_id) REFERENCES accounts(account_id) ON DELETE CASCADE
/// );
#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Profile {
    pub profile_id: i32,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub bibo: Option<String>,
    pub reputation: i32,
    pub account_id: i32,
}

impl Profile {
    pub async fn all(db: &PgPool) -> Result<Vec<Self>> {
        let profiles =
            sqlx::query_as::<_, Profile>(r#"SELECT * FROM profiles ORDER BY profile_id;"#)
                .fetch_all(db)
                .await?;
        Ok(profiles)
    }

    pub async fn get(db: &PgPool, profile_id: i32) -> Result<Option<Self>> {
        let profile_option =
            sqlx::query_as::<_, Profile>(r#"SELECT * FROM profiles WHERE profile_id=$1 LIMIT 1;"#)
                .bind(profile_id)
                .fetch_optional(db)
                .await?;

        Ok(profile_option)
    }
    pub async fn get_by_account(db: &PgPool, account_id: i32) -> Result<Option<Self>> {
        let profile_option =
            sqlx::query_as::<_, Profile>(r#"SELECT * FROM profiles WHERE account_id=$1 LIMIT 1;"#)
                .bind(account_id)
                .fetch_optional(db)
                .await?;

        Ok(profile_option)
    }

    pub async fn nickname_exist(db: &PgPool, nickname: &String) -> Result<bool> {
        let is_exist: bool =
            sqlx::query(r#"SELECT exists (SELECT 1 FROM profiles WHERE nickname = $1 LIMIT 1);"#)
                .bind(nickname)
                .map(|r: PgRow| r.try_get("exists").unwrap_or(false))
                .fetch_one(db)
                .await?;
        Ok(is_exist)
    }

    pub async fn filter_by_nickname(db: &PgPool, nickname: &String) -> Result<Option<Self>> {
        let profile_option =
            sqlx::query_as::<_, Profile>(r#"SELECT * FROM profiles WHERE nickname=$1;"#)
                .bind(nickname)
                .fetch_optional(db)
                .await?;
        Ok(profile_option)
    }

    // pub async fn create(db: &PgPool, nickname: &String, avatar: &String) -> Result<Self> {
    //     let profile = sqlx::query_as::<_, Profile>(
    //         r#"INSERT INTO profiles(nickname, avatar) VALUES ($1, $2) returning *;"#,
    //     )
    //     .bind(nickname)
    //     .bind(avatar)
    //     .fetch_one(db)
    //     .await?;
    //     Ok(profile)
    // }

    pub async fn update_nickname(&self, db: &PgPool, newname: String) -> Result<()> {
        todo!()
    }

    pub async fn update_avatar(&self, db: &PgPool, newavatar: String) -> Result<()> {
        todo!()
    }

    pub async fn update_reputation(&self, db: &PgPool, reputation: i32) -> Result<()> {
        todo!()
    }
}

/// The auth model
/// Database table:
/// CREATE TABLE IF NOT EXISTS accounts (
///     account_id serial PRIMARY KEY,
///     email VARCHAR(255) UNIQUE,
///     phone VARCHAR(13) UNIQUE,
///     password_hash VARCHAR(255),
///     last_login TIMESTAMP DEFAULT current_timestamp,
///     created TIMESTAMP DEFAULT current_timestamp,
///     is_active BOOL DEFAULT true
/// );
#[derive(Clone, Debug, Deserialize, Serialize, FromRow)]
pub struct Account {
    pub account_id: i32,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: Option<String>,
    pub last_login: NaiveDateTime,
    pub created: NaiveDateTime,
    pub is_active: bool,
}

impl Account {
    pub async fn email_exist(db: &PgPool, email: &String) -> Result<bool> {
        let is_exist: bool =
            sqlx::query(r#"SELECT exists (SELECT 1 FROM accounts WHERE email = $1 LIMIT 1);"#)
                .bind(email)
                .map(|r: PgRow| r.try_get("exists").unwrap_or(false))
                .fetch_one(db)
                .await?;
        Ok(is_exist)
    }
    pub async fn register(db: &PgPool, email: &String, password: &String) -> Result<Profile> {
        let mut transaction = db.begin().await?;
        let account = sqlx::query_as::<_, Account>(
            r#"INSERT INTO accounts(email, password_hash) VALUES ($1, $2) returning *;"#,
        )
        .bind(email)
        .bind(Self::hash_password(password))
        .fetch_one(&mut transaction)
        .await?;

        let nickname = format!("学霸 {}", account.account_id + 10000);
        let profile = sqlx::query_as::<_, Profile>(
            r#"INSERT INTO profiles(nickname, account_id) VALUES ($1, $2) returning *;"#,
        )
        .bind(nickname)
        .bind(account.account_id)
        .fetch_one(&mut transaction)
        .await?;
        transaction.commit().await?;

        Ok(profile)
    }

    pub async fn authenticate(db: &PgPool, email: &String, password: &String) -> Option<Profile> {
        let result: Option<(i32, String)> =
            sqlx::query(r#"SELECT  account_id, password_hash FROM accounts WHERE email = $1"#)
                .bind(email)
                .map(|r: PgRow| (r.get("account_id"), r.get("password_hash")))
                .fetch_optional(db)
                .await
                .expect("faild to query database @ user::models::authenticate()");

        match result {
            Some((account_id, hash)) => {
                if Self::check_password(password, &hash) {
                    Profile::get_by_account(db, account_id).await.unwrap()
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub async fn login() {}

    pub async fn logout() {}

    pub async fn update_password() {}

    // More details: https://cloudmaker.dev/authenticate-api-users/
    fn hash_password(password: &String) -> String {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
    }

    fn check_password(password: &String, hash: &String) -> bool {
        argon2::verify_encoded(hash, password.as_bytes()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_profile_table(db: &PgPool) {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS profiles (
            profile_id serial PRIMARY KEY,
            nickname VARCHAR (50),
            avatar VARCHAR (255),
            bibo VARCHAR(24),
            reputation INT DEFAULT 0 NOT NULL,
            account_id INT UNIQUE NOT NULL,
            FOREIGN KEY(account_id) REFERENCES accounts(account_id) ON DELETE CASCADE
        );"#;
        sqlx::query(sql)
            .execute(db)
            .await
            .expect("faild to create profile table.");
    }

    async fn create_account_table(db: &PgPool) {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS accounts (
            account_id serial PRIMARY KEY,
            email VARCHAR(255) UNIQUE,
            phone VARCHAR(13) UNIQUE,
            password_hash VARCHAR(255),
            last_login TIMESTAMP DEFAULT current_timestamp,
            created TIMESTAMP DEFAULT current_timestamp,
            is_active BOOL DEFAULT true
        );"#;
        sqlx::query(sql)
            .execute(db)
            .await
            .expect("faild to create account table.");
    }

    async fn drop_profile_table(db: &PgPool) {
        sqlx::query(r#"DROP TABLE IF EXISTS profiles;"#)
            .execute(db)
            .await
            .expect("faild to drop profile table");
    }
    async fn drop_account_table(db: &PgPool) {
        sqlx::query(r#"DROP TABLE IF EXISTS accounts;"#)
            .execute(db)
            .await
            .expect("faild to drop account table");
    }

    #[async_std::test]
    async fn test_profile_crud() {
        dotenv::dotenv().ok();
        let db_url = env::var("TEST_DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let db = PgPool::connect(&db_url).await.expect("faild to connect db");
        // create table
        create_account_table(&db).await;
        create_profile_table(&db).await;

        let email1 = String::from("rust@rust.com");
        let password1 = String::from("123456asdfgf");
        let password2 = String::from("13456asdfgf");

        let profile = Account::register(&db, &email1, &password1).await.unwrap();
        println!("register: {:?}", profile);

        let profile = Account::authenticate(&db, &email1, &password1).await;
        println!("auth: {:?}", profile);

        let profile = Account::authenticate(&db, &email1, &password2).await;
        println!("auth failed: {:?}", profile);

        let profiles = Profile::all(&db).await.unwrap();
        println!("all: {:?}", profiles);

        let profile = Profile::get(&db, 1).await.unwrap();
        println!("get: {:?}", profile);
        let profile = Profile::get(&db, 2).await.unwrap();
        println!("get: {:?}", profile);

        let profile = Profile::get_by_account(&db, 1).await.unwrap();
        println!("get_by_account: {:?}", profile);

        println!(
            "{:?}",
            Profile::nickname_exist(&db, &"Rut".to_owned())
                .await
                .expect("faild check nickname")
        );
        // drop profile table
        drop_profile_table(&db).await;
        drop_account_table(&db).await;
    }
}

// example url: https://gist.github.com/jeremychone/34d1e3daffc38eb602b1a9ab21298d10

pub use sqlx;

static DB: std::sync::OnceLock<sqlx::PgPool> = std::sync::OnceLock::new();

async fn create_pool() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specified");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("migrations failed");

    pool
}

pub async fn init_db() -> Result<(), sqlx::Pool<sqlx::Postgres>> {
    DB.set(create_pool().await)
}

pub fn db<'a>() -> &'a sqlx::PgPool {
    DB.get().expect("database uninitialized")
}

#[macro_export]
macro_rules! db_query {
    // No bind arguments
    ($func:ident, $sql:expr) => {{
        use common::db::sqlx;
        sqlx::query($sql).$func(common::db::db()).await
    }};
    // With bind arguments
    ($func:ident, $sql:expr, $($bind:expr),+ $(,)?) => {{
        use common::db::sqlx;
        sqlx::query($sql)$(.bind($bind))*.$func(common::db::db()).await
    }};
}

#[macro_export]
macro_rules! db_query_as {
    // No bind arguments
    ($ty:ty, $func:ident, $sql:expr) => {{
        use common::db::sqlx;
        sqlx::query_as::<_, $ty>($sql).$func(common::db::db()).await
    }};
    // With bind arguments
    ($ty:ty, $func:ident, $sql:expr, $($bind:expr),+ $(,)?) => {{
        use common::db::sqlx;
        sqlx::query_as::<_, $ty>($sql)$(.bind($bind))*.$func(common::db::db()).await
    }};
}

#[macro_export]
macro_rules! db_query_scalar {
    // No bind arguments
    ($ty:ty, $func:ident, $sql:expr) => {{
        use common::db::sqlx;
        sqlx::query_scalar::<_, $ty>($sql).$func(common::db::db()).await
    }};
    // With bind arguments
    ($ty:ty, $func:ident, $sql:expr, $($bind:expr),+ $(,)?) => {{
        use common::db::sqlx;
        sqlx::query_scalar::<_, $ty>($sql)$(.bind($bind))*.$func(common::db::db()).await
    }};
}

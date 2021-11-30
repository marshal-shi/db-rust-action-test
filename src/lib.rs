use anyhow;
use sqlx::postgres::PgPool;

struct Student;

impl Student {
    async fn create(pool: &PgPool, name: String) -> anyhow::Result<i32> {
        let res = sqlx::query!(
            "insert into student (name) values ($1) returning id",
            name
        )
        .fetch_one(pool)
        .await?;

        Ok(res.id)
    }

    async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<i32> {
        let res =
            sqlx::query!("delete from student where id = $1 returning id", id)
                .fetch_one(pool)
                .await?;

        Ok(res.id)
    }

    async fn get(pool: &PgPool, id: i32) -> anyhow::Result<String> {
        let res = sqlx::query!("select name from student where id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(res.name.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Student;
    use anyhow;
    use dotenv;
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::env;

    async fn setup() -> anyhow::Result<PgPool> {
        let _ = dotenv::dotenv();

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_TEST_URL")?)
            .await?;

        Ok(pool)
    }

    #[test]
    fn works() {
        assert_eq!(4, 2 + 2);
    }

    #[tokio::test]
    async fn create_works() -> anyhow::Result<()> {
        let pool = setup().await?;

        let res = Student::create(&pool, "1st".into()).await;
        assert!(res.is_ok());

        Student::delete(&pool, res.unwrap()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_works() -> anyhow::Result<()> {
        let pool = setup().await?;

        let id = Student::create(&pool, "2nd".into()).await?;
        let name = Student::get(&pool, id).await?;
        assert!(name == "2nd");

        Student::delete(&pool, id).await?;
        Ok(())
    }

    #[test]
    fn should_fail() {
        assert_eq!(3, 2 + 2);
    }

}

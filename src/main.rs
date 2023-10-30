use std::error::Error;

struct Game {
    title: String,
    designer: String,
    minplayers: i32,
    maxplayers: i32,
}

async fn create(game: &Game, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query =
        "INSERT INTO games (title, designer, minplayers, maxplayers) VALUES ($1, $2, $3, $4)";

    sqlx::query(query)
        .bind(&game.title)
        .bind(&game.designer)
        .bind(&game.minplayers)
        .bind(&game.maxplayers)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://dbuser:mysecretpassword@localhost:5432/boardgamedb";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let game = Game {
        title: "Dune: Imperium".into(),
        designer: "Paul Dennen".into(),
        minplayers: 1,
        maxplayers: 4,
    };

    create(&game, &pool).await?;

    Ok(())
}

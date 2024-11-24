use sea_orm::{Database, DbConn, EntityTrait, ActiveModelTrait, Set};
use std::env;

use crate::entity::users;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // データベース接続
  let database_url = env::var("DATABASE_URL")?;
  let db: DbConn = Database::connect(database_url).await?;

  // シードデータの挿入
  let user = users::ActiveModel {
    name: Set("John Doe".to_owned()),
    email: Set("john.doe@example.com".to_owned()),
    ..Default::default()
  };
  user.insert(&db).await?;

  Ok(())
}
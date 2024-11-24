use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
          .create_table(
              Table::create()
                .table(Users::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Users::Id)
                      .integer()
                      .not_null()
                      .auto_increment()
                      .primary_key(),
                )
                .col(
                    ColumnDef::new(Users::Name).string().not_null()
                )
                .col(
                    ColumnDef::new(Users::Email).string().not_null()
                )
                .col(
                    ColumnDef::new(Users::CreatedAt)
                      .timestamp_with_time_zone()
                      .not_null()
                      .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(Users::UpdatedAt)
                      .timestamp_with_time_zone()
                      .not_null()
                      .default(Expr::current_timestamp())
                )
                .to_owned(),
          )
          .await?;

        let db = manager.get_connection();

        // トリガー関数の作成
        db
          .execute_unprepared(
              r#"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = NOW();
                    RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#,
          )
          .await?;

        // トリガーの作成
        db
          .execute_unprepared(
              r#"
                CREATE TRIGGER update_updated_at
                BEFORE UPDATE ON users
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
                "#,
          )
          .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // トリガーの削除
        db
          .execute_unprepared(
              r#"
                DROP TRIGGER IF EXISTS update_updated_at ON users;
                "#,
          )
          .await?;

        // トリガー関数の削除
        db
          .execute_unprepared(
              r#"
                DROP FUNCTION IF EXISTS update_updated_at_column();
                "#,
          )
          .await?;

        manager
          .drop_table(Table::drop().table(Users::Table).to_owned())
          .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}
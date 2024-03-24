use sea_orm_migration::prelude::*;

use crate::m20220101_000001_users_roles::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()

                    .col(
                        ColumnDef::new(Sessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )

                    .col(
                        ColumnDef::new(Sessions::UserId)
                            .integer()
                            .not_null()
                    )

                    .col(
                        ColumnDef::new(Sessions::Token)
                            .string()
                    )
                        .to_owned()
            )
                .await?;
        
        
        // Defining FOREIGN KEYS
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-sessions-user-id")
                    .from(Sessions::Table, Sessions::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned()
            )
                .await
                
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Sessions::Table)
                    .name("fk-sessions-user-id")
                    .to_owned()
            )
                .await?;

        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    UserId,
    Token,
}
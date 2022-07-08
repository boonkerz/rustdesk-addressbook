use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000002_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
            .create_table(
                Table::create()
                    .table(Peer::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Peer::Id).string().not_null())
                    .col(ColumnDef::new(Peer::Uuid).string().not_null().primary_key())
                    .col(ColumnDef::new(Peer::User).string().not_null())
                    .col(ColumnDef::new(Peer::Alias).string())
                    .col(ColumnDef::new(Peer::Hostname).string())
                    .col(ColumnDef::new(Peer::Platform).string())
                    .col(ColumnDef::new(Peer::Tags).string())
                    .col(ColumnDef::new(Peer::Created).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Peer::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Peer {
    Table,
    Id,
    Uuid,
    User,
    Alias,
    Hostname,
    Platform,
    Created,
    Tags
}


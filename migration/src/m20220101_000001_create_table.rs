use sea_orm_migration::{prelude::*, sea_orm::{EnumIter, DerivePrimaryKey, PrimaryKeyTrait}};
use sea_orm::Statement;


#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE "events" (
            "sequence_number" INT8 NOT NULL,
            "creation_number" INT8 NOT NULL,
            "account_address" STRING(66) NOT NULL,
            "transaction_version" STRING(66) NOT NULL,
            "transaction_block_height" INT8 NOT NULL,
            "event_index" INT8 NOT NULL,
            "event_type" STRING NOT NULL,
            "data" JSONB NOT NULL,
            "inserted_at" TIMESTAMP(6) NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Events::Table)
        //             .if_not_exists()
        //             .col(ColumnDef::new(Events::SequenceNumber).integer().not_null())
        //             .col(ColumnDef::new(Events::CreationNumber).integer().not_null())
        //             .col(ColumnDef::new(Events::AccountAddress).string_len(66).not_null())
        //             .col(ColumnDef::new(Events::TransactionVersion).string_len(66).not_null().primary_key())
        //             .col(ColumnDef::new(Events::TransactionBlockHeight).integer().not_null())
        //             .col(ColumnDef::new(Events::EventIndex).integer().not_null().primary_key())
        //             .col(ColumnDef::new(Events::EventType).string().not_null())
        //             .col(ColumnDef::new(Events::Data).json().not_null())
        //             .col(ColumnDef::new(Events::InsertedAt).timestamp().not_null().default(Expr::current_timestamp()))
        //             .to_owned(),
        //     )
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"DROP TABLE "events""#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
        // manager
        //     .drop_table(Table::drop().table(Events::Table).to_owned())
        //     .await
    }
}

// #[derive(DeriveIden)]
// enum Events {
//     Table,
//     SequenceNumber,
//     CreationNumber,
//     AccountAddress,
//     TransactionVersion,
//     TransactionBlockHeight,
//     EventIndex,
//     EventType,
//     Data,
//     InsertedAt,
// }

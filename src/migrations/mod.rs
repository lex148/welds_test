use crate::errors::Result;
use welds::migrations::{create_table, types::Type, MigrationFn, MigrationStep, TableState};

pub async fn up(db: &dyn welds::TransactStart) -> Result<()> {
    let list: Vec<MigrationFn> = vec![
        m20250210064356_create_table_dogs::step, /* MIGRATION LIST MARKER */
    ];
    welds::migrations::up(db, list.as_slice()).await?;
    Ok(())
}

mod m20250210064356_create_table_dogs;

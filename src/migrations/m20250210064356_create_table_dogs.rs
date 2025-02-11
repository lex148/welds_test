use welds::errors::Result;
use welds::migrations::{create_table, types::Type, MigrationStep, TableState};

pub(super) fn step(_state: &TableState) -> Result<MigrationStep> {
    let m = create_table("dogs")
        .id(|c| c("id", Type::IntBig))
        .column(|c| c("name", Type::String))
        .column(|c| c("owner_name", Type::String))
        .column(|c| c("breed", Type::String))
        .column(|c| c("pats", Type::Int))
        .column(|c| c("wags", Type::Int))
        .column(|c| c("scratches", Type::Int));
    Ok(MigrationStep::new("m20250210064356_create_table_dogs", m))
}

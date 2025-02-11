use crate::DbClient;
use welds::prelude::*;

#[derive(Debug, WeldsModel, PartialEq)]
#[welds(table = "dogs")]
pub(crate) struct Dog {
    #[welds(primary_key)]
    pub id: i64,
    pub name: String,
    pub owner_name: String,
    pub breed: String,
    pub pats: i32,
    pub wags: i32,
    pub scratches: i32,
}

pub(crate) async fn seed(db: &DbClient) -> crate::errors::Result<()> {
    // NOTE: lots of little small DB write can be very expansive.
    // if you batch them in a transaction, you will have MUST better performance
    //
    // If you are doing more than this, you might want to consider using BULK operations

    let transaction = db.begin().await?;

    Dog::all().delete(&transaction).await?;

    for _i in 0..1000 {
        let mut dog = DbState::new_uncreated(Dog {
            id: 0,
            name: "Spot".to_owned(),
            owner_name: "Bob".to_owned(),
            breed: "golden retriever".to_owned(),
            pats: 22,
            wags: 122,
            scratches: 34,
        });
        dog.save(&transaction).await?;
    }

    transaction.commit().await?;

    println!("SEEDED test data");

    Ok(())
}

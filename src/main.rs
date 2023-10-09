mod error;
mod model;

use self::error::Result;
use model::{
    inventory_transaction::{DepositForCreate, DepositForCreateItem},
    pageable::Pageable,
    ModelManager,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mm = ModelManager::new().await?;

    // model::organization::provision_organization(
    //     &mm,
    //     model::organization::OrganizationForProvision {
    //         name: "hello".to_owned(),
    //         display_name: "hello".to_owned(),
    //     },
    // )
    // .await?;

    // model::inventory_log::add_logs(
    //     &mm,
    //     vec![model::inventory_log::InventoryLogForCreate {
    //         action: model::enums::InventoryLogAction::Incoming,
    //         product_id: 1,
    //         quantity: 10,
    //         warehouse_id: 1,
    //         price: 100.0,
    //     }],
    // )
    // .await?;

    // model::inventory_transaction::deposit(
    //     &mm,
    //     DepositForCreate {
    //         items: vec![DepositForCreateItem {
    //             price: 100.0,
    //             product_id: 1,
    //             quantity: 10,
    //             warehouse_id: 1,
    //         }],
    //     },
    // )
    // .await?;

    // let r = model::inventory_transaction::get_deposit_logs(
    //     &mm,
    //     Pageable {
    //         items_per_page: 100,
    //         page: 1,
    //     },
    // )
    // .await?;

    let r = model::products::get_all_stock_levels(&mm).await?;

    println!("{:?}", r);
    Ok(())
}

mod crypt;
mod ctx;
mod error;
mod model;
mod web;

use std::net::SocketAddr;

use self::error::Result;
use axum::Router;
use crypt::hash_value;
use model::ModelManager;
use web::{
    page_test::page_test_route, pages::categories::pages_cateogries,
    routes_inventory_deposit::routes_inventory_deposit, routes_test::test_routes,
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

    // let r = model::products::get_all_stock_levels(&mm).await?;

    // println!("{:?}", r);

    let routes_all = Router::new()
        .merge(pages_cateogries(mm.clone()))
        .merge(test_routes(mm.clone()))
        .merge(page_test_route(mm.clone()))
        .merge(routes_inventory_deposit(mm.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    println!("{}", hash_value("password").unwrap());

    Ok(())
}

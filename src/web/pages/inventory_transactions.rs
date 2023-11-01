use axum::Router;

use crate::model::ModelManager;

pub fn pages_products(mm: ModelManager) -> Router {
    Router::new()
        // read
        .route("/inventories/transactions/deposits", get(products_page))
        // create
        .route("/products", post(create_category))
        .with_state(mm)
}

// region: Table templates
// region:  Deposit
#[derive(Template)]
#[template(path = "products/fragments/table_entries.html")]
pub struct TableEntries {
    pub products: Vec<ProductWithStockLevel>,
}

#[derive(Template)]
#[template(path = "products/fragments/table_entry.html")]
pub struct TableEntry {
    pub product: ProductWithStockLevel,
}
// endregion:   Deposit
// endregion: Table templates

// region: Read
// region:  Deposit

// endregion:   Deposit
// endregion: Read

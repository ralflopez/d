use crate::ctx::Ctx;
use crate::model;
use crate::model::products::{
    get_all_products_with_stock_levels, get_product_with_stock_level, ProductForCreate,
    ProductForSearch, ProductForUpdate, ProductWithStockLevel,
};
use crate::model::ModelManager;
use crate::web::error::Result;
use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};

use super::toasts::{with_toast_response, ToastSeverity};

pub fn pages_products(mm: ModelManager) -> Router {
    Router::new()
        // read
        .route("/products", get(products_page))
        .route("/products/:id", get(get_product_row))
        // create
        .route("/products", post(create_category))
        // delete
        .route("/products/:id", delete(delete_category_row))
        .route("/products/:id/delete", get(delete_product_row_action))
        // update
        .route("/products/:id", put(update_product_row))
        .route("/products/:id/edit", get(get_editable_product_row))
        // search
        .route("/products/search", post(search_products))
        .with_state(mm)
}

// region: Table templates
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
// endregion: Table templates

// region: Handlers
// region: Read
#[derive(Template)]
#[template(path = "products/pages_products.html")]
pub struct ProductsPage {
    pub products: Vec<ProductWithStockLevel>,
}
pub async fn products_page(State(mm): State<ModelManager>) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let products = get_all_products_with_stock_levels(&ctx, &mm).await?;

    let template = ProductsPage { products };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}

pub async fn get_product_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let product = get_product_with_stock_level(&ctx, &mm, id).await?;

    let template = TableEntry {
        product: product.unwrap(),
    };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}
// endregion: Read

// region: Create
pub async fn create_category(
    State(mm): State<ModelManager>,
    Form(product_for_create): Form<ProductForCreate>,
) -> Result<impl IntoResponse> {
    println!("{:?}", product_for_create);
    // Check authorization
    let ctx = Ctx::new(1, 1);
    model::products::create_product(&ctx, &mm, product_for_create).await?;

    let products = get_all_products_with_stock_levels(&ctx, &mm).await?;
    let template = TableEntries { products };
    let reply_html = template.render().unwrap();

    Ok((
        StatusCode::OK,
        Html(with_toast_response(
            reply_html,
            ToastSeverity::Succes,
            "Product Created",
        ))
        .into_response(),
    ))
}
// endregion: Create

// region: Delete
pub async fn delete_category_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);
    model::products::delete_product(&ctx, &mm, id).await?;

    let products = get_all_products_with_stock_levels(&ctx, &mm).await?;
    let template = TableEntries { products };
    let reply_html = template.render().unwrap();
    Ok((
        StatusCode::OK,
        Html(with_toast_response(
            reply_html,
            ToastSeverity::Succes,
            "Product Deleted",
        ))
        .into_response(),
    ))
}

#[derive(Template)]
#[template(path = "products/fragments/delete_row_action.html")]
pub struct DeleteRowAction {
    pub product: ProductWithStockLevel,
}
pub async fn delete_product_row_action(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);
    let product = model::products::get_product_with_stock_level(&ctx, &mm, id).await?;

    let template = DeleteRowAction {
        product: product.unwrap(),
    };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}
// endregion: Delete

// region: Update
pub async fn update_product_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
    Form(product_for_update): Form<ProductForUpdate>,
) -> Result<impl IntoResponse> {
    let ctx = Ctx::new(1, 1);

    model::products::update_product(&ctx, &mm, id, product_for_update).await?;

    let product = model::products::get_product_with_stock_level(&ctx, &mm, id).await?;
    let template = TableEntry {
        product: product.unwrap(),
    };

    let reply_html = template.render().unwrap();
    Ok((
        StatusCode::OK,
        Html(with_toast_response(
            reply_html,
            ToastSeverity::Succes,
            "Product Updated",
        ))
        .into_response(),
    ))
}

#[derive(Template)]
#[template(path = "products/fragments/editable_row.html")]
pub struct EditableRow {
    pub product: ProductWithStockLevel,
}
pub async fn get_editable_product_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let product = get_product_with_stock_level(&ctx, &mm, id).await?;

    let template = EditableRow {
        product: product.unwrap(),
    };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}
// endregion: Update

// region: Search
pub async fn search_products(
    State(mm): State<ModelManager>,
    Form(product_for_search): Form<ProductForSearch>,
) -> Result<impl IntoResponse> {
    println!("{:?}", product_for_search);
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let products = model::products::search_products(&ctx, &mm, product_for_search).await?;

    let template = TableEntries { products };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}
// endregion: Search
// endregion: Handlers

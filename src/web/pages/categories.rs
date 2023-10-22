use crate::ctx::Ctx;
use crate::model;
use crate::model::category::{get_all_categories, get_category_by_id, Category};
use crate::model::ModelManager;
use crate::web::error::Result;
use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{delete, get, post, put};
use axum::{Form, Router};
use serde::Deserialize;

use super::toasts::{with_toast_response, ToastSeverity};

pub fn pages_cateogries(mm: ModelManager) -> Router {
    Router::new()
        .route("/categories", get(categories_page))
        .route("/categories", post(create_category))
        .route("/categories/:id", get(get_category_row))
        .route("/categories/:id", delete(delete_category_row))
        .route("/categories/:id/delete", get(delete_category_row_action))
        .route("/categories/:id", put(update_category_row))
        .route("/categories/:id/edit", get(edit_category_row))
        .route("/categories/search", post(search_category))
        .with_state(mm)
}

// region: Table templates
#[derive(Template)]
#[template(path = "categories/fragments/table_entries.html")]
pub struct TableEntries {
    pub categories: Vec<Category>,
}

#[derive(Template)]
#[template(path = "categories/fragments/table_entry.html")]
pub struct TableEntry {
    pub category: Category,
}

#[derive(Template)]
#[template(path = "categories/fragments/edit_row.html")]
pub struct EditRowFragment {
    pub category: Category,
}
// endregion: Table templates

// region: Handlers
#[derive(Template)]
#[template(path = "categories/pages_categories.html")]
pub struct CategoriesPage {
    pub categories: Vec<Category>,
}
pub async fn categories_page(State(mm): State<ModelManager>) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let categories = get_all_categories(&ctx, &mm).await?;

    let template = CategoriesPage { categories };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}

#[derive(Deserialize)]
pub struct CategoryForCreate {
    name: String,
}
pub async fn create_category(
    State(mm): State<ModelManager>,
    Form(create): Form<CategoryForCreate>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);
    model::category::create_category(&ctx, &mm, create.name).await?;

    let categories = get_all_categories(&ctx, &mm).await?;
    let template = TableEntries { categories };
    let reply_html = template.render().unwrap();

    Ok((
        StatusCode::OK,
        Html(with_toast_response(
            reply_html,
            ToastSeverity::Succes,
            "Category Created",
        ))
        .into_response(),
    ))
}

pub async fn get_category_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let category = get_category_by_id(&ctx, &mm, id).await?;

    let template = TableEntry { category };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}

pub async fn delete_category_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);
    println!("category");
    println!("{}", id);
    model::category::delete_category(&ctx, &mm, id).await?;

    let categories = get_all_categories(&ctx, &mm).await?;
    let template = TableEntries { categories };
    let reply_html = template.render().unwrap();
    Ok((
        StatusCode::OK,
        Html(with_toast_response(
            reply_html,
            ToastSeverity::Succes,
            "Category Deleted",
        ))
        .into_response(),
    ))
}

#[derive(Template)]
#[template(path = "categories/fragments/delete_row_action.html")]
pub struct DeleteRowAction {
    pub category: Category,
}
pub async fn delete_category_row_action(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);
    let category = model::category::get_category_by_id(&ctx, &mm, id).await?;

    let template = DeleteRowAction { category };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}

#[derive(Deserialize)]
pub struct CategoryForUpdate {
    name: String,
}
pub async fn update_category_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
    Form(new_category): Form<CategoryForUpdate>,
) -> Result<impl IntoResponse> {
    let ctx = Ctx::new(1, 1);
    println!("category");
    let category = model::category::update_category(
        &ctx,
        &mm,
        model::category::CategoryForUpdate {
            id,
            name: new_category.name,
        },
    )
    .await?;

    let template = TableEntry { category };
    let reply_html = template.render().unwrap();
    Ok((
        StatusCode::OK,
        Html(with_toast_response(
            reply_html,
            ToastSeverity::Succes,
            "Category Updated",
        ))
        .into_response(),
    ))
}

pub async fn edit_category_row(
    State(mm): State<ModelManager>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let category = get_category_by_id(&ctx, &mm, id).await?;

    let template = EditRowFragment { category };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}

#[derive(Deserialize)]
pub struct CategoryForSearch {
    search: String,
}
pub async fn search_category(
    State(mm): State<ModelManager>,
    Form(category_for_search): Form<CategoryForSearch>,
) -> Result<impl IntoResponse> {
    // Check authorization
    let ctx = Ctx::new(1, 1);

    let categories =
        model::category::search_category(&ctx, &mm, category_for_search.search).await?;

    let template = TableEntries { categories };
    let reply_html = template.render().unwrap();
    Ok((StatusCode::OK, Html(reply_html).into_response()))
}
// endregion: Handlers

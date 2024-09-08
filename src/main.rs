mod schemas;
mod resolvers;
mod utils;
mod services;

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use resolvers::user_resolver::Query;
use utils::db::DatabaseConnection;

async fn graphql_handler(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DatabaseConnection::new().await;

    let schema = Schema::build(Query {db: db.clone()}, EmptyMutation, EmptySubscription)
        .data(db)
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)))
            .service(web::resource("/").route(web::get().to(graphql_playground)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
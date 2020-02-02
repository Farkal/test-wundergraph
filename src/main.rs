use std::env;

use actix_web::{
    middleware, web,
    web::{Data, Json},
    App, Error as ActixError, HttpResponse, HttpServer,
};
use diesel::backend::Backend;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, Identifiable};
use env_logger;
use juniper::http::GraphQLRequest;
use juniper::LookAheadSelection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use test_wundergraph::generated::*;
use test_wundergraph::*;
use wundergraph::error::Result as WunderResult;
use wundergraph::query_builder::selection::offset::ApplyOffset;
use wundergraph::query_builder::selection::{BoxedQuery, LoadingHandler, QueryModifier};
use wundergraph::scalar::WundergraphScalarValue;
use wundergraph::WundergraphContext;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate diesel_migrations;

// actix integration stuff
#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLData(GraphQLRequest<WundergraphScalarValue>);

pub type DBConnection = diesel::PgConnection;

#[derive(Debug)]
pub struct MyContext<Conn>
where
    Conn: Connection + 'static,
{
    conn: PooledConnection<ConnectionManager<Conn>>,
}

impl<Conn> MyContext<Conn>
where
    Conn: Connection + 'static,
{
    pub fn new(conn: PooledConnection<ConnectionManager<Conn>>) -> Self {
        Self { conn }
    }
}

impl<T, C, DB> QueryModifier<T, DB> for MyContext<C>
where
    C: Connection<Backend = DB>,
    DB: Backend + ApplyOffset + 'static,
    T: LoadingHandler<DB, Self>,
    Self: WundergraphContext,
    Self::Connection: Connection<Backend = DB>,
{
    fn modify_query<'a>(
        &self,
        _select: &LookAheadSelection<'_, WundergraphScalarValue>,
        query: BoxedQuery<'a, T, DB, Self>,
    ) -> WunderResult<BoxedQuery<'a, T, DB, Self>> {
        match T::TYPE_NAME {
            //            "Heros" => Err(Error::from_boxed_compat(String::from("Is user").into())),
            _ => Ok(query),
        }
    }
}

impl WundergraphContext for MyContext<DBConnection> {
    type Connection = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<DBConnection>>;

    fn get_connection(&self) -> &Self::Connection {
        &self.conn
    }
}

pub type Schema<Ctx> =
    juniper::RootNode<'static, Query<Ctx>, Mutation<Ctx>, WundergraphScalarValue>;

#[derive(Clone)]
struct AppState {
    schema: Arc<Schema<MyContext<DBConnection>>>,
    pool: Arc<Pool<ConnectionManager<DBConnection>>>,
}

async fn graphql(
    Json(GraphQLData(data)): Json<GraphQLData>,
    st: Data<AppState>,
) -> Result<HttpResponse, ActixError> {
    let ctx = MyContext::new(st.get_ref().pool.get().expect("Fail to get pool"));
    let res = data.execute(&st.get_ref().schema, &ctx);
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&res)?))
}

diesel_migrations::embed_migrations!("migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    std::env::set_var("RUST_LOG", "actix_web=info");
    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()),
        env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
        env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()),
        env::var("POSTGRES_DB").unwrap_or_else(|_| "test_wunder".to_string())
    );
    env_logger::init();

    let manager = ConnectionManager::<DBConnection>::new(db_url);
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to init pool");

    let query = Query::<MyContext<DBConnection>>::default();
    let mutation = Mutation::<MyContext<DBConnection>>::default();
    let schema = Schema::new(query, mutation);

    let schema = Arc::new(schema);
    let pool = Arc::new(pool);
    let data = AppState { schema, pool };

    let my_url = env::var("MY_URL").unwrap_or_else(|_| String::from("127.0.0.1:8088"));

    println!("Started http server: {}", my_url);

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Logger::default())
            .route("/graphql", web::get().to(graphql))
            .route("/graphql", web::post().to(graphql))
    })
    .bind(&my_url)
    .expect("Failed to start server")
    .run().await
}

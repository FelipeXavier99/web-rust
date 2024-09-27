pub mod app;
pub mod controller;
pub mod database;
pub mod error;
pub mod models;
pub mod render;
pub mod result;
pub mod routes;
pub mod utils;
pub mod view;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
// use actix_web::{cookie::Key, , , Responder};
// use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use app::AppState;
use controller::ping;
use env_logger::Env;
use minijinja::context;
use serde_json::json;

use actix_files::NamedFile;
use std::path::PathBuf;

//exemplo mais simplificado

// #[tokio::main]
// async fn main() -> std::io::Result<()> {
//     // Conexão com o banco de dados
//     let database_url = "postgres://user:password@localhost/mydb";
//     let pool = PgPool::connect(database_url).await.expect("Failed to create pool.");

//     // Iniciar o servidor HTTP
//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .service(get_sales)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();
    // let port = 3001;  //std::env::var("PORT_API").unwrap().parse::<u32>().unwrap();
    let port = 8080;
    let host = "localhost".to_owned(); //std::env::var("SERVER_API").unwrap();
    let database = database::DbInstance::init().await;

    let _ = sqlx::migrate!()
        .run(&database.conn.clone())
        .await
        .map_err(|e| format!("Erro na migração do banco de dados {e}"));
    let app_data = web::Data::new(app::AppState {
        client: reqwest::Client::new(),
        database,
    });

    // minijinja_embed::load_templates!(&mut env);
    let _database_url = std::env::var("DATABASE_URL").unwrap();

    println!("🌎 server running at {}:{}", host.clone(), port);

    // let host1 = host.clone();
    let host2 = host.clone();
    HttpServer::new(move || {
        // let cors = actix_cors::Cors::default()
        // most restritive
        // .allowed_origin(format!("http://{}:{port}", host1 ).as_str())
        // .allowed_origin(format!("http://localhost:{port}" ).as_str())
        // .allowed_origin(format!("http://27.0.0.1:{port}" ).as_str())
        // .allowed_origin(format!("http://www.pedidonanuvem.com.br:{port}" ).as_str())
        // most permissive
        // .allow_any_header()
        // .allow_any_origin()
        // .allow_any_method()
        // .expose_any_header()
        // .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            // .wrap(cors) //enable cors
            // .wrap(SessionMiddleware::new(
            //     CookieSessionStore::default(),
            //     secret_key.clone(),
            // )) //use cookies
            .wrap(middleware::Logger::new(
                "%{r}a %r %s %b %{Referer}i %{User-Agent}i %T",
            )) // enable logger
            // .service(
            //     SwaggerUi::new("/swagger-ui/{_:.*}")
            //         .url("/api-docs/openapi.json", ApiDoc::openapi()),
            // )
            .service(
                actix_files::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .route("/", web::get().to(index)) // Atualizando a rota para "/"
            //ANTIGO RICARDO
            // .route("/index", web::get().to(index))
            .service(ping)
            // .service(index)
            .service(actix_files::Files::new("/images", "./static/img"))
            .configure(routes::vendas::routes)
    })
    .bind(format!("{}:{}", host2, port))?
    .run()
    .await
}

// Serving the Registration and sign-in page
async fn index(
    // session: Session, usuário autenticado
    data: web::Data<AppState>,
    _req: HttpRequest,
) -> impl Responder {
    // let path: PathBuf = "./static/index.html".parse().unwrap();
    // Ok(NamedFile::open(path).unwrap())
    // let usuario = auth::session::get_user(&data.database.conn, &session).await;

    let usuario = json!({"nome" : "Admin", "email": "contato@"});
    render::render_minijinja("index.html", context!(usuario))

    // let path: PathBuf = "/opt/web-rust/templates/index.html".parse().unwrap();
    // NamedFile::open(path)
}

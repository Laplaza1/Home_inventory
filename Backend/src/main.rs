
use axum_governor::GovernorLayer;
use real::RealIpLayer;

use lazy_limit::{init_rate_limiter, Duration as dur, RuleConfig};

use tower_http::cors::{CorsLayer, AllowOrigin};


use axum::{
    http::{ HeaderValue, Method}, routing::{delete, get, post, put}, Router
};
use std::{env, sync::Arc};


mod routes;
use routes::{*};
use log::{*};
mod Auth;
use Auth::{*};

#[tokio::main]
async fn main() {

    match simple_logging::log_to_file(
        match env::var("LOG_FILE")
            {
                Ok(x)=>{x},
                Err(error)=>{error!("{error}@ finding log file! ");std::process::exit(1)},

            }, LevelFilter::Info)
                {
                    Ok(_)=>{},
                    Err(error)=>{error!("{error}")}
                };
    info!("Application Starting up ");
    
    init_rate_limiter!(
        default: RuleConfig::new(dur::seconds(1), 5), // 5 req/s globally
        routes: [
            ("/api/special", RuleConfig::new(dur::seconds(1), 10)),
        ]
    ).await;
    let origins = vec![
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_static("http://localhost"),
        HeaderValue::from_static("http://127.0.0.1:5500"),
        HeaderValue::from_static("https://laplaza1.github.io"),
    ];

    let cliento = handle_client().await;
    let state = AppState { client: Arc::new(cliento) };
    //let allowed_origins:[tower_http::cors::AllowOrigin;2] = ["http://localhost".parse().unwrap(),"http://127.0.0.1:5500".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE]) // Allow GET and POST
        .allow_origin(AllowOrigin::list(origins))
        .allow_headers([axum::http::header::CONTENT_TYPE,axum::http::header::COOKIE])
        .allow_credentials(true);

    
    let app = Router::new()

    //User
    .route("/user", post(create_user)).with_state(state.clone())
    .route("/user/{user_id}", get(check_user)).with_state(state.clone())
    .route("/user",put(change_user)).with_state(state.clone())
    .route("/user/{user_id}",delete(delete_user)).with_state(state.clone())
    

    
    .route("/pending",post(create_pending)).with_state(state.clone())



    //login
    .route("/login",post(login)).with_state(state.clone())

    //Item
    .route("/item",post(insert_item)).with_state(state.clone())
    .route("/item",get(get_item)).with_state(state.clone())
    .route("/specificItem/{item_id}",get(specific_item)).with_state(state.clone())
    .route("/item",put(change_item)).with_state(state.clone())
    .route("/item",delete(delete_item)).with_state(state.clone())

    //recipe
    .route("/recipe",post(create_recipe)).with_state(state.clone())
    .route("/recipe",get(get_recipes)).with_state(state.clone())
    // .route("/recipes/{recipeID}", get(specific_recipe))
    // .route("/recipe/{recipeID}",delete(delete_recipe))
    
    .route("/cookies", get(show_cookies))
    .route("/test",get(test)).with_state(state.clone())


    .route("/notify",post(send_notification).with_state(state.clone()))
    //Data paths

    .route("/data",get(pull_data)).with_state(state.clone())
    .route("/graph/{id}",get(pull_specific_data)).with_state(state.clone())
    .route("/admin_data",get(general_data)).with_state(state.clone())



    .layer(cors)
    .layer(tower::ServiceBuilder::new()
            .layer(RealIpLayer::default()) // Extracts the real IP
            .layer(GovernorLayer::default()));

     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}








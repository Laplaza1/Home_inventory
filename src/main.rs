use bson::{datetime, oid::Error, Array, DateTime, Document,Decimal128};
use ::futures::StreamExt;
use serde_json::{
    Value,
    json
};

use tower_http::cors::{CorsLayer, Any};
use rand::{Rng};
use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, Path, Request},
    http::{HeaderMap, Method, StatusCode,header::COOKIE}, response::{IntoResponse, Json}, routing::{delete, get, post, put}, Router
};
use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions,ResolverConfig},
    Client, Collection,
    bson::oid::ObjectId,
    bson::Bson
};
use serde::{Serialize, Deserialize};

// for future additions
use reqwest;
use futures::{future::ok, io::Cursor, sink::SinkExt, TryStreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower::ServiceExt;





#[derive(Debug,Serialize,Deserialize,Clone)]
struct Change{
    user_name:String,
    item_id:Option<ObjectId>,
    old_quantity:i64,
    new_quantity:i64
}


#[derive(Debug, Serialize, Deserialize,Clone)]
struct User{
    user_name:String,
    title:String,
    token:i32    
}

#[derive(Debug, Serialize, Deserialize,Clone)]
struct Item {
    
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>, // Optional ObjectId for _id
    item_name: String,
    category: Vec<String>,
    quantity: i64,
    method_measure:String,
    unit_price: Decimal128,
    date: DateTime
}
#[derive(Debug, Serialize, Deserialize,Clone)]
struct Token{
    token:String

}




#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE]) // Allow GET and POST
        .allow_origin(Any)
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    
    let app = Router::new()

    //User
    .route("/user", post(create_user))
    .route("/user/{user_id}", get(check_user))
    .route("/user/{user_id}",put(change_user))
    .route("/user/{user_id}",delete(delete_user))
    
    //login
    .route("/login",post(login))

    //Item
    .route("/item",post(insert_item))
    .route("/item",get(get_item))
    .route("/item/{item_id}",put(change_item))
    .route("/item/{item_id}",delete(delete_item))
    .layer(cors);

     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}





//User functions

async fn create_user(Json(payload): Json<serde_json::Value>){
    println!("Creating User!");
    //Checks if Username was sent
    let _username = payload.get("username")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::NOT_ACCEPTABLE, "Missing or invalid 'username' field".to_string()));
    //Checks if password was sent
    let _password = payload.get("password")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::NOT_ACCEPTABLE, "Missing or invalid 'password' field".to_string())) ;

    println!("Username and Password are valid");


 
}


async fn check_user()->Result<Json<Vec<User>>,(StatusCode, String)>{
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let user: Collection<User> = client.database("test").collection("user");
    let curser = user
    .find(None,None)
    .await
    .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();
    let users: Vec<User> = curser.try_collect().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    println!("{:#?}",users);

    return Ok(Json(users))

}

async fn change_user(headers:HeaderMap, Json(payload): Json<serde_json::Value>)->Result<String,(StatusCode,String)>{

    let user = payload.get("user_id");
    let token = headers.get(COOKIE).and_then(|value|value.to_str().ok()).ok_or((StatusCode::BAD_REQUEST, "Missing or invalid Cookie header".to_string()));
    println!("User: {:?} was given token: {:#?}",user,token);

    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let user: Collection<User> = client.database("test").collection("user");
    let _curser = user
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();
    



    return Ok("If you're reading this then ya changed information on user".to_string())
}

async fn delete_user(){

}





///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////




//login function
async fn login()-> Result<(Json<Token>),(StatusCode,String)>{
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let token:Token=Token { token:"test12".to_string() };
    return Ok(Json(token))

}






///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////






//Item function

async fn get_item()->Result<Json<Vec<Item>>,(StatusCode,String)>{
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let item: Collection<Item> = client.database("test").collection("item");

    println!("Item var: {:#?}",item);


    let curser = item
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();


    let items:Vec<Item> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x))})?;
    return Ok(Json(items))
}


async fn insert_item(Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{
    
    
    let item_name: String = payload.get("item_name").and_then(|x|Some(x.to_string())).unwrap();

    let category:Vec<String> = payload.get("category").and_then(|x|Some(x.as_array())).unwrap().unwrap().iter().map(|x|x.to_string()).collect();
    let quantity:  i64 = payload.get("quantity").and_then(|x|Some(x.as_i64())).unwrap().unwrap();

    let method_measure: String = payload.get("method_measure").and_then(|x|Some(x.to_string())).unwrap();

    let unit_price:Decimal128 = payload.get("unit_price").and_then(|x|Some(x.to_string().parse::<Decimal128>().ok())).unwrap().unwrap();

    let date:DateTime=  payload.get("date").and_then(|x|Some(bson::DateTime::parse_rfc3339_str(x.to_string()))).unwrap().unwrap();
    


    let new_item = Item{id:None,item_name,category,quantity,method_measure,unit_price,date};


    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let item: Collection<Item> = client.database("test").collection("item");

    

    let _cursor = item.insert_one(new_item, None);


    return Ok(Json(json!({"Success":true})))

}

async fn change_item(Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    let item_id: i64=payload.get("item_id").and_then(|x|Some(x.as_i64().unwrap())).unwrap();
    let item_name: String = payload.get("item_name").and_then(|x|Some(x.to_string())).unwrap();
    let category:Vec<String> = payload.get("category").and_then(|x|Some(x.as_array())).unwrap().unwrap().iter().map(|x|x.to_string()).collect();
    let quantity:  i64 = payload.get("quantity").and_then(|x|Some(x.as_i64())).unwrap().unwrap();
    let method_measure: String = payload.get("method_measure").and_then(|x|Some(x.to_string())).unwrap();
    let unit_price:f32 = payload.get("unit_price").and_then(|x|Some(x.to_string().parse::<f32>().ok())).unwrap().unwrap();
    let date:DateTime=  payload.get("date").and_then(|x|Some(bson::DateTime::parse_rfc3339_str(x.to_string()))).unwrap().unwrap();
    

    let find_item = doc!{"item_id":item_id};
    let new_item = doc! {"$set":{"item_id":item_id,"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date}};


    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let itemo: Collection<Item> = client.database("test").collection("item");
    let _cursor = itemo.update_one(find_item,new_item,None).await;
    let change_log :&Collection<Change>=client.database("test").collection("change");


    return Ok(Json(json!({"Success":true})))

}

async fn delete_item(Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    let item_id: i64=payload.get("_id").and_then(|x|Some(x.as_i64().unwrap())).unwrap();
    let filtered_document = doc! {"_id":item_id};

    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let item: Collection<Item> = client.database("test").collection("item");

    

    let _cursor = item.delete_one(filtered_document, None);

    return Ok(Json(json!({"Success":true})))


}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////


//Fun functions




async fn display_fractial()->Result<&Json<&Value>,(StatusCode,String)>{




}
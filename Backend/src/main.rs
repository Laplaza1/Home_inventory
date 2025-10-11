use bson::{ oid::Error, DateTime, Document,Decimal128};
use chrono::{Utc};
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
use core::f32;
use std::{env, future::IntoFuture};

use mongodb::{
    bson::{doc, oid::ObjectId, Bson}, options::{ClientOptions,ResolverConfig}, results::UpdateResult, Client, Collection
};
use serde::{Serialize, Deserialize};

// for future additions
use reqwest;
use futures::{future::ok, io::Cursor, sink::SinkExt, TryStreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower::ServiceExt;




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
    .route("/specificItem/{item_id}",get(specific_Item))
    .route("/item",put(change_item))
    .route("/item",delete(delete_item))


    //get changed data
    .route("/data",get(pull_data))
    .route("/graph/{id}",get(pull_specific_data))

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


async fn specific_Item()->Result<Json<Vec<Item>>,(StatusCode,String)>{
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
    
    println!("payload: {:#?}",payload);
    
    let item_name: String = match payload.get("name")
        {
            Some(Value::String(x))=>{x.to_string()},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    //.and_then(|x|Some(x.to_string())).unwrap();
    let category:Vec<String> = match payload.get("categories") {
        Some(Value::String(s))=>{vec![s.to_string()]},
        Some(Value::Array(s))=>{let arrayer:Vec<String>= s.iter().map(|x|x.to_string()).collect(); arrayer},
        _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}      
    };
    
    let quantity:  i64 = match payload.get("amount")
        {
            Some(Value::String(x))=>{x.parse::<i64>().expect("This shouldn't be wrong if posted through")},
            Some(Value::Number(x))=>{x.as_i64().expect("Should be right")}
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    
    //.and_then(|x|Some(x.as_i64())).unwrap().unwrap();
    
    //.and_then(|x|Some(x.as_i64())).unwrap().unwrap();;
    let method_measure: String = match payload.get("method of measure")
        {
            Some(Value::String(x))=>{x.to_string()}
            _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    let unit_price:Decimal128 =match payload.get("price")
        {
            Some(Value::String(x))=>{x.parse::<Decimal128>().expect("This Decimal cast from String shouldn't mess up but if it does its users fault")},
            Some(Value::Number(x))=>{x.to_string().parse::<Decimal128>().expect("This Decimal cast from Number shouldn't mess up but if it does its users fault")},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
        
        };
    //println!("Method of measure: {:#?}",method_measure);
        //.and_then(|x|Some(x.to_string().parse::<f32>().ok())).unwrap().unwrap();
    let date:DateTime=  match payload.get("time") {
        Some(Value::Number(x))=>{match x.as_i64() {
         Some(x)=>{bson::DateTime::from_millis(x)},
         _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}   
        }}
        _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };

    let newo_item :Document= doc! {"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date};
    println!("{:#?}",newo_item);
    //println!("Doc of items{:#?}",new_item);
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;



    let item: Collection<Document> = client.database("test").collection("item");
    item.insert_one(newo_item , None).await.ok();
    //let x = item.insert_one(document, options).await.ok();

    return Ok(Json(json!({"Success":true})))

}

async fn change_item(Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    
    println!("{:#?}",payload);
    
    let item_id: String=match payload.get("id") {
        Some(Value::String(x))=>{x.to_string()},
        _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))} 
    };
    let item_name: String = match payload.get("name")
        {
            Some(Value::String(x))=>{x.to_string()},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    //.and_then(|x|Some(x.to_string())).unwrap();
    let category:Vec<String> = match payload.get("categories") {
        Some(Value::String(s))=>{vec![s.to_string()]},
        Some(Value::Array(s))=>{let arrayer:Vec<String>= s.iter().map(|x|x.to_string()).collect(); arrayer},
        _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}      
    };
    
    let quantity:  i64 = match payload.get("amount")
        {
            Some(Value::String(x))=>{x.parse::<i64>().expect("This shouldn't be wrong if posted through")},
            Some(Value::Number(x))=>{x.as_i64().expect("Should be right")}
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    
    //.and_then(|x|Some(x.as_i64())).unwrap().unwrap();
    let old_quantity:i64 = match payload.get("oldAmount") 
        {
           Some(Value::String(x))=>{x.parse::<i64>().expect("This shouldn't be wrong if posted through")},
           Some(Value::Number(x))=>{x.as_i64().expect("Should be right")}
           _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
        };
    
    //.and_then(|x|Some(x.as_i64())).unwrap().unwrap();;
    let method_measure: String = match payload.get("method of measure")
        {
            Some(Value::String(x))=>{x.to_string()}
            _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    let unit_price:Decimal128 =match payload.get("price")
        {
            Some(Value::String(x))=>{x.parse::<Decimal128>().expect("This Decimal cast from String shouldn't mess up but if it does its users fault")},
            Some(Value::Number(x))=>{x.to_string().parse::<Decimal128>().expect("This Decimal cast from Number shouldn't mess up but if it does its users fault")},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
        
        };
    println!("Method of measure: {:#?}",method_measure);
        //.and_then(|x|Some(x.to_string().parse::<f32>().ok())).unwrap().unwrap();
    let date:DateTime=  match payload.get("time") {
        Some(Value::Number(x))=>{match x.as_i64() {
         Some(x)=>{bson::DateTime::from_millis(x)},
         _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}   
        }}
        _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };
    println!("{:#?}",item_id);
    let token = payload.get("token");
    let object_id = ObjectId::parse_str(item_id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let find_item = doc!{"_id":object_id};
    let new_item = doc! {"$set":{"_id":object_id,"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date}};


    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    println!("Prior to Collection call");    
    let itemo: Collection<Item> = client.database("test").collection("item");
    println!("After Collection call");

    let _cursor = itemo.update_one(find_item,new_item,None).await.ok();

   

    let difference = Some(quantity-old_quantity).unwrap();
    
    let change_line = doc! {"item":item_id.clone(),"change":difference,"price":unit_price,"date":date};

    let _ = client.database("test").collection("change").insert_one(change_line, None).await;


    return Ok(Json(json!({"Success":true})))

}

async fn delete_item(Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    let item_id: String= match payload.get("id")
        {
            Some(Value::String(x))=>{x.to_string()},
            _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    
    let object_id = ObjectId::parse_str(item_id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let filtered_document = doc! {"_id":object_id};

    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;

    let item: Collection<Item> = client.database("test").collection("item");

    

    let _cursor = item.delete_one(filtered_document, None).await;
    println!("Fulfilled functions");
    return Ok(Json(json!({"Success":true})))


}

async fn pull_data()->Result<Json<Vec<Document>>,(StatusCode,String)>{

    let data:Collection<Document> = match handle_client().await {
        Ok(c) => { c.database("test").collection("change")},
        Err(_) => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };
     let curser = data
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();


    let items:Vec<Document> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x))})?;
    return Ok(Json(items))

}



async fn pull_specific_data(Path(id): Path<String>)->Result<Json<Vec<Document>>,(StatusCode,String)>{
    
    let object_id = ObjectId::parse_str(id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let find_item = doc!{"item":id};
    println!("Object{:#?}",find_item);
    let data:Collection<Document> = match handle_client().await {
        Ok(c) => { c.database("test").collection("change")},
        Err(_) => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };

    println!("Document {:#?}",data);
     let curser = data
        .find(find_item,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();


    let items:Vec<Document> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x))})?;
    println!("Items {:#?}",items);
    
    return Ok(Json(items))

}



////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////


//Fun functions

async fn handle_client()->Result<Client,Error>{
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string())).expect("Error on Client URI");

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e))).expect("Error on Client options");
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e))).expect("Error on client result");
    return Ok(client)
}



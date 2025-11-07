use axum_extra::extract::{cookie,CookieJar};
use bson::{DateTime, Decimal128, Document};
use base64::{engine::general_purpose, Engine as _};
use ::cookie::{Cookie, Expiration, SameSite};
// use chrono::{Utc};
use serde_json::{
    Value,
    json
};
use reqwest;
use tower_http::cors::{CorsLayer, AllowOrigin,Any};

// use rand::{Rng};
use axum::{
    body::Body, debug_handler, extract::{ws::close_code::STATUS, Path, State}, http::{header::{self, COOKIE, SET_COOKIE}, HeaderMap, HeaderValue, Method, Response, StatusCode}, response::{self, IntoResponse, Json}, routing::{delete, get, post, put}, Router
};

use core::panic;
use std::{env, hash::{DefaultHasher, Hash, Hasher}, time::{Duration, SystemTime}};

use mongodb::{
    bson::{doc, oid::ObjectId}, options::{ClientOptions,ResolverConfig}, Client, Collection
};
use serde::{Serialize, Deserialize};

// for future additions
use futures::{TryStreamExt};
use std::sync::Arc;

//use tower::ServiceExt;
use std::time::{Instant};
use sha2::{Sha256,Digest};

#[derive(Debug, Serialize, Deserialize,Clone)]
struct UserContact{

    id:String,
    number:String

}
#[derive(Debug, Serialize, Deserialize,Clone)]
struct Message{
    message: String
}


#[derive(Debug, Serialize, Deserialize,Clone)]
struct Notification{
    r#type:String,
    to:UserContact,
    sms:Message
}


#[derive(Debug, Serialize, Deserialize,Clone)]
struct SimplifiedItems{
    item_name:String,
    quantity:i64,
    method_of_measure:String
}
#[derive(Debug, Serialize, Deserialize,Clone)]
struct Recipe{
    recipe_name:String,
    itemers:Vec<SimplifiedItems>
}


struct _Change{
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
struct Usero {
    
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>, // Optional ObjectId for _id
    username: String,
    password:String
}



#[derive(Debug, Serialize, Deserialize,Clone)]
struct Token{
    token: String

}

#[derive(Clone)]
struct AppState {
    client: Arc<Client>,
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}


fn signature_to_hex(bytes:&[u8])->String{
    bytes.iter().map(|b|format!("{:02x}",b)).collect()
}


fn create_token(value1:String,value2:String)->String{
    let mut hashest = Sha256::new();
    hashest.update(format!("{:?}{:?}",value1,value2));    
    let result = hashest.finalize();
    let changed_result = signature_to_hex(&result);
    return changed_result;

}



fn check_token(token:CookieJar)->bool 
    {
        println!("\nThe Cookies {:?}\n",token);

        if let Some(cookie) = token.get("Session_ID")
        {
            let value = cookie.value();
            println!("The value {}",value);
            return true
        }
        else 
        {
            println!("No session_ID");
            return false
                
        }
    }




#[tokio::main]
async fn main() {

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
    //get changed data
    .route("/data",get(pull_data)).with_state(state.clone())
    .route("/graph/{id}",get(pull_specific_data)).with_state(state.clone())

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


async fn check_user(State(state):State<AppState>,headers:HeaderMap)->Response<Body>{
    
    println!("\nHeader {:?}\n",headers);

    let sol = check_token(CookieJar::from_headers(&headers.clone()));
    if sol==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }
    
    let user_token = CookieJar::from_headers(&headers);
    let token = user_token
                                .get("Session_ID")
                                .expect("Some how something fucked up").value();

    let filter = doc! {"token":token};

    let user: Collection<User> = state.client.database("test").collection("users");
    let curser = user
    .find(filter,None)
    .await
    .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();
    let users: Vec<User> = curser
                                .try_collect()
                                .await
                                .map_err(|e| 
                                    {
                                      return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                                    })
                                .expect("test");
    println!("{:?}",users);

    return Json(json!({"Sucess":true})).into_response()

}

async fn change_user(headers:HeaderMap,State(state):State<AppState>, Json(payload): Json<serde_json::Value>)->Result<String,(StatusCode,String)>{

     let user: String=match payload.get("user_id") {
        Some(Value::String(x))=>{x.to_string()},
        _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))} 
    };
    let user_token = CookieJar::from_headers(&headers);
    let token = user_token
                                .get("Session_ID")
                                .expect("Some how something fucked up").value();


    let new_item = doc! {"$set":{"token":token}};
    let filter = doc! {"_id":user};
    let user: Collection<User> = state.client.database("test").collection("user");
    let _curser = user
        .update_one(filter,new_item,None)
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
async fn login(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)-> Response<Body>{
    
    println!("\nPayload {:?}\n",payload);
    
    println!("\nHeader {:?}\n",headers);
    
    let db:Collection<Usero> =state.client.database("test").collection("users");
    
    // Checks db
    // println!("\n DB: {:?} \n",db);

    let username:String = match payload.get("username"){
        Some(Value::String(x))=>{x.to_string()},
         _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    
    };


    let password:String = match payload.get("password"){
        Some(Value::String(x))=>{x.to_string()},
        _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };


    // println!("\n{:?},{:?}\n",username,password);
    let token =create_token(username.clone(), password.clone());

    let x =db.find(doc! {"username":username, "password":password}, None).await.unwrap();
    

    let users:Vec<Usero> =x
                                .try_collect()
                                .await
                                //.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x.kind)).into_response()});
                                .expect("error");
    println!("{:?}",users[0].id);
    


    if users.len()<1 {
        println!("Len of users are greater then 0{:?}",users.len()<1);
        return (StatusCode::NOT_FOUND,"Not Found".to_string()).into_response();
    };
    let expires_in = Duration::from_secs(7 * 24 * 60 * 60);
    let expires_at = SystemTime::now() + expires_in;

    
   let mut cookier = Cookie::new("Session_ID", "cookieSet");
        cookier.set_expires(Expiration::DateTime(expires_at.into()));
        cookier.set_secure(true);
        cookier.set_same_site(SameSite::None);
        //cookier.set_http_only(true);
        cookier.set_path("/");


    let mut cookier2 = Cookie::new("gsI", users[0].id.expect("nothing").to_string());
        cookier2.set_expires(Expiration::DateTime(expires_at.into()));
        cookier2.set_secure(true);
        cookier2.set_same_site(SameSite::None);
        cookier2.set_path("/");



    let new_item = doc! {"$set":{"token":token}};
    let filter = doc! {"_id":users[0].id};
    let user: Collection<User> = state.client.database("test").collection("users");
    let curser = user
        .update_one(filter,new_item,None)
        .await
        .map_err(|x|
            return (StatusCode::EXPECTATION_FAILED , format!("Failed to update logon {}", x)).into_response()
        );
    curser.ok();
    println!("{:?}",users[0].id.expect("nothing").to_string());




    
    return ([(axum::http::header::SET_COOKIE, cookier.to_string())],Json(json!({"user_id":users[0].id.expect("nothing").to_string()}))).into_response()
    
    

}
#[axum::debug_handler]
async fn show_cookies(jar: CookieJar) -> impl IntoResponse {
    let mut text = String::new();
    println!("\n CookieJar {:?} \n",jar);
    if jar.iter().count() == 0 {
        text.push_str("No cookies received.\n");
    } else {
        text.push_str("Cookies:\n");
        for cookie in jar.iter() {
            text.push_str(&format!("  {} = {}\n", cookie.name(), cookie.value()));
        }
    }

    println!("{}", text); // Also log to terminal
    (StatusCode::OK, text)
}









///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////






//Item function
#[axum::debug_handler]
async fn get_item(State(state):State<AppState>,headers:HeaderMap)->Response<Body>{
    println!("Headers{:?}",&headers.clone());
    let sol = check_token(CookieJar::from_headers(&headers.clone()));
    println!("Token exists {}",sol);
    if sol==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }
    let start = Instant::now();

    let item: Collection<Item> = state.client.database("test").collection("item");


    let curser = item
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x.kind)))
        .unwrap();

    
    let items:Vec<Item> = curser
                                .try_collect()
                                .await
                                .expect("Error making item");
    

    
    // Build the response with the cookie
    
    let duration = start.elapsed();
 
    println!("get_item took {:?} to complete",duration);
    
    


    return (Json(items)).into_response();
}


async fn specific_item()->Result<Json<Vec<Item>>,(StatusCode,String)>{
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



async fn insert_item(State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>

{
    let item: Collection<Document> = state
                                            .client
                                            .database("test")
                                            .collection("item");

    println!("payload: {:#?}",payload);
    
    let time = Instant::now();

    let item_name: String = match payload.get("name")
        {
            Some(Value::String(x))=>{x.to_string()},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
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

    let date:DateTime=  match payload.get("time") {
        Some(Value::Number(x))=>{match x.as_i64() {
         Some(x)=>{bson::DateTime::from_millis(x)},
         _ => {return Err((StatusCode::NOT_FOUND,"Wrong input".to_string()))}   
        }}
        _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };

    let newo_item :Document= doc! {"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date};
    
    
    

    
    item
        .insert_one(newo_item , None)
        .await
        .ok();
        
    
    let duration = time.elapsed();

    println!("{:?}",duration);

    return Ok(Json(json!({"Success":true})))

}

async fn change_item(State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    
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
    let _token = payload.get("token");
    let object_id = ObjectId::parse_str(item_id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let find_item = doc!{"_id":object_id};
    let new_item = doc! {"$set":{"_id":object_id,"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date}};


    
    println!("Prior to Collection call");    
    let itemo: Collection<Item> = state.client.database("test").collection("item");
    println!("After Collection call");

    let _cursor = itemo.update_one(find_item,new_item,None).await.ok();

   

    let difference = Some(quantity-old_quantity).unwrap();
    
    let change_line = doc! {"item":item_id.clone(),"change":difference,"price":unit_price,"date":date};

    let _ = state.client.database("test").collection("change").insert_one(change_line, None).await;


    return Ok(Json(json!({"Success":true})))

}

async fn delete_item(State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    let item_id: String= match payload.get("id")
        {
            Some(Value::String(x))=>{x.to_string()},
            _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    
    let object_id = ObjectId::parse_str(item_id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let filtered_document = doc! {"_id":object_id};

   

    let item: Collection<Item> = state.client.database("test").collection("item");

    

    let _cursor = item.delete_one(filtered_document, None).await;
    println!("Fulfilled functions");
    return Ok(Json(json!({"Success":true})))


}

async fn pull_data(State(state):State<AppState>)->Result<Json<Vec<Document>>,(StatusCode,String)>{
    let time = Instant::now();
    let data:Collection<Document> = state.client.database("test").collection("change");
    let curser = data
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create curser: {}", x.kind)))
        .unwrap();


    
    let items:Vec<Document> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x.kind))})?;
    let duration = time.elapsed();
    println!("pull_data took {:?}",duration);
    return Ok(Json(items))

}



async fn pull_specific_data(Path(id): Path<String>,State(state):State<AppState>)->Result<Json<Vec<Document>>,(StatusCode,String)>{
    
    let start = Instant::now();
    let _object_id = ObjectId::parse_str(id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let find_item = doc!{"item":id};
    println!("Object{:#?}",find_item);
    let data:Collection<Document> = state.client.database("test").collection("change");

    println!("Document {:#?}",data);
     let curser = data
        .find(find_item,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))).unwrap();


    let duration =start.elapsed();
    let items:Vec<Document> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x))})?;
    println!("Pulling specific data took {:#?}",duration);
    
    return Ok(Json(items))

}

//Recipe

async fn create_recipe(State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>
        {

            println!("{:#?}",payload);
            let steps:Vec<String> = match payload.get("steps") 
            {
                Some(Value::Array(x))=>{println!("{:#?}",x);let ab = x.iter().map(|f|f.to_string()).collect();ab},
                _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Cant find steps".to_string()))}
            };

            let cooktime:i64 = match payload.get("time_to_cook")
                {
                    Some(Value::String(s))=>{s.parse::<i64>().expect("Error converting String to i64")},
                    Some(Value::Number(n))=>{n.as_i64().expect("Error converting Number to i64")},
                    _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Cant find time to cook".to_string()))}
                };
            let description:String = match payload.get("description")
                {
                    Some(Value::String(s))=>{s.to_string()}
                    _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Cant find description".to_string()))}

                };
           
        
        let recipe_payload = doc! 
            {
                "recipe_name": match payload.get("recipe_name") 
                    {
                        Some(Value::String(x))=>{x.to_string()},
                        _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Cant find recipe_name".to_string()))}


                    },
                "ingredients":match payload.get("ingredients") 
                {
                    Some(Value::Array(x))=>{let ab:Vec<Document> =  x.iter().map(|f|
                                        doc! 
                                        {
                                            "item_name":match &f[0] 
                                                {
                                                    Value::String(x)=>{x.trim_matches('"').parse::<String>().expect("Error")}
                                                    _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong Type wasnt string as expected".to_string()))}
                                                },
                                            "quantity":match &f[1]
                                                {

                                                    Value::String(x)=> {x.parse::<i64>().unwrap()},
                                                    Value::Number(x) => {x.as_i64().unwrap()},
                                                    _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Error on Second Quantity check".to_string()))}
                        
                                                },
                                            "method_of_measure":match &f[2] 
                                                {
                                                    Value::String(x)=>{x.trim_matches('"').parse::<String>().expect("Error")}
                                                    _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong Type wasnt string as expected".to_string()))}
                                                },
                                        }).collect();
                                        
                                        ab
                                }
                    _ =>  {panic!("{:#?}", (StatusCode::NOT_FOUND,"Cant find vec".to_string()))},
                },
                "steps": steps,
                "time_to_cook":cooktime,
                "Description":description
                                            
            };
                    
        
        
        println!("{:#?}",recipe_payload);


        let data:Collection<Document> = state.client.database("test").collection("recipe");

        let _ = data.insert_one(recipe_payload, None).await.ok();
        


        return Ok(Json(json!({"Sucess":true})))


    }

async fn get_recipes(State(state):State<AppState>,headers:HeaderMap)->Result<Json<Vec<Document>>,(StatusCode,String)>{

    println!("Headers{:?}",&headers.clone());
    let sol = check_token(CookieJar::from_headers(&headers.clone()));
    println!("Token exists {}",sol);
    if sol==false {

        return Err((StatusCode::FORBIDDEN,"User isnt logged in".to_string()))
    }
    let data:Collection<Document> = state.client
                                            .database("test")
                                            .collection("recipe");
    let time =Instant::now();
    let curser = data
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create curser: {}", x.kind)))
        .unwrap();
    
    let end_time = time.elapsed();

    let items:Vec<Document> = curser
                                    .try_collect()
                                    .await
                                    .map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x.kind))})?;

    

    println!{"{:?}",end_time}

    return Ok(Json(items));
}



async fn send_notification(State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Response<Body>{

    let raw = payload.get("message").expect("Couldn't get message").to_string();
    let messageo = raw.replace("\\n", "\n");


    let phone_number = payload.get("phone_number").expect("Couldnt find phone_number").to_string();

    println!("message: {} test",messageo);
    let credentials = env::var("notification").expect("error getting notification key");
    let email = env::var("email").expect("Error finding email");
    
    println!("{:?}",format!("+{}",phone_number));


    let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
    let cliento = reqwest::Client::new();
    let res = cliento.post("https://api.notificationapi.com/rezku83gmbz3zgky2pptrtw1za/sender")
    .header("Authorization",format!("Basic {} ",encoded))
    .header("Content-Type", "application/json")
    .body(reqwest::Body::from(
        serde_json::to_string
        (
            &Notification
                {
                    r#type:"inventory_status".to_string(),
                    to:UserContact
                        {
                            id:email,
                            number:format!("+{}",phone_number)
                        },
                    sms:Message
                        {
                            message:messageo.trim_matches('"').to_string()
                        }
                }
        ).expect("Error converting to Json")))
    .send()
    .await
    .expect("error awaiting the response");
    println!("the status of the request is {:?}",res.status());
    return Json(json!({"Success":true})).into_response();
}
// async fn specific_recipe(Path(id): Path<String>)->Result<Json<Vec<Document>>,(StatusCode,String)>{





// }


// async fn delete_recipe(Path(id): Path<String>)->Result<Json<Vec<Document>>,(StatusCode,String)>{






// }



async fn test()->Result<Json<Value>,(StatusCode,String)>{

    return Ok(Json(json!({"Sucess":true})))



}


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////


//Fun functions

async fn handle_client()->Client{
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))
        .expect("Error on Client URI");

    let mut options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e.kind)))
        .expect("Error on Client options");
    options.min_pool_size = Some(2);
    options.max_pool_size = Some(10);
    options.server_selection_timeout = Some(std::time::Duration::from_secs(5));
    
    
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e.kind)))
        .expect("Error on client result");
    
    return client
}



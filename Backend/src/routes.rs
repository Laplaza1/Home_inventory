use axum_extra::extract::{CookieJar};

use bson::{DateTime, Decimal128, Document};
use base64::{engine::general_purpose, Engine as _};
use ::cookie::{Cookie, Expiration, SameSite};
use log::{*};



// use chrono::{Utc};
use serde_json::{
    Value,
    json
};


// use rand::{Rng};
use axum::{
    body::Body, debug_handler, extract::{ws::close_code::STATUS, Path, State}, http::{header::{self, COOKIE, SET_COOKIE}, HeaderMap, HeaderValue, Method, Response, StatusCode}, response::{self, IntoResponse, Json}, routing::{delete, get, post, put}, Router
};

use core::panic;
use std::{any::{type_name, type_name_of_val}, collections::HashMap, env, hash::{DefaultHasher, Hash, Hasher}, time::{Duration, SystemTime}};

use mongodb::{
    bson::{doc, oid::ObjectId}, options::{ClientOptions, ResolverConfig}, Client, Collection
};
use serde::{Serialize, Deserialize};

// for future additions
use futures::{StreamExt, TryStreamExt};
use std::sync::Arc;
use std::time::{Instant};
use sha2::{Sha256,Digest};
use hex;




#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct UserContact{

    pub id:String,
    pub number:String

}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Message{
    pub message: String
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Notification{
    pub r#type:String,
    pub to:UserContact,
    pub sms:Message
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SimplifiedItems{
    pub item_name:String,
    pub quantity:i64,
    pub method_of_measure:String
}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Recipe{
    pub recipe_name:String,
    pub itemers:Vec<SimplifiedItems>
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct UserInfo{
    pub user_ID:Option<ObjectId>,
    pub home:String,
    pub email:String,
    pub phone_number:String

}


pub struct _Change{
    pub user_name:String,
    pub item_id:Option<ObjectId>,
    pub old_quantity:i64,
    pub new_quantity:i64
}



#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Item {
    
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Optional ObjectId for _id
    pub item_name: String,
    pub category: Vec<String>,
    pub quantity: i64,
    pub method_measure:String,
    pub unit_price: Decimal128,
    pub date: DateTime,
    pub home:String
}

#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct MicroUsero {

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, 
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct User {
    
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Optional ObjectId for _id
    pub username: String,
    pub password:String,
    pub status: Option<i64>,
    pub title:Option<String>,
    pub token:Option<i32>, 
    
}




impl User {
    fn check(self)->Result<Self,String>{
        
        match self.status {
           Some(0|1) => {return Ok(self)},
            
            _=>{return Err("Failed to be 0|1".to_string())}

        }
    }
    
}

#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub enum AccessLevel{
    Creator,
    Admin,
    #[default] 
    User

}
#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct UseroInfo{
    
        pub user_id:Option<ObjectId>,
        pub access:AccessLevel,
        pub home:String,
        pub email:String,
        pub phone_number:String
}


#[derive(Debug, Serialize, Deserialize,Clone,Default)]
pub struct Pending{
    pub username:String,
    pub email:String,
    pub home:String,
    pub password:String,
    pub phone_number:String,
    pub reason:String

}



#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Token{
    pub token: String

}

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<Client>,
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
    //Creates hash here
    let mut hash = Sha256::new();

    //Inserts Values here
    hash.update(format!("{:?}{:?}",value1,value2));    

    //finalize hash here
    let result = hash.finalize();

    //converts to hex here
    let changed_result = signature_to_hex(&result);
    return changed_result;

}

fn pull_token(header:CookieJar,key:&str)->String{
    //pulls token from headers
    return header.get(key).expect("Error pulling token").value().to_string()

}
pub async fn find_home(token:String,state:AppState)->String{
    

    let start = Instant::now();

    
   
    let user_info:Collection<UseroInfo> = state.client.database("test").collection("user_info");
    let found_user_info:UseroInfo = match user_info
                                            .find_one(doc! {"user_id":
                                                                            match ObjectId::parse_str(
                                                                                                    token.as_str()
                                                                                                )
                                                                                                {
                                                                                                    Ok(x)=>{x},
                                                                                                    _=>{error!("Can't parse str");std::process::exit(1)}
                                                                                                
                                                                                                }
                                                                    }, None)
                                            .await
                                            {
                                                Ok(Some(x))=>{x},
                                                Err(error)=>{error!("{}",error);std::process::exit(1)},
                                                _=>{error!("No user info");std::process::exit(1)}
                                            };

    let duration = start.elapsed();
 
    info!("find_home took {:?} to complete",duration);
    
    return found_user_info.home

}




fn check_token(token:CookieJar,key:&str)->bool 
    {
        info!("\nThe Cookies {:?}\n",token);

        if let Some(cookie) = token.get(key)
        {
            let value = cookie.value();
            info!("The value {}",value);
            return true
        }
        else 
        {
            info!("No {}",key);
            return false
        }
    }





pub async fn check_item(State(state):State<AppState>)->HashMap<&'static str, u64>{



    let item_data:Collection<Item> = state.client.database("test").collection("item");
    let item_name = ["Meat","Spice","Vegetable","Fruit","Dairy","Cleaning","Animal"];
    let mut item_counter= HashMap::new();
    for i in item_name{

        //Counts documents by filters
        let data = item_data.count_documents(doc! {"category":doc! { "$elemMatch": { "$eq": i } }}, None).await.ok().expect("couldn't proerly find element");
        item_counter.insert(i, data);
    }

    return item_counter

}



//User functions

pub async fn create_user(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    
    let token = check_token(CookieJar::from_headers(&headers.clone()),"Session_ID");
    if token==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }


    let user_data:Collection<User> = state.clone().client.database("test").collection("users");
    let user_info:Collection<UseroInfo> =state.client.database("test").collection("user_info") ;

    info!{"Payload: {:?}",payload}

    let username = payload
                                .get("username")
                                .expect("error trying to find username")
                                .to_string().trim_matches('"').to_string();
    let password = payload
                                .get("password")
                                .expect("error trying to find password")
                               .to_string().trim_matches('"').to_string();

    let email = payload
                                    .get("email")
                                    .expect("Couldn't find email")
                                    .to_string().trim_matches('"').to_string();

    let phone_number = payload
                                    .get("phonenumber")
                                    .expect("Couln't find phone number")
                                    .to_string().trim_matches('"').to_string();
    let home = payload
                                    .get("home")
                                    .expect("Couln't find home")
                                    .to_string().trim_matches('"').to_string();

    



    let user:User =  User { id: None, username: username.clone(), password: password.clone(),..Default::default()};
    
    user_data.insert_one(&user, None).await.ok();
    let user_id:mongodb::Cursor<User> = user_data
                                            .find(doc!{"username":username.clone(),"password":password.clone()},None)
                                            .await
                                            .map_err(|x|info!("Failed to create client: {}", x.kind))
                                            .expect("error trying to collect");
                                             
    
    let convert_user_id:Vec<User>= user_id
                                        .try_collect()
                                        .await
                                        .expect("Test");

    
    
    info!("Created user {:?} ",convert_user_id);

    let user_info1:UseroInfo = UseroInfo{ 
                                            user_id: convert_user_id[0].id,
                                            access: AccessLevel::User,
                                            home: home,
                                            email:email.clone(),
                                            phone_number:phone_number.clone() 
                                        };
    
    
    user_info
        .insert_one(&user_info1, None)
        .await
        .ok();
    let found_user_info = user_info
                                                                    .find(doc! {"email":email,"phone_number":phone_number}, None)
                                                                    .await
                                                                    .map_err(|x|info!("Failed to create user info : {}", x.kind))
                                                                    .expect("error trying to collect");
    
    let vec_found_userinfo:Vec<UseroInfo> =found_user_info
                                                    .try_collect()
                                                    .await
                                                    .expect("Couldnt collect into UseroInfo");

    info!("Created user info: {:?}",vec_found_userinfo);


    let remove_pend:Collection<Pending> = state.client.database("test").collection("pending");

    remove_pend.delete_one(doc! {"username":username.clone(),"password":password}, None).await.ok();


    return Json(json!({"Sucess":true})).into_response()
 
}


pub async fn check_user(State(state):State<AppState>,headers:HeaderMap)->Response<Body>{
    
    info!("Header {:?}",headers);

    let token = check_token(CookieJar::from_headers(&headers.clone()),"Session_ID");
    if token==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }
    
    let user_token = CookieJar::from_headers(&headers);
    let token = user_token
                                .get("Session_ID")
                                .expect("Some how something fucked up").value();

    let filter = doc! {"token":token};

    let user: Collection<User> = state.client.database("test").collection("users");
    let curser = match user
                                    .find(filter,None)
                                    .await
                                    .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x)))
                                    {
                                        Ok(x)=>{x},
                                        Err(error)=>{error!("error: {:?}",error);std::process::exit(1)},
                                        _=>{error!("error couldn't create client");std::process::exit(1)}


                                    };
    let users: Vec<User> = curser
                                .try_collect()
                                .await
                                .map_err(|e| 
                                    {
                                      return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                                    })
                                .expect("test");
    info!("{:?}",users);

    return Json(json!({"Sucess":true})).into_response()

}

pub async fn change_user(headers:HeaderMap,State(state):State<AppState>, Json(payload): Json<serde_json::Value>)->Result<String,(StatusCode,String)>{

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
    let _curser = match user
        .update_one(filter,new_item,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))){

            Ok(x)=>{x},
            Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
            _=>{error!("Couldn't create collection of Users");std::process::exit(1)}
        };
    



    return Ok("If you're reading this then ya changed information on user".to_string())
}

pub async fn delete_user(headers:HeaderMap,State(state):State<AppState>,Path(id): Path<String>)->Response<Body>{

    let token = check_token(CookieJar::from_headers(&headers.clone()),"Session_ID");
    if token==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }

    let data:Collection<Document> = state.client.database("test").collection("users");
    let _ =data.delete_one(doc! {"_id":ObjectId::parse_str(id).ok()}, None).await;

    return Json(json!({"success":true})).into_response()

}





///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////




//login function
pub async fn login(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)-> impl IntoResponse{
    
    info!("Payload {:?}",payload);
    
    info!("Header {:?}",headers);
    
    let db:Collection<User> =state.client.database("test").collection("users");
    let user_info:Collection<UseroInfo> = state.client.database("test").collection("user_info");
    // Checks db
    

    let username:String = match payload.get("username"){
        Some(Value::String(x))=>{x.to_string()},
         _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    
    };


    let password:String = match payload.get("password"){
        Some(Value::String(x))=>{x.to_string()},
        _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };


    
    let token =create_token(username.clone(), password.clone());

    let x =match db.find_one(doc! {"username":username, "password":password}, None).await{
        Ok(x)=>{x},
        Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
        _=>{error!("Error:");std::process::exit(1)}
    }.expect("failed finding one user");
    let y = match user_info.find_one(doc! {"user_id":x.id}, None).await{
        Ok(x)=>{x},
        Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
        _=>{error!("Error: ");std::process::exit(1)}
    }.expect("failed finding one user info");

    let mut header = HeaderMap::new();
    
    info!("{:?}",x.id);
    


    
    let expires_in = Duration::from_secs(7 * 24 * 60 * 60);
    let expires_at = SystemTime::now() + expires_in;

    
   let mut cookier = Cookie::new("Session_ID", x.id.expect("error converting to string").to_string());
        cookier.set_expires(Expiration::DateTime(expires_at.into()));
        cookier.set_secure(true);
        cookier.set_same_site(SameSite::None);
        //cookier.set_http_only(true);
        cookier.set_path("/");
     header.append(SET_COOKIE, match cookier.to_string().parse(){
        Ok(x)=>{x},
        Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
        _=>{error!("Error handling header");std::process::exit(1)}
    });

    info!("Session_ID {:?}",cookier);  

    let mut home_cookie = Cookie::new("hwt", y.home.clone());
        home_cookie.set_expires(Expiration::DateTime(expires_at.into()));
        home_cookie.set_secure(true);
        home_cookie.set_same_site(SameSite::None);
        home_cookie.set_path("/");
     header.append(SET_COOKIE, match home_cookie
        .to_string()
        .parse()
        {
        Ok(x)=>{x},
        Err(error)=>{error!("error: {:?}",error);std::process::exit(1)},
        _=>{error!("Error: ");std::process::exit(1)}
        });

    let encoded = hex::encode(y.home.clone());

    info!("home_cookie to hex: {} ",encoded);

    
    info!("hwt {:?}",home_cookie);


    let mut cookier2 = Cookie::new("gsI", x.id.expect("nothing").to_string());
        cookier2.set_expires(Expiration::DateTime(expires_at.into()));
        cookier2.set_secure(true);
        cookier2.set_same_site(SameSite::None);
        cookier2.set_path("/");



    let new_item = doc! {"$set":{"token":x.id}};
    let filter = doc! {"_id":x.id};
    let user: Collection<User> = state.client.database("test").collection("users");
    let curser = user
        .update_one(filter,new_item,None)
        .await
        .map_err(|x|
            return (StatusCode::EXPECTATION_FAILED , format!("Failed to update logon {}", x)).into_response()
        );
    curser.ok();
    info!("{:?}",x.id.expect("nothing").to_string());
    
    info!("{:?}",header);
    



    


    return (StatusCode::OK,header,Json(json!({"user_id":x.id.expect("nothing").to_string()})))
    
    

}




///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////






//Item function
#[axum::debug_handler]
pub async fn get_item(State(state):State<AppState>,headers:HeaderMap)->Response<Body>{
    info!("Headers: {:?}",&headers.clone());
    let ab =pull_token(CookieJar::from_headers(&headers.clone()),"Session_ID");
    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let token = ab.len()>0;
    info!("Token exists: {}",token);
    if token==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }
    
    
                                            

    let start = Instant::now();

    let item: Collection<Item> = state.client.database("test").collection("item");

    //let x = find_home(ab, state).await;
    let curser =match item
        .find(doc! {"home":home},None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x.kind)))
        {
            Ok(x)=>{x},
            Err(error)=>{error!("error: {:?}",error);std::process::exit(1)},
            _=>{error!("Error get items");std::process::exit(1)}
        };

    
    let items:Vec<Item> = curser
                                .try_collect()
                                .await
                                .expect("Error making item");
    

    
    // Build the response with the cookie
    
    let duration = start.elapsed();
 
    info!("get_item took {:?} to complete",duration);
    
    


    return (Json(items)).into_response();
}


pub async fn specific_item()->Result<Json<Vec<Item>>,(StatusCode,String)>{
    let client_uri = env::var("MONGODB_URI")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Missing MONGODB_URI".to_string()))?;

    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse client options: {}", e)))?;
    let client = Client::with_options(options)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", e)))?;
    let item: Collection<Item> = client.database("test").collection("item");

    info!("Item var: {:#?}",item);

    

    let curser = match item
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))){
            Ok(x)=>{x},
            Err(error)=>{error!("error: {:?}",error);std::process::exit(1)},
            _=>{error!("Couldn't create collection of items");std::process::exit(1)}
        };

     let items:Vec<Item> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x))})?;
    

    return Ok(Json(items))
}



pub async fn insert_item(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>

{
    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let item: Collection<Document> = state
                                            .client
                                            .database("test")
                                            .collection("item");

    info!("payload: {:#?}",payload);
    
    let time = Instant::now();

    let item_name: String = match payload.get("name")
        {
            Some(Value::String(x))=>{x.to_string()},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    let category:Vec<String> = match payload.get("categories") {
        Some(Value::String(s))=>{let x = vec![s.to_string()];x},
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

    let newo_item :Document= doc! {"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date,"home":home};
    
    
    

    
    item
        .insert_one(newo_item , None)
        .await
        .ok();
        
    
    let duration = time.elapsed();

    info!("{:?}",duration);

    return Ok(Json(json!({"Success":true})))

}

pub async fn change_item(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{

    
    info!("Payload: {:#?}",payload);
    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let item_id: String=match payload.get("id") {
        Some(Value::String(x))=>{x.to_string()},
        _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))} 
    };
    let item_name: String = match payload.get("name")
        {
            Some(Value::String(x))=>{x.to_string()},
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };

    let category:Vec<String> = match payload.get("categories") {
        Some(Value::String(s))=>{let x=vec![s.to_string()];x},
        Some(Value::Array(s))=>{let arrayer:Vec<String>= s.iter().map(|x|x.to_string()).collect(); arrayer},
        _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}      
    };
    
    let quantity:  i64 = match payload.get("amount")
        {
            Some(Value::String(x))=>{x.parse::<i64>().expect("This shouldn't be wrong if posted through")},
            Some(Value::Number(x))=>{x.as_i64().expect("Should be right")}
            _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    
    //.and_then(|x|Some(x.as_i64())){

    let old_quantity:i64 = match payload.get("oldAmount") 
        {
           Some(Value::String(x))=>{x.parse::<i64>().expect("This shouldn't be wrong if posted through")},
           Some(Value::Number(x))=>{x.as_i64().expect("Should be right")}
           _=>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
        };
    
    //.and_then(|x|Some(x.as_i64())){

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
    info!("Method of measure: {:#?}",method_measure);
        //.and_then(|x|Some(x.to_string().parse::<f32>().ok())){
        
    let date:DateTime=  match payload.get("time") {
        Some(Value::Number(x))=>{match x.as_i64() {
         Some(x)=>{bson::DateTime::from_millis(x)},
         _ => {panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}   
        }}
        _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}
    };
    info!("{:#?}",item_id);
    let _token = payload.get("token");
    let object_id = ObjectId::parse_str(item_id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let find_item = doc!{"_id":object_id};
    let new_item = doc! {"$set":{"_id":object_id,"item_name":item_name,"category":category,"quantity":quantity,"method_measure":method_measure,"unit_price":unit_price,"date":date}};


    
    info!("Prior to Collection call");    
    let itemo: Collection<Item> = state.client.database("test").collection("item");
    info!("After Collection call");

    let _cursor = itemo.update_one(find_item,new_item,None).await.ok();

   

    let difference = match Some(quantity-old_quantity){
        Some(x)=>{x},
        _=>{error!("Error couldnt handle the calculation check values");std::process::exit(1)}
    };
    
    let change_line = doc! {"item":item_id.clone(),"change":difference,"price":unit_price,"date":date};

    let _ = state.client.database("test").collection("change").insert_one(change_line, None).await;


    return Ok(Json(json!({"Success":true})))

}

pub async fn delete_item(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>{
    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let item_id: String= match payload.get("id")
        {
            Some(Value::String(x))=>{x.to_string()},
            _ =>{panic!("{:#?}", (StatusCode::NOT_FOUND,"Wrong input".to_string()))}

        };
    
    let object_id = ObjectId::parse_str(item_id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let filtered_document = doc! {"_id":object_id};

   

    let item: Collection<Item> = state.client.database("test").collection("item");

    

    let _cursor = item.delete_one(filtered_document, None).await;
    info!("Fulfilled functions");
    return Ok(Json(json!({"Success":true})))


}

pub async fn pull_data(headers:HeaderMap,State(state):State<AppState>)->Result<Json<Vec<Document>>,(StatusCode,String)>{
    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let time = Instant::now();
    let data:Collection<Document> = state.client.database("test").collection("change");
    let curser = match data
        .find(None,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create curser: {}", x.kind)))
        {
            Ok(x)=>{x},
            Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
            _=>{error!("Error");std::process::exit(1)}
        };


    
    let items:Vec<Document> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x.kind))})?;
    let duration = time.elapsed();
    info!("pull_data took {:?}",duration);
    return Ok(Json(items))

}



pub async fn pull_specific_data(headers:HeaderMap,Path(id): Path<String>,State(state):State<AppState>)->Result<Json<Vec<Document>>,(StatusCode,String)>{
    
    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let start = Instant::now();
    let _object_id = ObjectId::parse_str(id.as_str()).map_err(|x|(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create client: {}", x))).ok();
    let find_item = doc!{"item":id};
    info!("Object{:#?}",find_item);
    let data:Collection<Document> = state.client.database("test").collection("change");

    info!("Document {:#?}",data);
     let curser =match  data
        .find(find_item,None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create client: {}", x))){
            Ok(x)=>{x},
            Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
            _=>{error!("Error creating client");std::process::exit(1)}
        };


    let duration =start.elapsed();
    let items:Vec<Document> = curser.try_collect().await.map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x))})?;
    info!("Pulling specific data took {:#?}",duration);
    
    return Ok(Json(items))

}

//Recipe

pub async fn create_recipe(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Result<Json<Value>,(StatusCode,String)>
        {
            let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
            info!("{:#?}",payload);
            let steps:Vec<String> = match payload.get("steps") 
            {
                Some(Value::Array(x))=>{info!("{:#?}",x);let ab = x.iter().map(|f|f.to_string()).collect();ab},
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

                                                    Value::String(x)=> {match x.parse::<i64>(){
                                                        Ok(x)=>{x},
                                                        Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
                                                        _=>{error!("Error: ");std::process::exit(1)}
                                                    }},
                                                    Value::Number(x) => {match x.as_i64(){
                                                        Some(x)=>{x},
                                                        _=>{error!("Error: ");std::process::exit(1)}
                                                    }},
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
                "Description":description,
                "home":home
                                            
            };
                    
        
        
        info!("{:#?}",recipe_payload);


        let data:Collection<Document> = state.client.database("test").collection("recipe");

        let _ = data.insert_one(recipe_payload, None).await.ok();
        


        return Ok(Json(json!({"Sucess":true})))


    }

pub async fn get_recipes(State(state):State<AppState>,headers:HeaderMap)->Result<Json<Vec<Document>>,(StatusCode,String)>{

    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    info!("Headers{:?}",&headers.clone());
    let token = check_token(CookieJar::from_headers(&headers.clone()),"Session_ID");
    info!("Token exists {}",token);
    if token==false {

        return Err((StatusCode::FORBIDDEN,"User isnt logged in".to_string()))
    }
    let data:Collection<Document> = state.client
                                            .database("test")
                                            .collection("recipe");
    let time =Instant::now();
    let curser = match data
        .find(doc! {"home":home},None)
        .await
        .map_err(|x|(StatusCode::EXPECTATION_FAILED , format!("Failed to create curser: {}", x.kind)))
        {
            Ok(x)=>{x},
            Err(error)=>{error!("Error: {:?}",error);std::process::exit(1)},
            _=>{error!("Error: creating document");std::process::exit(1)}
        };
    
    let end_time = time.elapsed();

    let items:Vec<Document> = curser
                                    .try_collect()
                                    .await
                                    .map_err(|x|{(StatusCode::EXPECTATION_FAILED,format!("Error: {} happend when creating item",x.kind))})?;

    

    info!{"{:?}",end_time}

    return Ok(Json(items));
}



pub async fn send_notification(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Response<Body>{

    let home =pull_token(CookieJar::from_headers(&headers.clone()), "hwt");
    let raw = payload.get("message").expect("Couldn't get message").to_string();
    let messageo = raw.replace("\\n", "\n");


    let raw_phone_number = payload.get("phone_number").expect("Couldnt find phone_number").to_string();
    let phone_number = raw_phone_number.trim_matches('"').to_string();


    info!("message: {} test",messageo);
    let credentials = env::var("notification").expect("error getting notification key");
    let email = env::var("email").expect("Error finding email");
    
    info!("{:?}",phone_number);


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
                            number:phone_number
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
    info!("the status of the request is {:?}",res.status());
    return Json(json!({"Success":true})).into_response();
}


// pub async fn pull_admin_data(State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Response<Body>{





// }


pub async fn create_pending(headers:HeaderMap,State(state):State<AppState>,Json(payload): Json<serde_json::Value>)->Response<Body>{
    
    let data:Collection<Pending> = state.client.database("test").collection("pending");

    info!("payload {:?} ",payload);

    let username:String = match payload.get("username").expect("Couldnt find username"){
        Value::String(x)=>{x.to_string()},
        _=>{return StatusCode::NOT_FOUND.into_response()}
    };

    let email:String = match payload.get("email").expect("Couldnt find Email"){
        Value::String(x)=>{x.to_string()},
        _=>{return StatusCode::NOT_FOUND.into_response()}
    };
    let password = match payload.get("password").expect("couldnt find Password"){
        Value::String(x)=>{x.to_string()},
        _=>{return StatusCode::NOT_FOUND.into_response()}
    };
    let home = match payload.get("home").expect("couldnt find home"){
        Value::String(x)=>{x.to_string()},
        _=>{return StatusCode::NOT_FOUND.into_response()}
    };
    let phone_number = match payload.get("phoneNumber").expect("couldnt find phone number"){
        Value::String(x)=>{x.to_string()},
        _=>{return StatusCode::NOT_FOUND.into_response()}
    };
    if phone_number.len()!=12{
        info!("phone # is {} long not 12",phone_number.len());
        return StatusCode::NOT_FOUND.into_response()
    }
    
    let reason = match payload.get("reason").expect("couldnt find reason"){
        Value::String(x)=>{x.to_string()},
        _=>{return StatusCode::NOT_FOUND.into_response()}
    };



    let body = Pending{username:username.clone(),email:email.clone(),home:home.clone(),password:password.clone(),phone_number:phone_number.clone(),reason:reason};

    let pendingo:Vec<Document> = vec![
        doc!{"username":username.clone()},
        doc!{"email":email.clone()},
        doc!{"home":home.clone()},
        doc!{"phone_number":phone_number.clone()},

    ]; 
    let pend_filter = doc!{"$or":pendingo};
    
    let zy = data.count_documents(pend_filter,None).await.expect("Pending user with that info already exists");
    
    if zy>0
        {

            info!("{:?} Pending User documents with similar info exist ",zy);
            return (StatusCode::FOUND,Json(json!({"Failed":"Already exists"}))).into_response()

        }
    let user_info_or_conditions:Vec<Document> = vec![
        doc! {"email":body.email.clone()},
        doc! {"phone_number":body.phone_number.clone()},
        doc!{"home":home.clone()},
    ]; 
    

    let filter =doc! {"$or":user_info_or_conditions};


    let user_info_state:Collection<UseroInfo> =  state.client.database("test").collection("user_info");
    
    let xy = user_info_state.count_documents(filter, None).await.expect("The email or phone already exists");
    
        if xy>0 {

            info!("{:?} UserInfo documents with similar info exist",xy);
            return (StatusCode::FOUND,Json(json!({"Failed":"Already exists"}))).into_response()
        }

    


    let userexisto:Collection<User>= state.client.database("test").collection("users");
    let yx = userexisto.count_documents(doc! {"username":username.clone()}, None).await.expect("Username already exists");
    
    if yx>0{

        info!("{:?} User documents with similar info exist ",yx);
        return (StatusCode::FOUND,Json(json!({"Failed":"Already exists"}))).into_response()

    }



    return Json(json!({"Sucess":data.insert_one(body, None).await.is_ok()})).into_response()





}



// pub async fn specific_recipe(Path(id): Path<String>)->Result<Json<Vec<Document>>,(StatusCode,String)>{





// }


// pub async fn delete_recipe(Path(id): Path<String>)->Result<Json<Vec<Document>>,(StatusCode,String)>{






// }


pub async fn general_data(headers:HeaderMap,State(state):State<AppState>)->Response<Body>{
    let token = check_token(CookieJar::from_headers(&headers.clone()),"Session_ID");
    if token==false {

        return (StatusCode::FORBIDDEN,"User isnt logged in").into_response()
    }

    let data:Collection<User> = state.client
                                        .database("test")
                                        .collection("users");

    let data1:Collection<MicroUsero> = state.client
                                        .database("test")
                                        .collection("users");

    let users = data1
                                    .find(None,None)
                                    .await
                                    .expect("error");
    let vec_users:Vec<MicroUsero>=users
                                    .try_collect::<Vec<MicroUsero>>()
                                    .await
                                    .ok()
                                    .expect("error");                             

    let data_count= data
                                    .count_documents(None, None)
                                    .await
                                    .expect("Couldnt convert into Vec ");
    let homes:Collection<UseroInfo> =state.client
                                            .database("test")
                                            .collection("user_info");

    let home_count =homes
                                .distinct("home", None,None)
                                .await
                                .ok()
                                .expect("error finding homes")
                                .len();

                                
    
    
    let pending:Collection<Pending> = state.client.database("test").collection("pending");

    let pending_users = pending.find(None,None).await.ok().expect("Error");
    

    let pending_data:Vec<Pending> =pending_users
                                        .try_collect::<Vec<Pending>>()
                                        .await
                                        .expect("error converting");                            
let item_type_count = check_item(axum::extract::State(state.clone())).await;

    return Json(json!({
                        "Number_of_users":data_count,
                        "Number_of_homes":home_count,
                        "Item_count":item_type_count,
                        "Pending_users":pending_data,
                        "users":vec_users
                        
                        })).into_response();

}



pub async fn test()->Result<Json<Value>,(StatusCode,String)>{

    return Ok(Json(json!({"Sucess":true})))



}












#[axum::debug_handler]
pub async fn show_cookies(jar: CookieJar) -> impl IntoResponse {
    let mut text = String::new();
    info!("\n CookieJar {:?} \n",jar);
    if jar.iter().count() == 0 {
        text.push_str("No cookies received.\n");
    } else {
        text.push_str("Cookies:\n");
        for cookie in jar.iter() {
            text.push_str(&format!("  {} = {}\n", cookie.name(), cookie.value()));
        }
    }

    info!("{}", text); // Also log to terminal
    (StatusCode::OK, text)
}

pub async fn handle_client()->Client{
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
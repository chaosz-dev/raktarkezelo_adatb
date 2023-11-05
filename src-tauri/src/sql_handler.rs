extern crate mysql;
extern crate log;
extern crate env_logger;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::Deserialize;
use log::{debug, error, info, warn};
use mysql::prelude::*;
use mysql::*;

type MySqlResult = Result<mysql::PooledConn, Error>;

#[derive(Debug)]
pub struct User {
  username: String,
  email: String,
  password: String,
  role: bool,
}

#[derive(Debug, Deserialize)]
pub struct Product {
  name: String,
  unit_price: u64,
  description: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchProduct {
  name: String,
  amount: u64,
  address: String,
}

impl Serialize for Product {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Product", 3)?;
        state.serialize_field("field1", &self.name)?;
        state.serialize_field("field2", &self.unit_price)?;
        state.serialize_field("field3", &self.description)?;
        state.end()
    }
}

impl Serialize for SearchProduct {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 5 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("SearchProduct", 5)?;
        state.serialize_field("field1", &self.name)?;
        state.serialize_field("field2", &self.amount)?;
        state.serialize_field("field3", &self.address)?;
        state.end()
    }
}

pub fn sql_conn() -> MySqlResult  {
  println!("Starting sql connection...");
  let pool = Pool::new("mysql://root@localhost:3306/adatb")?;
  println!("Succesful connection.");
  pool.get_conn()
}

#[tauri::command]
pub async fn register_user(username: String, email: String, password: String) -> bool {
  println!("Start!");
  let mut conn = sql_conn().unwrap();

  if username.is_empty()  {
    return false;
  }
  if email.is_empty() || !email.contains("@")  {
    return false;
  }
  if password.is_empty() {
    return false;
  }

  println!("Starting slq querry...");
  let users = conn.query_map("SELECT * FROM user;", |(username, email, password, role)| {
      User { username, email, password, role }
    },
  );
  println!("Succes!");

  for user in users.as_ref().unwrap() {
    if email == user.email {
      return false;
    }
  }

  let insert = conn.exec_drop(
    r"INSERT INTO user (name, email, password) VALUES (?, ?, ?);",
    (&username, &email, &password)
  ).expect("Failed to insert data");
  println!("{:?}",insert);

  true
}

#[tauri::command]
pub async fn login(username: String, password: String) -> bool {
  let mut conn = sql_conn().unwrap();

  if username.is_empty() {
    return false;
  }

  if password.is_empty() {
    return false;
  }

  let mut query = String::from("SELECT * FROM user WHERE user.name LIKE ';");
  query.push_str(&(username + "'"));
  println!("Starting slq querry...");
  let users = conn.query_map(query, |(email, username, password, role)| {
      User { username, email, password, role }
    },
  );
  println!("Succes!");

  for user in users.as_ref().unwrap() {
    println!("{}", user.username);
  }

  true
}

#[tauri::command]
pub async fn products() -> Vec<Product> {
  let mut conn = sql_conn().unwrap();

  let products = conn.query_map("SELECT * FROM product;", |(name, unit_price, description)| {
    Product {name, unit_price, description}
  }).unwrap();

  products
}


#[tauri::command]
pub async fn search_product(name: String) -> Vec<SearchProduct> {
  let mut conn = sql_conn().unwrap();

  let query = format!("
  SELECT product.name, contains.amount, storageSite.address
  FROM product, contains, storageSite
  WHERE contains.product_name = product.name
  AND contains.storage_id = storageSite.id
  AND product.name LIKE '{}';", name);
  let products = conn.query_map(query, |(name, amount, address)| {
    SearchProduct {name, amount, address}
  }).unwrap();

  products
}

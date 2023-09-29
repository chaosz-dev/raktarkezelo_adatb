extern crate mysql;
use mysql::*;

type SqlPool = Result<mysql::Pool, Error>;

pub fn sql_setup() -> SqlPool  {
  let url = "mysql://root:@localgost:3307/"; // still needs db name after 3307/
  mysql::Pool::new(url)
}
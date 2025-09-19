use sqlx::{Pool, Sqlite};

pub type Db = Pool<Sqlite>;

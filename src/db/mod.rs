use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub mod model;
pub mod repository;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConn = PooledConnection<PgPool>;

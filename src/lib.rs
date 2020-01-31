#[macro_use]
extern crate diesel;

#[macro_use]
extern crate juniper;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::serialize::{self, ToSql};
use diesel::sql_types::SmallInt;
use std::io::Write;
use wundergraph::query_builder::types::{HasMany, HasOne, WundergraphValue};

pub mod generated;
// mod schema;

#[derive(
    Debug, Copy, Clone, AsExpression, FromSqlRow, GraphQLEnum, WundergraphValue, Eq, PartialEq, Hash,
)]
#[sql_type = "SmallInt"]
pub enum Episode {
    NEWHOPE = 1,
    EMPIRE = 2,
    JEDI = 3,
}

impl<DB> ToSql<SmallInt, DB> for Episode
where
    DB: Backend,
    i16: ToSql<SmallInt, DB>,
{
    fn to_sql<W: Write>(&self, out: &mut serialize::Output<'_, W, DB>) -> serialize::Result {
        (*self as i16).to_sql(out)
    }
}

impl<DB> FromSql<SmallInt, DB> for Episode
where
    DB: Backend,
    i16: FromSql<SmallInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        let value = i16::from_sql(bytes)?;
        Ok(match value {
            1 => Episode::NEWHOPE,
            2 => Episode::EMPIRE,
            3 => Episode::JEDI,
            _ => unreachable!(),
        })
    }
}

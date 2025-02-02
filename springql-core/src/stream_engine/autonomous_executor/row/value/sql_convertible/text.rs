// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use crate::{
    error::Result,
    stream_engine::{
        autonomous_executor::row::value::sql_value::nn_sql_value::NnSqlValue,
        time::timestamp::Timestamp,
    },
};

use super::SqlConvertible;

impl SqlConvertible for String {
    fn into_sql_value(self) -> NnSqlValue {
        NnSqlValue::Text(self)
    }

    fn try_from_string(v: &str) -> Result<Self> {
        Ok(v.to_string())
    }

    fn try_from_timestamp(v: &Timestamp) -> Result<Self> {
        Ok(v.to_string())
    }
}

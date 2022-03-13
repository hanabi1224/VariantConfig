#![allow(clippy::unit_arg)]

use serde::{Deserialize, Serialize};
use variant_config::{hashbrown::HashMap, *};
use vercel_lambda::{error::VercelError, *};

pub mod convert;
pub mod ip;

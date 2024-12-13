use super::DatabaseError;
use crate::models::ids::base62_impl::to_base62;
use crate::models::ids::random_base62_rng;
// use crate::models::ids::{random_base62_rng, random_base62_range};
use censor::Censor;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use sqlx::sqlx_macros::Type;

const ID_RETRY_COUNT: usize = 20;

macro_rules! generate_ids {
    ($vis:vis $function_name:ident, $return_type:ty, $id_length:expr, $select_stmnt:literal, $id_function:expr) => {
        $vis async fn $function_name(
            con: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        ) -> Result<$return_type, DatabaseError> {
            let mut rng = ChaCha20Rng::from_entropy();
            let length = $id_length;
            let mut id = random_base62_rng(&mut rng, length);
            let mut retry_count = 0;
            let censor = Censor::Standard + Censor::Sex;

            // Check if ID is unique
            loop {
                let results = sqlx::query!($select_stmnt, id as i64)
                    .fetch_one(&mut **con)
                    .await?;

                if results.exists.unwrap_or(true) || censor.check(&*to_base62(id)) {
                    id = random_base62_rng(&mut rng, length);
                } else {
                    break;
                }

                retry_count += 1;
                if retry_count > ID_RETRY_COUNT {
                    return Err(DatabaseError::RandomId);
                }
            }

            Ok($id_function(id as i64))
        }
    };
}

// macro_rules! generate_bulk_ids {
//     ($vis:vis $function_name:ident, $return_type:ty, $select_stmnt:literal, $id_function:expr) => {
//         $vis async fn $function_name(
//             count: usize,
//             con: &mut sqlx::Transaction<'_, sqlx::Postgres>,
//         ) -> Result<Vec<$return_type>, DatabaseError> {
//             let mut rng = rand::thread_rng();
//             let mut retry_count = 0;

//             // Check if ID is unique
//             loop {
//                 let base = random_base62_range(&mut rng, 1, 10) as i64;
//                 let ids = (0..count).map(|x| base + x as i64).collect::<Vec<_>>();

//                 let results = sqlx::query!($select_stmnt, &ids)
//                     .fetch_one(&mut **con)
//                     .await?;

//                 if !results.exists.unwrap_or(true) {
//                     return Ok(ids.into_iter().map(|x| $id_function(x)).collect());
//                 }

//                 retry_count += 1;
//                 if retry_count > ID_RETRY_COUNT {
//                     return Err(DatabaseError::RandomId);
//                 }
//             }
//         }
//     };
// }

generate_ids!(
    pub generate_user_id,
    UserId,
    8,
    "SELECT EXISTS(SELECT 1 FROM account.users WHERE id=$1)",
    UserId
);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Type, Hash, Serialize, Deserialize,
)]
#[sqlx(transparent)]
pub struct UserId(pub i64);

use crate::models::ids;

impl From<ids::UserId> for UserId {
    fn from(id: ids::UserId) -> Self {
        UserId(id.0 as i64)
    }
}

impl From<UserId> for ids::UserId {
    fn from(id: UserId) -> Self {
        ids::UserId(id.0 as u64)
    }
}

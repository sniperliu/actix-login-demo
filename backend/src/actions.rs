use diesel::prelude::*;

use crate::models;

pub fn find_user_by_id(
    uid: i64,
    conn: &MysqlConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn insert_new_user(
    new_user: models::NewUser,
    conn: &MysqlConnection,
) -> Result<models::NewUser, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)?;

    Ok(new_user)
}

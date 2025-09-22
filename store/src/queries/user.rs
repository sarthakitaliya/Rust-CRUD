use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    models::user::{NewUser, User},
    schema::users::dsl::*,
    store::Store,
};

impl Store {
    pub fn create_user(&mut self, user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users)
            .values(&user)
            .get_result(&mut self.conn)
    }

    pub fn get_user_by_id(&mut self, uid: Uuid) -> QueryResult<Option<User>> {
        users.find(uid).first::<User>(&mut self.conn).optional()
    }
}

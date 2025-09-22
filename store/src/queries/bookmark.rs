use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    models::bookmark::{Bookmark, NewBookmark},
    schema::bookmark::dsl::*,
    store::Store,
};

impl Store {
    pub fn create_bookmark(&mut self, new: NewBookmark) -> QueryResult<Bookmark> {
        diesel::insert_into(bookmark)
            .values(&new)
            .get_result(&mut self.conn)
    }

    pub fn list_bookmarks(&mut self, uid: Uuid, limit_n: i64) -> QueryResult<Vec<Bookmark>> {
        bookmark
            .filter(user_id.eq(uid))
            .limit(limit_n)
            .load(&mut self.conn)
    }

    pub fn set_fav(&mut self, bid: Uuid, fav: bool) -> QueryResult<Bookmark> {
        diesel::update(bookmark.find(bid))
            .set(is_favorite.eq(fav))
            .get_result(&mut self.conn)
    }

    pub fn delete_bookmark(&mut self, bid: Uuid) -> QueryResult<Bookmark> {
        diesel::delete(bookmark.find(bid)).get_result(&mut self.conn)
    }
}

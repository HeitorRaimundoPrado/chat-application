use crate::schema::room;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = room)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RoomModel {
    id: i32,
    title: String,
    owner_id: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = room)]
pub struct InsertRoomModel {
    pub title: String,
    pub owner_id: String,
}

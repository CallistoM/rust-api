use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use super::schema::olc_internal;
use super::schema::olc_external;
use super::schema::olc_gps;
use chrono::NaiveDateTime;
use chrono::prelude::*;

#[table_name = "olc_internal"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct OlcIntern {
    pub id: Option<i32>,
    pub value: i32,
    pub date: Option<NaiveDateTime>,
}

#[table_name = "olc_external"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct OlcExtern {
    pub id: Option<i32>,
    pub value: i32,
    pub date: Option<NaiveDateTime>,
}

#[table_name = "olc_gps"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct GPS {
    pub id: Option<i32>,
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub accuracy: i32,
    pub satellites: i32,
}


impl OlcIntern {
    pub fn create(olc_v: OlcIntern, connection: &MysqlConnection) -> OlcIntern {
        diesel::insert_into(olc_internal::table)
            .values(&olc_v)
            .execute(connection)
            .expect("Error creating new olc!");

        olc_internal::table.order(olc_internal::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<OlcIntern> {
        olc_internal::table.order(olc_internal::date).load::<OlcIntern>(connection).unwrap()
    }

    pub fn update(id: i32, olc_v: OlcIntern, connection: &MysqlConnection) -> bool {
        diesel::update(olc_internal::table.find(olc_v.id)).set(olc_internal::id.eq(id)).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(olc_internal::table.find(id)).execute(connection).is_ok()
    }
}

impl OlcExtern {
    pub fn create(mut olc_v: OlcExtern, connection: &MysqlConnection) -> OlcExtern {

        olc_v.date = Some(Utc::now().naive_utc());

        diesel::insert_into(olc_external::table)
            .values(&olc_v)
            .execute(connection)
            .expect("Error creating new olc!");

        olc_external::table.order(olc_external::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<OlcExtern> {
        olc_external::table.order(olc_external::date).load::<OlcExtern>(connection).unwrap()
    }

    pub fn update(id: i32, olc_v: OlcExtern, connection: &MysqlConnection) -> bool {
        diesel::update(olc_external::table.find(olc_v.id)).set(olc_external::id.eq(id)).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(olc_external::table.find(id)).execute(connection).is_ok()
    }
}

impl GPS {
    pub fn create(olc_v: GPS, connection: &MysqlConnection) -> GPS {
        diesel::insert_into(olc_gps::table)
            .values(&olc_v)
            .execute(connection)
            .expect("Error creating new olc!");

        olc_gps::table.order(olc_gps::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<GPS> {
        olc_gps::table.order(olc_gps::id).load::<GPS>(connection).unwrap()
    }

    pub fn update(id: i32, olc_v: GPS, connection: &MysqlConnection) -> bool {
        diesel::update(olc_gps::table.find(olc_v.id)).set(olc_gps::id.eq(id)).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(olc_gps::table.find(id)).execute(connection).is_ok()
    }
}
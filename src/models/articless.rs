use diesel::prelude::*;

use super::super::schema::webarticle;
#[derive(Queryable, Serialize, Debug, Deserialize)]
pub struct Articles {
    pub id: i64,
    pub title: String,
    pub content: String,
    // pub created_time: DateTime<UTC>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name="webarticle"]
pub struct NewArticles<'a> {
    pub title: &'a str,
    pub content: &'a str,
    // pub created_time: DateTime<UTC>,
}

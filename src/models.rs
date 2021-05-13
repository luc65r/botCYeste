use super::schema::amimir_urls;

#[derive(Queryable)]
pub struct AmimirUrl {
    pub id: i32,
    pub url: String,
}

#[derive(Insertable)]
#[table_name="amimir_urls"]
pub struct NewAmimirUrl {
    pub url: String,
}

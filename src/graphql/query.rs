use async_graphql::*;
pub struct Query;

#[Object]
impl Query {
    async fn get_preferences(&self) -> Vec<String> {
        vec!["lorem".to_owned(), "ipsum".to_string()]
    }
}
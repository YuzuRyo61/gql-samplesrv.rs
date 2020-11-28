use juniper::{GraphQLObject, RootNode, FieldResult};

#[derive(GraphQLObject)]
#[graphql(description = "Test query")]
struct Hello {
    hello: String
}

pub struct Query;
pub struct Mutation;

#[juniper::object]
impl Query {
    fn hello() -> FieldResult<Hello> {
        Ok(Hello {
            hello: "world".to_owned()
        })
    }
}

#[juniper::object]
impl Mutation {

}


pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}

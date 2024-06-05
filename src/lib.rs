use borsh::BorshSchema;

#[derive(BorshSchema)]
enum A {
    UnitVariant,
}

use schema_full_path_crate::A as AForeign;

#[derive(BorshSchema)]
struct B {
    x: A,
    y: AForeign,

}



#[cfg(test)]
mod tests {
    use borsh::schema::BorshSchemaContainer;

    use super::*;

    #[test]
    fn it_works_question_mark() {
        let container = BorshSchemaContainer::for_type::<B>();
        println!("{:#?}", container);
    }
}


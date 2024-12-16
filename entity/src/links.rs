use sea_orm::{prelude::Expr, sea_query::IntoCondition, Linked, RelationTrait};

use crate::{group, user, user_group};

pub struct UserToGroup;

impl Linked for UserToGroup {
    type FromEntity = user::Entity;
    type ToEntity = group::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            user_group::Relation::User.def().rev(),
            user_group::Relation::Group.def(),
        ]
    }
}
pub struct UserToGroupAdmin;

impl Linked for UserToGroupAdmin {
    type FromEntity = user::Entity;
    type ToEntity = group::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        //Todo: filter Admin only
        vec![
            user_group::Relation::User
                .def()
                .on_condition(|_left, right| {
                    Expr::col((right, user_group::Column::Admin))
                        .eq(true)
                        .into_condition()
                })
                .rev(),
            user_group::Relation::Group.def(),
        ]
    }
}

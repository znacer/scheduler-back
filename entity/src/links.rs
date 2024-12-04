use sea_orm::{Linked, RelationTrait};

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

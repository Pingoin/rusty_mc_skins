use api::{user_is_member, Group, User};
use dioxus::prelude::*;

#[component]
pub fn UserGroupConnection(
    group:Group,
    user:User,
    show_group:bool,
)-> Element{
    let user_id=user.clone().id.clone();
    let group_id=group.clone().id.clone();
    let is_member=use_resource(async move||{
        user_is_member(user_id,group_id).await.unwrap_or_default()
    });


    

    rsx!{

    }
}
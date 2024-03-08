#![allow(warnings)]

use sea_orm;
use std::fmt;

use ::entity::users::{self, Column, Entity as Users, Model as UsersModel};
use sea_orm::sea_query::OnConflict;
use sea_orm::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetAllNationsParams {
    pub is_npc: Option<bool>,
}
pub struct UserQuery;
impl UserQuery {}

pub struct UserMutation;

#[derive(Debug, Clone, Deserialize)]
pub struct Auth0UserPart {
    pub email: String,
    pub email_verified: bool,
    pub auth0_sub: String,
}

impl UserMutation {
    pub async fn insert_or_return_user(
        db: &DbConn,
        partial_user: Auth0UserPart,
    ) -> Result<UsersModel, DbErr> {
        let user = users::ActiveModel {
            email: Set(partial_user.email.to_owned()),
            email_verified: Set(partial_user.email_verified.to_owned()),
            auth0_sub: Set(partial_user.auth0_sub.to_owned()),
            ..Default::default()
        };

        let result = user.clone().insert(db).await;

        match result {
            Ok(u) => {
                println!("{u:#?}");
                Ok(u.try_into_model().unwrap() as UsersModel)
            }
            Err(error) => {
                let user = Users::find()
                    .filter(users::Column::Auth0Sub.eq(&partial_user.auth0_sub))
                    .one(db)
                    .await?;

                Ok(user.unwrap() as UsersModel)
            }
        }
    }
}

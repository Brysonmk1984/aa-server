#![allow(warnings)]
pub use sea_orm;

use ::entity::armies::{self, Entity as Armies, Model};
use ::entity::nation_armies::{self, Entity as NationArmies, Model as NationArmiesModel};
use ::entity::nations::{self, Entity as Nations, Model as NationsModel};
use ::entity::users::{self, Column, Entity as Users, Model as UsersModel};
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use serde::Deserialize;

pub struct Query;

impl Query {
    pub async fn find_army_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find_by_id(id).one(db).await
    }

    pub async fn get_all_armies(
        db: &DbConn,
    ) -> Result<Vec<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find().all(db).await
    }

    pub async fn get_nation_with_nation_armies(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<(NationsModel, Vec<NationArmiesModel>)>, DbErr> {
        let result = Nations::find_by_id(id)
            .find_with_related(NationArmies)
            .all(db)
            .await?;

        Ok(result)
    }

    pub async fn get_nation_with_nation_armies_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<(NationsModel, Vec<NationArmiesModel>), DbErr> {
        let nation = Nations::find()
            .filter(nations::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        if nation.is_none() {
            panic!("No nation for user {}", user_id)
        }

        let nation_id = &nation.clone().unwrap().id;
        let nation_armies = NationArmies::find()
            .filter(nation_armies::Column::NationId.eq(*nation_id))
            .all(db)
            .await;

        match nation_armies {
            Ok(n_armies) => Ok((nation.unwrap(), n_armies)),
            Err(_) => Ok((nation.unwrap(), vec![])),
        }
    }
}

pub struct Mutation;

#[derive(Debug, Clone, Deserialize)]
pub struct Auth0UserPart {
    pub email: String,
    pub email_verified: bool,
    pub auth0_sub: String,
}

impl Mutation {
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

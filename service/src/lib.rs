// mod mutation;
// mod query;

// pub use mutation::*;
// pub use query::*;

pub use sea_orm;

use ::entity::armies::Entity as Armies;
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_army_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find_by_id(id).one(db).await
    }

    // pub async fn find_posts_in_page(
    //     db: &DbConn,
    //     page: u64,
    //     posts_per_page: u64,
    // ) -> Result<(Vec<post::Model>, u64), DbErr> {
    //     // Setup paginator
    //     let paginator = Post::find()
    //         .order_by_asc(post::Column::Id)
    //         .paginate(db, posts_per_page);
    //     let num_pages = paginator.num_pages().await?;

    //     // Fetch paginated posts
    //     paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    // }
}

use crate::utils::constant::APPCONFIG;
use anyhow::{Ok, Result};
use sqlx::{PgConnection, PgPool, Postgres};

use super::{recipe_info::RecipeInfo, recipe_scores::RecipeScores};

async fn create_connection_pool() -> Result<PgConnection> {
    let conf = &APPCONFIG.database;
    let url = format!(
        "{}://{}:{}@{}:{}/{}",
        conf.db_type, conf.username, conf.password, conf.url, conf.port, conf.database
    );
    //如果有数据库其他配置需要修改
    let pool = PgPool::connect(&url).await?;
    Ok(pool.acquire().await?.detach())
}

pub async fn get_recipe(taste: String, limit: i32) -> Result<Vec<RecipeInfo>> {
    let mut conn = create_connection_pool().await?;
    let res = sqlx::query_as::<Postgres, RecipeInfo>(
        "select info.* from recipe_info info  
    right join 
        (select * from recipe_scores where recipe_id in 
            (select id from  recipe_info where taste =$1 ) 
            order by recipe_scores limit $2) 
            as res
    on info.id = res.recipe_id",
    )
    .bind(taste)
    .bind(limit)
    .fetch_all(&mut conn)
    .await.unwrap();

    for ele in &res {
        println!("{:#?}", ele)
    }
    Ok(res)
}

pub async fn select_recipe(taste: String) -> Result<Vec<RecipeInfo>> {
    let mut conn = create_connection_pool().await?;
    let res = sqlx::query_as::<Postgres, RecipeInfo>("select * from recipe_info where taste = $1 ")
        .bind(taste)
        .fetch_all(&mut conn)
        .await?;

    for ele in &res {
        println!("{:#?}", ele)
    }
    Ok(res)
}

pub async fn select_scores() -> Result<Vec<RecipeScores>> {
    let mut conn = create_connection_pool().await?;
    let res = sqlx::query_as::<Postgres, RecipeScores>(
        "select * from recipe_scores order by recipe_scores limit 5",
    )
    .fetch_all(&mut conn)
    .await?;
    for ele in &res {
        println!("{:?}", ele)
    }
    Ok(res)
}

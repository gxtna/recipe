use crate::recipe::service;

pub async fn select(){
    service::get_recipe("清淡".to_string(),3).await;
}
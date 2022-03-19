use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;

use serde_json::{json, Value};
use uuid::Uuid;

use lambda_runtime::{Context, Error};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Event {
    store_id: String,
    user_id: String,
}

#[derive(Serialize)]
struct Output {
    message: String,
}

async fn handler(event: Event, context: Context) -> Result<Value, Error> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else("eu-north-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let result = client
        .query()
        .table_name("discountcodes")
        .index_name("claimed_by-index")
        .index_name("store_id-index")
        .key_condition_expression(
            "store_id = :store_id"
        )
        .filter_expression(
            "attribute_not_exists(claimed_by)"
        )
        .expression_attribute_values(
            ":store_id",
            AttributeValue::S(String::from(&event.store_id)),
        )
        .limit(1)
        .send()
        .await?;

    if let Some(item) = &result.items {

        let first = item.first();
        let id = &first.unwrap().get("id").unwrap().as_s().unwrap().clone();

        dbg!(first);
        dbg!(id);

        let update = client
            .update_item()
            .table_name("discountcodes")
            .key(
                "id",
                AttributeValue::S(String::from(id))
            )
            .update_expression("set claimed_by = :user_id")
            .expression_attribute_values(
                ":user_id",
                AttributeValue::S(String::from(&event.user_id))
            );

        update.send().await?;
    }

    // Ok(json!({resp.items.item}));

    Ok(json!({ "message": "Record written!" }))
}
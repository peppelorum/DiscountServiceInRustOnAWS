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
    number_of_codes: u32,
    short_name: String
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

    let mut i = 0;
    let number_of_codes = u32::from(event.number_of_codes);
    let short_name = String::from(event.short_name);

    while i < number_of_codes {
        let code = format!("{}-{}", short_name, i);
        let uuid = Uuid::new_v4().to_string();
        let request = client.put_item()
            .table_name("discountcodes")
            .item("id", AttributeValue::S(String::from(uuid)))
            .item("store_id", AttributeValue::S(String::from(&event.store_id)))
            .item("code", AttributeValue::S(String::from(code)));

        request.send().await?;
        i = i + 1;
    }

    Ok(json!({ "message": "Record written!" }))
}
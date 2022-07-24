
#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
  simple_logger::init_with_level(log::Level::Info)?;
  lambda_runtime::run(lambda_runtime::service_fn(handle_f)).await?;
  Ok(())
}

async fn handle_f(_event: lambda_runtime::LambdaEvent<serde_json::Value>) -> Result<serde_json::Value, lambda_runtime::Error> {
  match do_f().await {
    Ok(o) => Ok(o),
    Err(e) => {
      log::error!("Error: {}", e);
      Err(Box::new(simple_error::SimpleError::new(format!("{}", e).as_str())))
    }
  }
}

async fn do_f() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
  log::info!("Load aws_config");
  let config = aws_config::load_from_env().await;
  log::info!("Create dynamoDB client");
  let client = aws_sdk_dynamodb::Client::new(&config);

  let table_name = "DynamoDbTestTable";

  log::info!("Query item key 001 from table {}", table_name);
  let item = client.get_item()
             .table_name(table_name)
             .key("ItemId", aws_sdk_dynamodb::model::AttributeValue::S("001".to_string()))
             .send().await?;
  log::info!("response: {:?}", item);
  
  Ok(serde_json::Value::String("Ok".to_string()))
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  simple_logger::init_with_level(log::Level::Info)?;
  if let Err(e) = do_f().await {
    log::error!("Error: {}", e);
  }
  Ok(())
}

async fn do_f() -> Result<(), Box<dyn std::error::Error>> {
  log::info!("Load aws_config");
/*
 * For use on EC2
 */
  let config = aws_config::load_from_env().await;
  let client = aws_sdk_dynamodb::Client::new(&config);
/*
 * For local use
  // Adapted from java implementation in 
  // https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/CodeSamples.Java.html#CodeSamples.Java.RegionAndEndpoint
  let config = aws_sdk_dynamodb::Config::builder()
       .endpoint_resolver( aws_sdk_dynamodb::Endpoint::immutable("http://localhost:8000".parse()?) )
       .region( aws_sdk_dynamodb::Region::new("eu-west-1") )
       .credentials_provider( aws_sdk_dynamodb::Credentials::new("dummy_key", "dummy_secret", None, None, "dummy") )
       .build();
  log::info!("Create dynamoDB client");
  let client = aws_sdk_dynamodb::Client::from_conf(config);
*/

  let table_name = "DynamoDbTestTable";

  log::info!("Query item key 001 from table {}", table_name);
  let item = client.get_item()
             .table_name(table_name)
             .key("ItemId", aws_sdk_dynamodb::model::AttributeValue::S("001".to_string()))
             .send().await?;
  log::info!("response: {:?}", item);
  
  Ok(())
}


use serde::{Serialize, Deserialize};
use reqwest::Error;
use std::collections::HashMap;

pub use serde;
pub use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct CombineInfoModules {
  success: bool,
  info: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombineInfoExports {
  success: bool,
  functions: serde_json::Value
}

impl Default for CombineInfoExports {
  fn default() -> CombineInfoExports{
    CombineInfoExports {
      success: false,
      functions: serde_json::from_str("{}").unwrap()
    }
  } 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CombineResult {
  success: bool,
  data: serde_json::Value
}

#[derive(Debug)]
pub struct CombineArguments {
  arguments: Vec<serde_json::Value>
}

impl CombineArguments {
  pub fn new () -> CombineArguments {
    CombineArguments {
      arguments: vec![]
    }
  }

  pub fn push(mut self, argument: impl Serialize) -> CombineArguments {
    let _ = &self.arguments.push(serde_json::value::to_value(argument).unwrap());
    self
  }
}

async fn get_combine_info_exports (client: &reqwest::Client, host:&str, module: &str) -> Result<CombineInfoExports, Error> {
  let mut href = String::from(host);
  href.push_str("data");

  let mut map = HashMap::new();
  map.insert("module", module);
  map.insert("info", "true");

  let result_text = client.post(href).json(&map).send().await?.text().await?;
  let combine_info_exports: CombineInfoExports = serde_json::from_str(&result_text).unwrap();
  if !combine_info_exports.success {
    panic!("get_combine_info_exports failed did not return a success true")
  }
  Ok(combine_info_exports)
}

async fn run_combine_function (client: &reqwest::Client, host: &str, module: &str, export: &str, arguments: CombineArguments) -> Result<serde_json::Value, Error> {
  let json = serde_json::json!({"module": module, "export": export, "arguments": arguments.arguments});
  let mut href = String::from(host);
  href.push_str("data");

  let result_text = client.post(href).json(&json).send().await?.text().await?;
  let combine_result: CombineResult = serde_json::from_str(&result_text).unwrap();
  if !combine_result.success {
    panic!("run_combine_function failed did not return a success true")
  }
  Ok(combine_result.data)
}

async fn get_combine_value (client: &reqwest::Client, host: &str, module: &str, export: &str) -> Result<serde_json::Value, Error> {
  Ok(run_combine_function(client, host, module, export, CombineArguments::new()).await?)
}

fn unwrap_result <ContentType: serde::de::DeserializeOwned> (result: Result<serde_json::Value, reqwest::Error>) -> ContentType {
  serde_json::from_value(result.unwrap()).unwrap()
}

pub struct Combine {
  host: String,
  client: reqwest::Client,
  module: String
}

impl Combine {
  pub async fn new (host: &str, module: &str) -> Combine {
    let combine = Combine {
      host: String::from(host),
      client: reqwest::Client::new(),
      module: String::from(module)
    };
    let _result = combine.get_combine_info_exports(&module).await;

    combine
  }

  pub async fn get_combine_info_exports (&self, module: &str) -> Result<CombineInfoExports, Error> {
    get_combine_info_exports(&self.client, &self.host, module).await
  }

  pub async fn run_combine_function <ContentType: serde::de::DeserializeOwned> (&self, export: &str, arguments: CombineArguments) -> ContentType {
   let result: ContentType = unwrap_result(run_combine_function(&self.client, &self.host, &self.module, export, arguments).await);
   result
  }

  pub async fn get_combine_value <ContentType: serde::de::DeserializeOwned>  (&self, export: &str) -> ContentType {
    let result: ContentType = unwrap_result(get_combine_value(&self.client, &self.host, &self.module, export).await);
    result
  }
}
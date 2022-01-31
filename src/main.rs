mod combine;

fn foo <T> (t: &str) {
  println!("{:?}", t);
}

#[tokio::main]
async fn main () {
  /*let x = &mut foo::<i32>;
  x("Hello");
  let c = combine::Combine::new("http://localhost:3000/api/", "user").await;
  let y: i32 = c.get_combine_value("test").await;
  println!("{:?}", y);

  let t: serde_json::Value = c.run_combine_function("checkCredentials", combine::CombineArguments::new().push("nqDjVRYV").push("123456")).await;

  println!("{:?}", t);*/
  let api = combine_rust::combine::Combine::new("http://localhost:3001/api/", "api.js").await;
  let shutdown: combine_rust::combine::serde_json::Value = api.run_combine_function("shutdown", combine_rust::combine::CombineArguments::new()).await;
    println!("{:?}", shutdown);
}

pub fn test () {
  println!("test")
}
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    client.set("hello", "wolrd".into()).await?;

    let result = client.get("hello").await?;
    match result {
        Some(v) => {
            println!("{:?}", v);
        }
        None =>{}
    }
    Ok(())

}
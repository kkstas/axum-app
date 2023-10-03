use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn http_tests() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello?name=Kamil").await?.print().await?;
    hc.do_get("/hello2/Kamil").await?.print().await?;
    hc.do_get("/").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username":"demo1",
            "pwd":"welcome"
        }),
    )
    .await?
    .print()
    .await?;
    Ok(())
}

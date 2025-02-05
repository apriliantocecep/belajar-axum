use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:7878")?;

    hc.do_get("/hello?name=Jane").await?.print().await?;

    Ok(())
}
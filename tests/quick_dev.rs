use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/vvv").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
                    "username": "vvv",
                    "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    let create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title" : "concert"
        }),
    );
    create_ticket.await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}

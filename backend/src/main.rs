use rocket_ws::{WebSocket, Channel};
use rocket::futures::{SinkExt, StreamExt};

#[rocket::get("/")]
fn chat(ws:WebSocket) ->Channel<'static> {
    ws.channel(move | mut stream| Box::pin(async move {
        while let Some(msg) = stream.next().await {
            let _ = stream.send(msg?).await;
        }
        Ok(())
    }))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            chat
        ])
        .launch()
        .await;
}

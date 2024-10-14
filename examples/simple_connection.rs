use twitch_comment_stream::TwitchCommentStream;

#[tokio::main]
async fn main() {
    let channel = std::env::args().nth(1).expect("Usage: cargo run <channel>");
    let mut stream = TwitchCommentStream::new(channel);
    stream.connect().await.expect("Failed to connect");

    while let Ok(comment) = stream.next().await {
        println!("{}: {}", comment.user, comment.body);
    }
}


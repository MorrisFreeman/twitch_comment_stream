# Twitch Comment Stream

This Rust library provides a simple interface to connect to Twitch chat and read comments in real-time using WebSockets.

## Features

- Connect to Twitch chat using WebSockets.
- Read and parse chat messages.
- Handle PING/PONG messages to maintain the connection.

## Installation

Add this library to your `Cargo.toml`:

```toml
[dependencies]
twitch-comment-stream = "0.1.0"
```

## Usage

Here's a basic example of how to use the `TwitchCommentStream`:

```rust
use twitch_comment_stream::TwitchCommentStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut stream = TwitchCommentStream::new("your_channel_name".to_string());
  stream.connect().await?;
  while let Ok(comment) = stream.next().await {
    println!("{}: {}", comment.user, comment.body);
  }
  Ok(())
}
```


## API

### TwitchCommentStream

- `new(channel: String) -> Self`: Creates a new instance for the specified channel.
- `connect(&mut self) -> Result<(), Box<dyn std::error::Error>>`: Connects to the Twitch chat.
- `write_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>>`: Sends a message to the chat.
- `next(&mut self) -> Result<Comment, Box<dyn std::error::Error>>`: Retrieves the next comment from the chat.

### Comment

- `user: String`: The username of the commenter.
- `body: String`: The content of the comment.

## Testing

Run the tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License.

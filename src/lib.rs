use futures_util::{
    sink::SinkExt,
    StreamExt
};
use tokio_tungstenite::{
    WebSocketStream,
    connect_async,
    tungstenite::Message,
    MaybeTlsStream
};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct TwitchCommentStream {
    channel: String,
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

#[derive(Debug)]
pub struct Comment {
    pub user: String,
    pub body: String,
}

impl TwitchCommentStream {
    pub fn new(channel: String) -> Self {
        Self {
            channel,
            ws_stream: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async("wss://irc-ws.chat.twitch.tv/").await?;
        self.ws_stream = Some(ws_stream);
        self.write_message(format!("NICK justinfan12345")).await?;
        self.write_message(format!("JOIN #{}", self.channel)).await?;
        Ok(())
    }

    pub async fn write_message(&mut self, message: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws_stream) = &mut self.ws_stream {
            ws_stream.send(Message::Text(message)).await?;
        }
        Ok(())
    }

    pub async fn next(&mut self) -> Result<Comment, Box<dyn std::error::Error>> {
        while let Some(message) = self.ws_stream.as_mut().unwrap().next().await {
            let message = message?;
            let text = message.to_text()?;

            if self.handle_ping(&text).await? {
                continue;
            }

            if let Some(comment) = self.parse_message(&text) {
                return Ok(comment);
            }
        }
        Err("Stream ended unexpectedly".into())
    }

    async fn handle_ping(&mut self, text: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if text.starts_with("PING") {
            self.write_message("PONG :tmi.twitch.tv\r\n".to_string()).await?;
            return Ok(true);
        }
        Ok(false)
    }

    fn parse_message(&self, text: &str) -> Option<Comment> {
        let parts = text.splitn(4, ' ').collect::<Vec<&str>>();
        if parts[1] == "PRIVMSG" {
            let user = parts[0].trim_start_matches(':').split('!').next().unwrap().to_string();
            let body = parts[3].trim_start_matches(':').trim_end().to_string();
            Some(Comment { user, body })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message() {
        let stream = TwitchCommentStream::new("test_channel".to_string());
        let comment = stream.parse_message(":test_user!test_user@test_user.tmi.twitch.tv PRIVMSG #test_channel :Hello, world!\r\n").unwrap();
        assert_eq!(comment.user, "test_user");
        assert_eq!(comment.body, "Hello, world!");
    }
}

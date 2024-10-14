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
        loop {
            let message = self.ws_stream.as_mut().unwrap().next().await.unwrap()?;
            let text = message.to_text()?;

            if text.starts_with("PING") {
                self.write_message("PONG :tmi.twitch.tv\r\n".to_string()).await?;
                continue;
            }

            // メッセージを解析
            let parts: Vec<&str> = text.split(' ').collect();
            if parts.len() >= 4 && parts[1] == "PRIVMSG" {
                let user = parts[0].trim_start_matches(':').split('!').next().unwrap_or("").to_string();
                let body = parts[3..].join(" ").trim_start_matches(':').trim_end().to_string();

                return Ok(Comment { user, body });
            }
        }
    }
}

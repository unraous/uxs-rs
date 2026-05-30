use serde::{Serialize, Deserialize};
use futures_util::StreamExt;
use std::thread;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
pub enum WSRequest {
    SolveQuestions{ html: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", content = "data")]
pub enum WSResponse {
    Error{ code: u16, message: String },
}

impl WSRequest {
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}


async fn on_event(event: WSRequest) {
    match event {
        WSRequest::SolveQuestions { html } => {
            log::debug!("收到SolveQuestions事件: {}", html);
            // 在这里处理HTML内容，例如解析问题并生成答案
        }
    }

}

async fn on_message(msg: tokio_tungstenite::tungstenite::protocol::Message) {
    match msg {
        tokio_tungstenite::tungstenite::protocol::Message::Text(text) => {
            log::debug!("收到WebSocket消息: {}", text);
            match WSRequest::from_json(&text) {
                Ok(event) => on_event(event).await,
                Err(e) => log::error!("解析WebSocket消息失败: {}", e),
            }
            
        }
        _ => {
            log::warn!("收到其他类型的WebSocket消息");
        }
    }
}

async fn client(stream: tokio::net::TcpStream) {
    if let Ok(mut stream) = tokio_tungstenite::accept_async(stream).await {
        log::info!("WebSocket客户端已连接");
        
        while let Some(Ok(msg)) = stream.next().await {
            on_message(msg).await;
        }
        log::warn!("WebSocket客户端已断开连接");
    }
}

async fn observe(listener: &tokio::net::TcpListener) {
    match listener.accept().await {
        Ok((stream, _)) => {
            tokio::spawn(client(stream));
        }
        Err(e) => {
            log::error!("接受连接失败: {}", e);
        }
    }
}

async fn server() {
    let addr = "127.0.0.1:9817";
    let listener = tokio::net::TcpListener::bind(addr).await
        .expect("无法绑定WebSocket端口");
    
    log::info!("WebSocket服务器启动在 {}", addr);
    
    loop {
        observe(&listener).await;
    }
}

fn thread_init() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(server());
}


/// Initializes the WebSocket server in a separate thread to handle incoming connections and messages.
pub fn setup() {
    thread::spawn(thread_init);
}
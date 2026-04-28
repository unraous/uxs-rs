use futures_util::StreamExt;
use std::thread;
use log::{info, debug, warn, error};

async fn on_message(msg: tokio_tungstenite::tungstenite::protocol::Message) {
    match msg {
        tokio_tungstenite::tungstenite::protocol::Message::Text(text) => {
            debug!("收到WebSocket消息: {}", text);
            // 这里可以添加对消息的处理逻辑
        }
        tokio_tungstenite::tungstenite::protocol::Message::Binary(bin) => {
            debug!("收到WebSocket二进制消息: {} bytes", bin.len());
            // 这里可以添加对二进制消息的处理逻辑
        }
        _ => {
            debug!("收到其他类型的WebSocket消息");
        }
    }
}

async fn client(stream: tokio::net::TcpStream) {
    if let Ok(mut stream) = tokio_tungstenite::accept_async(stream).await {
        info!("WebSocket客户端已连接");
        
        while let Some(Ok(msg)) = stream.next().await {
            on_message(msg).await;
        }
        warn!("WebSocket客户端已断开连接");
    }
}

async fn observe(listener: &tokio::net::TcpListener) {
    match listener.accept().await {
        Ok((stream, _)) => {
            tokio::spawn(client(stream));
        }
        Err(e) => {
            error!("接受连接失败: {}", e);
        }
    }
}

async fn server() {
    let addr = "127.0.0.1:9817";
    let listener = tokio::net::TcpListener::bind(addr).await
        .expect("无法绑定WebSocket端口");
    
    info!("WebSocket服务器启动在 {}", addr);
    
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
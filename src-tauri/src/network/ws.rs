use futures_util::StreamExt;
use std::thread;
use log::{info, debug, error};


/// Initializes the WebSocket server in a separate thread to handle incoming connections and messages.
pub fn init() {
    thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let addr = "127.0.0.1:9817";
            let listener = tokio::net::TcpListener::bind(addr).await
                .expect("无法绑定WebSocket端口");
            
            info!("WebSocket服务器启动在 {}", addr);
            
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        tokio::spawn(async move {
                            if let Ok(mut stream) = tokio_tungstenite::accept_async(stream).await {
                                info!("WebSocket客户端已连接");
                                
                                while let Some(msg) = stream.next().await {
                                    match msg {
                                        Ok(msg) => {
                                            debug!("收到消息: {:?}", msg);
                                        }
                                        Err(e) => {
                                            error!("WebSocket错误: {}", e);
                                            break;
                                        }
                                    }
                                }
                            }
                        });
                    }
                    Err(e) => {
                        error!("接受连接失败: {}", e);
                    }
                }
            }
        });
    });
}
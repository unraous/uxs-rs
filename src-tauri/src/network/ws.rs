use crate::core::qa_pipeline::execute_qa_workflow;

use serde::{Serialize, Deserialize};
use futures_util::{StreamExt, SinkExt};
use std::thread;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
pub enum WSRequest {
    SolveQuestions{ html: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status", content = "data")]
pub enum WSResponse {
    Success{ answer: String },
    Error{ code: u16, message: String },
}

impl WSRequest {
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}


async fn on_event(event: WSRequest, tx: mpsc::UnboundedSender<String>) {
    match event {
        WSRequest::SolveQuestions { html } => {
            log::debug!("开始处理 HTML 答题，长度: {}", html.len());

            match execute_qa_workflow(&html).await {
                Ok(answers) => {
                    let resp = WSResponse::Success { 
                        answer: serde_json::to_string(&answers).unwrap_or_default() 
                    };
                    let _ = tx.send(serde_json::to_string(&resp).unwrap());
                }
                Err(e) => {
                    let resp = WSResponse::Error { code: 500, message: e.to_string() };
                    let _ = tx.send(serde_json::to_string(&resp).unwrap());
                }
            }
        }
    }
}

async fn on_message(msg: Message, tx: mpsc::UnboundedSender<String>) {
    if let Message::Text(text) = msg {
        if let Ok(event) = WSRequest::from_json(&text) {
            on_event(event, tx).await;
        }
    }
}

async fn client(stream: TcpStream) {
    if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
        log::info!("WebSocket 客户端已连接");
        
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();

        // 任务 1：专门负责从通道读消息并发送给客户端（写任务）
        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = ws_sender.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await {
                    log::error!("发送 WebSocket 消息失败: {}", e);
                    break;
                }
            }
        });

        // 任务 2：专门负责接收客户端消息（读任务）
        while let Some(Ok(msg)) = ws_receiver.next().await {
            on_message(msg, tx.clone()).await;
        }

        log::warn!("客户端断开，清理任务");
        send_task.abort(); // 停止写任务
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

pub fn setup() {
    thread::spawn(thread_init);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_tungstenite::connect_async;
    use std::time::Duration;
    use dotenv::dotenv;
    use crate::{config::CONFIG, core::qa_pipeline::llm::LLM};

    #[tokio::test]
    async fn test_mock_frontend_flow() {
        // 1. 尝试在后台启动服务器
        // 注意：如果你的 9817 端口已经被占据（比如你正在运行应用），这里会报错。
        // 建议测试前先关闭正在运行的后端程序。
        tokio::spawn(async {
            let _ = server().await;
        });

        // 等待一秒确保服务器完成绑定
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        // 2. 模拟前端连接
        let (mut ws_stream, _) = connect_async("ws://127.0.0.1:9817")
        .await
        .expect("无法连接到 WebSocket 服务器");
    
        println!("✅ Mock 前端已成功连接");
        dotenv().ok();
    
        let api_key = std::env::var("BIGMODEL_API_KEY")
            .expect("请在 .env 中设置 BIGMODEL_API_KEY");
        CONFIG.llm.bigmodel.set_key(&api_key);

        let html_path = "tests/assets/course-page/webpage.html";
        let html_content = std::fs::read_to_string(html_path)
            .unwrap_or_else(|_| panic!("读取不到测试文件: {}", html_path));
        println!("✅ 已读取 HTML 文件，长度: {}", html_content.len());

        let request = WSRequest::SolveQuestions { html: html_content };
        let request_json = serde_json::to_string(&request).unwrap();
        
        ws_stream.send(Message::Text(request_json.into())).await.expect("发送请求失败");

        match tokio::time::timeout(Duration::from_secs(180), ws_stream.next()).await {
            Ok(Some(Ok(Message::Text(text)))) =>  {
                println!("📥 收到后端响应: {}", text);
                let response: WSResponse = serde_json::from_str(&text).expect("响应格式解析失败");
                match response {
                    WSResponse::Success { answer } => {
                        println!("🎉 答题成功！返回答案: {}", answer);
                    }
                    WSResponse::Error { code, message } => {
                        println!("❌ 后端处理报错: [Code {}] {}", code, message);
                    }
                }
                
            }
            Ok(None) => panic!("WebSocket 连接意外关闭"),
            Err(_) => panic!("测试超时：后端在 180 秒内没有返回答案"),
            _ => {}
        }
    }
}
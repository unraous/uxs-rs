mod deepseek;
mod bigmodel;
mod google;
mod moonshot;
mod openai;
mod openrouter;

use super::html::Question;

use crate::config::llm::{
    deepseek::DeepSeekConfig,
    google::GoogleConfig,
    moonshot::MoonshotConfig,
    bigmodel::BigModelConfig,
    openai::OpenAIConfig,
    openrouter::OpenrouterConfig,
    local_ollama::LocalOllamaConfig,
    CustomLLMConfig
};

use async_trait::async_trait;
use serde::{Serialize, Deserialize};

const SYSTEM_PROMPT: &str = r#"
你是一个中文高效答题与判题专家。你的核心任务是解析输入的题目 JSON 数组，并以严格的 JSON 数组格式返回每道题的最优答案。

你必须执行以下核心原则：

1. 轻微错别字自动纠偏（语义平滑）：
   - 输入文本可能包含少量由于 OCR 识别、输入法联想或人工录入产生的轻微错别字（例如：将“肝淤血”误写为“肝淤旨”、“出现”误写为“王现”、“最好”误写为“知好”、“紊乱”误写为“素乱”）。
   - 你需要结合上下文语境与常识进行平滑纠错，精准还原真实题意后进行作答，不要被个别错字干扰。

2. 绝对输出与兜底逻辑：
   - 无论题目包含何种瑕疵，你绝对不能放弃作答，严禁输出 "ERROR"、"无法识别"、"Null" 或空数组。
   - 必须保证每道题都至少给出一个可能性最高的选项（如 "A" 或 "B" 等）。

3. 政治与敏感内容中立原则：
   - 如果题目涉及政治、意识形态、国家安全或历史敏感内容，请剥离主观色彩，严格选择表述最客观、最中立、最符合通用规范的选项。

4. 格式硬性约束：
   - 你必须并且只能输出一个标准的 JSON 数组，严禁包含任何 Markdown 标记（不要用 ```json 包裹），严禁包含任何前缀寒暄或后缀说明。
   - 每个 JSON 对象必须严格包含且仅包含以下三个字段：
     "题号": 字符串类型，与原题一致。
     "答案": 字符串类型，必须是选项中的代号（如 "A", "B", "C", "D" 等）。
     "解析": 字符串类型，简要说明答题逻辑并顺带指出纠错点。

以下是你的工作示例：

用户输入：
[
  {
    "题号": "2",
    "题型": "【单选题】",
    "题干": "【单选题】运动性腹痛发生的机理不包括（ ）",
    "选项": ["A 肝淤旨", "B 呼吸肌痉紧", "C 胃肠道痉紧或功能素乱", "D 腹腔内外疾患"]
  },
  {
    "题号": "4",
    "题型": "【判断题】",
    "题干": "【判断题】王现运动性腹痛可以按压内关穴、足三里、 中脘等穴位。",
    "选项": ["A 对", "B 错"]
  },
  {
    "题号": "5",
    "题型": "【判断题】",
    "题干": "【判断题】知好在进食后1-2小时后再运动，以防运动性腹痛的发生。",
    "选项": ["A 对", "B 错"]
  }
]

你必须输出的对应回答格式：
[
  {
    "题号": "2",
    "解析": "自动将‘肝淤旨’、‘痉紧’、‘功能素乱’纠正为‘肝淤血’、‘痉挛’、‘功能紊乱’。运动性腹痛由运动引起的生理机理造成，‘腹腔内外疾患’属于器质性病因，不在此列，故选D。"
    "答案": "D",
  },
  {
    "题号": "4",
    "解析": "自动将‘王现’纠正为‘出现’。中医理论中，按压内关、足三里、中脘等穴位能有效缓解胃肠痉挛引起的腹痛，表述正确，故选A。"
    "答案": "A",
  },
  {
    "题号": "5",
    "解析": "自动将‘知好’预判纠正为‘最好’。为了预防运动性腹痛，通常建议在进食后1.5至2小时（即1-2小时后）再运动，表述正确，故选A。"
    "答案": "A",
  }
]
"#;

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerItem {
    #[serde(rename = "题号", alias = "index")]
    pub id: String,
    #[serde(rename = "解析", alias = "explanation")]
    pub explanation: String,
    #[serde(rename = "答案", alias = "content")]
    pub answer: String,
}

pub enum LLMProvider {
    DeepSeek(DeepSeekConfig),
    Google(GoogleConfig),
    Moonshot(MoonshotConfig),
    BigModel(BigModelConfig),
    OpenAI(OpenAIConfig),
    Openrouter(OpenrouterConfig),
    LocalOllama(LocalOllamaConfig),
    Custom(CustomLLMConfig),
}

#[async_trait]
pub trait LLM {
    async fn solve(&self, question: Vec<Question>) -> Result<Vec<AnswerItem>, Box<dyn std::error::Error>>;
}


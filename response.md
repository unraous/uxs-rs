

已提取 2 资源



已提取 2 资源

从官方文档和GitHub discussions来看，**Tauri关于WebView持久化的文档相当有限**。官方配置文档中只说了：

**在 [WindowConfig 中的 `dataDirectory`](https://tauri.app/reference/config/#datadirectory)**：
> 设置WebView数据目录（localStorage、缓存等）的自定义路径，相对于 `appDataDir()/${label}`

但这只是简单说明，没有详深入讨论实现细节、常见问题或最佳实践。

**从GitHub discussions看，也没有专门讨论这个话题的资源。**

**根据你的情况，关键发现是：**
- Cookies 确实被启用了（`navigator.cookieEnabled: true`）
- 但 Chaoxing 外部网站可能有自己的安全策略，阻止在嵌入式环境中保存登陆状态
- Windows 上如果多个 WebView 共享同一个 `data_directory`，它们必须有相同的 `additionalBrowserArgs` 和 `scrollBarStyle` 等设置

**建议的下一步：**

1. 检查 Chaoxing 的网络请求，看看登陆后返回的 Set-Cookie 头是否设置了 `SameSite=Strict` 或其他限制
2. 尝试在 tauri.conf.json 中配置：
```json
{
  "app": {
    "windows": [{
      "dataDirectory": "chaoxing"
    }]
  }
}
```
3. 考虑使用 Tauri 的 cookie API 手动注入或管理 cookies

可惜 Tauri 官方对这块的文档确实不够完善。
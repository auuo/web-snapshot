# web-snapshot

基于 tokio 的异步爬虫框架，提供 `ElementHandler` 即可爬取数据。

## 组件
- SpiderApplication: 核心，控制流程
- SpiderContext: 传递给 handler，目前主要作用是 push url
- UrlManager: url 管理器，存储和提供 url。预定义了一个宽度优先的管理器，也可以自己实现。
- ElementHandler: 元素处理器，用来处理从 url 获取的返回数据，element 有 `HTML`, `JSON`, `IMAGE` 等类型。
- ErrorHandler: 错误处理器


## example
bin/main 爬取花瓣网图片

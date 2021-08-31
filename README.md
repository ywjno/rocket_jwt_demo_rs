本代码来自 b 站 up 主 [原子之音](https://space.bilibili.com/437860379) 的 [rust jwt实战](https://www.bilibili.com/video/BV1UP4y1s7Ff)。

### How to use

使用 `cargo run` 启动服务后通过 [postman](https://www.postman.com) 等工具进行验证。

跟原版相比添加如下内容：
- `http://127.0.0.1:8000/jwt?sub=<sub>` 对传入参数进行获取 token 的处理，该 token 有效期为1天。
- `http://127.0.0.1:8000/jwt/value?sub=<sub>` 对传入参数进行 token 的验证。
    - 记得在 headers 里面添加 token 内容。

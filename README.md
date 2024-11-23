# Actix Web Server

![主要的流程图](./docs/screenshot123_22112024_220519.jpg)

- `Actix-Http-Server` 实现了 Http 协议
- `Actix-Http-Server` 内可以开启 `Actix-App`
- 当`App` 接受到请求之后会交给 `Actix-Route` 来分发路由到具体的 `Actix-Handler`
- `Actix-Handler` 处理好请求之后会返回 `Actix-Response` 给客户端

## 异步线程

`Actix` 默认会开启多线程来处理请求

- `Actix` 支持两类的并发
  - 异步`IO`, 给定的 `OS` 原生线程在等待 `I/O` 时执行其他任务(例如侦听网络连接)
  - 多线程并行: 默认情况下启动`OS` 原生线程的数量与系统逻辑 `CPU` 数量相同

## Restful Api

![restful_api](./docs/actix-restful-api-123_22112024_221523.jpg)

- `POST /courses`  创建一个课程
- `GET /courses/teacher_id` 获取某个老师的所有课程
- `GET /courses/teacher_id/course_id` 获取某个老师的某个课程



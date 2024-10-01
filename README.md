hello，欢迎来到极客时间课程《Rust 实战 · 手写下一代云原生消息队列》的 Demo Project，很高兴在这里遇到你，也祝你在Rust 学习路上披荆斩棘，高歌猛进。

这是一个运行的 Demo 项目，你下载完代码后，需要安装一下环境依赖，然后即可在本地运行。

## RobustMQ-Geek 和 RobustMQ的关系

- RobustMQ-Geek项目是课程《Rust 实战 · 手写下一代云原生消息队列》的配套示例项目。
- 《Rust 实战 · 手写下一代云原生消息队列》课程是基于开源项目 RobustMQ 的实战经验总结而来的。
- RobustMQ 是一个基于Apache LiCENSE 2.0 的消息队列领域的 Rust 开源项目

所以说 RobustMQ-Geek是 RobustMQ 的子集。也就是说 RobustMQ 里面会有更完善的代码实现。感兴趣的可以查看我们主项目RobustMQ，https://github.com/robustmq/robustmq 。

既然你看到了这里，请给我们送个见面礼吧🌟🎒 🌟🎒 ，给 https://github.com/robustmq/robustmq 点个 Star 🌟先！

## RobustMQ 期待你的参与
欢迎各位勇士加入RobustMQ的神秘领域！🌟

在这个充满魔法与代码的世界里，我们的RobustMQ还是一个初出茅庐的少年，但它拥有着无限的可能性和潜力！🚀

👩‍💻👨‍💻我们诚邀各位Rust大师，一起挥舞你们的键盘之剑，用Rust的魔法编织出强大的代码之网。在这里，我们不仅共同打造一个坚不可摧的消息队列，更是在探索技术的边界，一起成长，一起进步！

📚学习Rust，就像是掌握一门古老的语言，它将赋予你构建安全、高效软件的力量。让我们一起深入Rust的奥秘，解锁它的全部潜力！

🌐构建基础软件，就像是在数字世界中建立一座坚不可摧的城堡。我们的目标是让RobustMQ成为连接现实与虚拟，过去与未来的桥梁。

🎉所以，拿起你的装备，准备好你的代码，让我们一起踏上这场激动人心的冒险之旅吧！未来已来，让我们共同创造历史！

🌈欢迎加入RobustMQ开发组，让我们一起成为改变世界的力量！🚀🌟

欢迎查看🔮💻[《RobustMQ 魔法入门手册》](https://shimo.im/docs/XKq427g9v0Tj0PAN)


## 安装环境

1. 安装 Rust 基础环境
参考文档：https://course.rs/first-try/installation.html 。 安装完成后，查看 rustc 和 cargo 版本，能看到版本即安装成功
```
FWR3KG21WF:~ $ rustc --version
rustc 1.74.0 (79e9716c9 2023-11-13)
FWR3KG21WF:~ $ cargo version
cargo 1.74.0 (ecb9851af 2023-10-18)
```

2. 安装 Cmake.
mac 安装命令如下：
```
brew install cmake
```
安装完成后，查看cmake版本，能看到版本即安装成功
```
FWR3KG21WF:~ $ cmake --version
cmake version 3.30.2
CMake suite maintained and supported by Kitware (kitware.com/cmake).
```

3. 安装 RocksDB
参考文档：https://github.com/rust-rocksdb/rust-rocksdb 安装 rocksdb。mac 安装命令如下：
```
brew install rocksdb
```
安装完成后，查看rocksdb 版本，能看到版本即安装成功
```
FWR3KG21WF:~$ rocksdb_ldb --version
ldb from RocksDB 9.4.0
```

## 运行项目
建议直接在VsCode 中运行项目。VsCode环境配置请参考文档：https://course.rs/first-try/editor.html

打开项目后，直接打开文件：src/cmd/src/placement-center/server.rs 。点击运行main函数即可，启动成功日志如下：
```
2024-09-19T15:12:22.945107+08:00 INFO placement_center - PlacementCenterConfig { cluster_name: "placement-test", addr: "127.0.0.1", node_id: 1, grpc_port: 8871, nodes: {"1": String("127.0.0.1:1228")}, http_port: 8971, data_path: "/tmp/placement-center", log: Log { log_config: "./config/log4rs.yaml", log_path: "./logs" } }
2024-09-19T15:12:22.959227+08:00 INFO placement_center::server::http::server - Broker HTTP Server start. port:8971
2024-09-19T15:12:22.959334+08:00 INFO placement_center::server::grpc::server - Broker Grpc Server start. port:8871
2024-09-19T15:12:24.696262+08:00 INFO placement_center::raft::machine - Node Raft Role changes from  【Follower】 to 【Leader】
>> save entry index:2, value:Entry { entry_type: EntryNormal, term: 2, index: 2, data: [], context: [], sync_log: false }
2024-09-19T15:12:24.697367+08:00 INFO placement_center::raft::machine - save hardState!!!,len:HardState { term: 2, vote: 1, commit: 1 }
2024-09-19T15:12:24.697815+08:00 INFO placement_center::raft::machine - save light rd!!!,commit:2
>> commit entry index:2
```

你可以看到启动了 HTTP 和 GRPC Server，同时 Raft 集群也选举出 Leader 了

如果运行遇到问题，欢迎添加我们的微信群进行讨论。点击添加微信群：https://jsj.top/f/Dbjfwl


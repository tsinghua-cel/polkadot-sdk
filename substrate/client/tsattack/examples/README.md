# TSAttack Examples

这个目录包含了演示如何使用 TSAttack wrapper 的示例代码。

## 可用示例

### 1. simple.rs - 基础示例
最简单的示例，展示基本的连接检查和条件执行。

```bash
cargo run --example simple
```

### 2. usage.rs - 完整示例
完整的 API 使用演示，包括所有可用的功能。

```bash
cargo run --example usage
```

## 运行示例

### 基本运行
```bash
# 进入 tsattack 目录
cd /root/code/polkadot-sdk/substrate/client/tsattack

# 运行简单示例
cargo run --example simple

# 运行完整示例
cargo run --example usage
```

### 带日志运行
```bash
# 启用日志输出
RUST_LOG=debug cargo run --example usage
```

### 模拟攻击服务可用
```bash
# 设置环境变量（即使没有真实服务）
export TSATTACK_SERVICE_URL=http://localhost:50051
cargo run --example simple
```

## 示例行为

### 无攻击服务时
- 示例会检测到服务不可用
- 显示 API 使用方法
- 演示条件执行的安全性
- 不会产生错误

### 有攻击服务时
- 连接到指定的服务 URL
- 执行实际的攻击操作
- 显示操作结果
- 记录详细日志

## 开发新示例

要创建新的示例：

1. 在 `examples/` 目录下创建新的 `.rs` 文件
2. 使用 `cargo run --example <filename>` 运行
3. 示例会自动包含在构建中

## 常见问题

### Q: 示例无法连接到服务
A: 这是正常的！示例设计为在没有真实攻击服务时也能运行，用于演示 API 用法。

### Q: 如何测试真实的攻击功能？
A: 您需要：
1. 启动一个真实的攻击服务在指定端口
2. 设置 `TSATTACK_SERVICE_URL` 环境变量
3. 重新运行示例

### Q: 示例会影响生产环境吗？
A: 不会。示例使用测试数据，并且有条件执行保护。在没有明确配置攻击服务时，操作会被安全地跳过。
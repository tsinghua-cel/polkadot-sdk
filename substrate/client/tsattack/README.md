# TSAttack Client Wrapper

这个包提供了对 `tsattack-client` 包的高级包装方法，简化了在 Substrate 中使用攻击服务的过程。

## 功能特性

- **线程安全**: 使用 `Arc<Mutex<AttackClient>>` 确保多线程环境下的安全访问
- **运行时管理**: 内置 Tokio 运行时，无需外部异步环境
- **环境配置**: 通过 `TSATTACK_SERVICE_URL` 环境变量自动配置
- **错误处理**: 统一的错误处理和日志记录
- **便利宏**: 提供条件执行宏，避免在非攻击环境下的错误

## 使用方法

### 环境配置

首先设置攻击服务的 URL：

```bash
export TSATTACK_SERVICE_URL="http://localhost:50051"
```

### 基本用法

```rust
use tsattack;

// 报告验证者信息
tsattack::report_validator(0, "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;

// 延迟特定区块
tsattack::delay_block_by_hash("0x1234567890abcdef", 123)?;

// 报告验证者职责
tsattack::report_validator_duty_now("validator_id", 42, 1)?;

// 修改区块数据
let success = tsattack::modify_block(block_data)?;
```

### 条件执行

使用 `attack_if_enabled!` 宏可以在攻击服务不可用时自动跳过操作：

```rust
use tsattack::attack_if_enabled;

attack_if_enabled! {
    tsattack::report_validator(0, "validator_address")
};
```

## API 参考

### 核心函数

#### `report_validator_info(idx: i32, address: String) -> Result<(), String>`
报告验证者信息到攻击服务。

#### `delay_for_block(block_number: i32, block_hash: Vec<u8>, timestamp: i64) -> Result<(), String>`
请求对特定区块进行延迟处理。

#### `report_duty(duties: Vec<(String, i32, i64, i32)>) -> Result<(), String>`
报告多个验证者职责信息。

#### `modify_block(block_data: Vec<u8>) -> Result<bool, String>`
请求修改区块数据，返回操作是否成功。

### 便利函数

#### `report_validator(idx: i32, address: &str) -> Result<(), String>`
简化的验证者报告函数。

#### `delay_block_by_hash(block_hash_hex: &str, block_number: i32) -> Result<(), String>`
通过十六进制哈希字符串延迟区块。

#### `report_single_duty(validator: String, slot: i32, time: i64, priority: i32) -> Result<(), String>`
报告单个验证者职责。

#### `report_validator_duty_now(validator: &str, slot: i32, priority: i32) -> Result<(), String>`
使用当前时间戳报告验证者职责。

### 工具函数

#### `is_attacker_available() -> bool`
检查攻击客户端是否可用。

#### `get_attacker() -> Option<Arc<Mutex<AttackClient>>>`
获取攻击客户端实例（用于高级用法）。

## 错误处理

所有函数都返回 `Result<T, String>`，其中错误字符串包含详细的错误信息。建议使用 `attack_if_enabled!` 宏来处理可选的攻击操作。

## 线程安全

这个包是完全线程安全的，可以在多线程环境中安全使用。内部使用 `Arc<Mutex<>>` 来保护客户端实例。

## 日志记录

当使用 `attack_if_enabled!` 宏时，失败的操作会自动记录为警告级别的日志。确保在你的应用中初始化了日志系统。

## 运行示例

这个包包含了几个示例来演示如何使用 API：

### 基础示例
```bash
# 进入项目目录
cd substrate/client/tsattack

# 运行简单示例
cargo run --example simple

# 运行完整示例
cargo run --example usage
```

### 带环境变量和日志
```bash
# 设置攻击服务 URL 并启用日志
RUST_LOG=debug TSATTACK_SERVICE_URL=http://localhost:50051 cargo run --example usage
```

### 示例说明
- **simple.rs**: 基础功能演示，展示连接检查和条件执行
- **usage.rs**: 完整 API 演示，包括所有可用功能
- 示例设计为在没有真实攻击服务时也能安全运行
- 查看 `examples/README.md` 获取更详细的说明

## 集成到你的项目

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
tsattack = { path = "path/to/substrate/client/tsattack" }
```

然后在代码中使用：

```rust
use tsattack;

// 在你的共识或验证代码中
tsattack::attack_if_enabled! {
    tsattack::report_validator(validator_idx, &validator_address)
};
```
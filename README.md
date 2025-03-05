# pi_style

[![crates.io](https://img.shields.io/crates/v/pi_style)](https://crates.io/crates/pi_style)
[![MIT/Apache-2.0 licensed](https://img.shields.io/crates/l/pi_style)](./LICENSE)

CSS样式结构定义与解析库，提供高效的样式数据处理能力。

## 功能特性

- CSS样式结构定义
- 基于cssparser的高效解析实现
- 支持Serde序列化/反序列化
- 提供样式类型系统（style_type）
- 包含样式解析器（style_parse）

## 安装

在Cargo.toml中添加：

```toml
[dependencies]
pi_style = "0.1.17"
```

## 使用示例

```rust

fn main() {
    let css = r#"
        .container {
            width: 100%;
            padding: 20px;
        }
    "#;
    
    let stylesheet = pi_style::style_parse::parse_class_map_from_string(css, 0).unwrap();
    // 处理解析后的样式数据...
}
```

## 模块结构

- `style`: 基础样式结构定义
- `style_type`: 样式类型系统
- `style_parse`: CSS解析器实现

## 文档

完整API文档请参考：[docs.rs/pi_style](https://docs.rs/pi_style)

## 开发

运行测试用例：
```bash
cargo test
```

运行示例程序：
```bash
cargo run --example style_parse
```

## 许可证

双协议授权：[MIT](LICENSE-MIT) 或 [Apache-2.0](LICENSE-APACHE)

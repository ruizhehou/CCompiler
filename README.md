# CCompiler

A simple C compiler written in Rust.

[中文版本](#中文版本)

<a name="english-version"></a>

## Overview

CCompiler is an educational C compiler implementation written in Rust. It demonstrates the fundamental concepts of compiler design through a complete pipeline from source code to assembly.

### Features

- **Lexer**: Tokenizes C source code with support for all keywords, operators, and literals
- **Parser**: Recursive descent parser that builds an Abstract Syntax Tree (AST)
- **Semantic Analyzer**: Performs symbol table management, scope handling, and basic error checking
- **Code Generator**: Generates x86_64 assembly code in AT&T syntax for macOS

### Supported C Features

- Basic types: `int`, `char`, `void`
- Variable declarations and assignments
- Arithmetic operators: `+`, `-`, `*`, `/`, `%`
- Comparison operators: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Logical operators: `&&`, `||`, `!`
- Bitwise operators: `&`, `|`, `^`, `~`, `<<`, `>>`
- Control flow: `if`, `else`, `while`, `do-while`, `for`
- Functions with parameters
- Compound assignment operators: `+=`, `-=`, `*=`, `/=`, etc.
- Increment/decrement operators: `++`, `--`
- Comments: Both single-line (`//`) and multi-line (`/* */`)

### Building

```bash
cargo build --release
```

### Usage

```bash
# Compile a C file
./target/release/ccompiler input.c

# Specify output file
./target/release/ccompiler input.c output.s
```

### Example

Create a file `hello.c`:

```c
int main() {
    int a;
    int b;
    int c;

    a = 10;
    b = 20;
    c = a + b;

    return c;
}
```

Compile it:

```bash
./target/release/ccompiler hello.c
```

This generates `hello.s` with x86_64 assembly code.

### Project Structure

```
src/
├── ast.rs       # Abstract Syntax Tree definitions
├── lexer.rs     # Lexical analyzer (tokenizer)
├── parser.rs    # Syntax analyzer (parser)
├── semantic.rs  # Semantic analyzer (type checking, symbol table)
├── codegen.rs   # Code generator (x86_64 assembly)
└── main.rs      # CLI interface
```

### Architecture

1. **Lexical Analysis**: Source code → Tokens
2. **Syntax Analysis**: Tokens → Abstract Syntax Tree (AST)
3. **Semantic Analysis**: AST → Validated AST (with symbol table)
4. **Code Generation**: Validated AST → Assembly code

### Limitations

This is an educational compiler with some limitations:
- No support for arrays, structs, or unions
- Limited type checking
- No preprocessor
- Simplified function calling convention
- Only targets x86_64 macOS

### License

This project is for educational purposes.

---

[中文版本](#中文版本) | [Back to Top](#ccompiler)

## 中文版本

[English Version](#english-version) | [回到顶部](#ccompiler)

### 概述

CCompiler 是一个用 Rust 编写的教育性 C 编译器实现。它通过从源代码到汇编的完整管道，展示了编译器设计的基本概念。

### 功能特性

- **词法分析器**：对 C 源代码进行词法分析，支持所有关键字、运算符和字面量
- **语法分析器**：递归下降解析器，构建抽象语法树（AST）
- **语义分析器**：执行符号表管理、作用域处理和基本错误检查
- **代码生成器**：生成 macOS 平台的 x86_64 AT&T 语法汇编代码

### 支持的 C 语言特性

- 基本类型：`int`、`char`、`void`
- 变量声明和赋值
- 算术运算符：`+`、`-`、`*`、`/`、`%`
- 比较运算符：`==`、`!=`、`<`、`<=`、`>`、`>=`
- 逻辑运算符：`&&`、`||`、`!`
- 位运算符：`&`、`|`、`^`、`~`、`<<`、`>>`
- 控制流：`if`、`else`、`while`、`do-while`、`for`
- 带参数的函数
- 复合赋值运算符：`+=`、`-=`、`*=`、`/=` 等
- 自增/自减运算符：`++`、`--`
- 注释：支持单行（`//`）和多行（`/* */`）注释

### 构建

```bash
cargo build --release
```

### 使用方法

```bash
# 编译 C 文件
./target/release/ccompiler input.c

# 指定输出文件
./target/release/ccompiler input.c output.s
```

### 示例

创建文件 `hello.c`：

```c
int main() {
    int a;
    int b;
    int c;

    a = 10;
    b = 20;
    c = a + b;

    return c;
}
```

编译它：

```bash
./target/release/ccompiler hello.c
```

这会生成包含 x86_64 汇编代码的 `hello.s` 文件。

### 项目结构

```
src/
├── ast.rs       # 抽象语法树定义
├── lexer.rs     # 词法分析器（分词器）
├── parser.rs    # 语法分析器（解析器）
├── semantic.rs  # 语义分析器（类型检查、符号表）
├── codegen.rs   # 代码生成器（x86_64 汇编）
└── main.rs      # 命令行接口
```

### 架构

1. **词法分析**：源代码 → 词法单元（Tokens）
2. **语法分析**：词法单元 → 抽象语法树（AST）
3. **语义分析**：AST → 验证后的 AST（带符号表）
4. **代码生成**：验证后的 AST → 汇编代码

### 限制

这是一个教育性质的编译器，存在一些限制：
- 不支持数组、结构体或联合体
- 类型检查功能有限
- 没有预处理器
- 简化的函数调用约定
- 仅支持 x86_64 macOS 平台

### 许可证

本项目仅用于教育目的。

# jni-rs-template

## Usage

### 1. 按照`api`示例编写rust lib

./api/src/lib.rs
```rust
pub fn hello(name: &str) -> String {
    format!("hello {name}!")
}
```

### 2. 修改`dylib`中所有`jni函数名`包含的java包名与类名

./dylib/src/lib.rs
```rust
...
#[no_mangle]
pub extern "system" fn Java_com_example_Api_hello(
    env: JNIEnv,
...
```

### 3. 编译rust端代码，得到动态链接库，将其放置在`java.library.path`对应的路径下并按类型与版本进行重命名

```shell
❯ cargo build -r

❯ exa -TL 1 target/release/
├── ...
├── libapi_dylib.dylib
└── ...

MacOs:
❯ cp './target/release/libapi_dylib.dylib' ~/Library/java/Extensions/libapi_dylib.dylib
Linux:
❯ sudo cp './target/release/libapi_dylib.so' /usr/java/packages/lib/libapi_dylib.so

❯ exa -TL 1 ~/Library/java/Extensions/
├── ...
└── libapi_dylib.dylib
```

### 4. 在`java_lib`中新建对应版本类型的api类，暴露动态链接库中的方法

./java_lib/api/src/main/java/com/example/Api.java
```java
package com.example;

class Api {
    static native String hello(String input);

    static {
        System.loadLibrary("api_dylib");
    }
}
```

### 5. 编译`java_lib`，得到jar包

```shell
❯ cd java_lib
❯ gradle build
```

### 6. 在应用中引用，示例`examples/java_example`

运行前的准备
- 正确引用动态连接库

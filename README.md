# rust-bindings-learn
* 于[github首发](https://github.com/zy445566/rust-bindings-learn),如需转载，请先联系作者。
* 这是一个用于制作node的rust扩展的的中文教程，接下来我将会一步一步教学。
* 由于天朝墙的问题，建议大家先准备好代理。

## 准备
### 如果之前使用过node-gyp则只需要安装rust即可（优势可以不需要python了）

### 在 Unix 系统的软件列表
* `make`
* [GCC](https://gcc.gnu.org)
* [Node](https://nodejs.org)
* [Rust](https://www.rust-lang.org)

### 在 Mac 系统的软件列表
* [Xcode](https://developer.apple.com/xcode/download/)
* [Node](https://nodejs.org)
* [Rust](https://www.rust-lang.org)

### 在 Windows 系统的软件列表
* [windows-build-tools](https://github.com/felixrieseberg/windows-build-tools)
* [Visual C++ Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools)
* [Visual Studio 2015以上](https://www.visualstudio.com/products/visual-studio-community-vs)
* (仅限于Windows Vista / 7)[.NET Framework 4.5.1](http://www.microsoft.com/en-us/download/details.aspx?id=40773)
* [Node](https://nodejs.org)
* [Rust](https://www.rust-lang.org)
* 设置Visual Studio的版本如2017则改为2017即可，npm config set msvs_version 2015

## 开始建立项目
### 先全局安装生成器
``` sh
npm install -g neon-cli
```
### 正式开始建立项目
* 运行命令建立项目，那我们来一发求斐波那契数吧
* 先建立项目，同时看是否之前的环境是否搭建有误
``` sh
neon new fib
cd fib
npm install
```
如果出现以下情况则安装正确(如图)：

### 目录结构


### 开始写我们的斐波那契数吧
打开native目录里的src中的lib.rs文件,修改为
```rust
#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::js::JsInteger;
use neon::js::Variant;

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello node").unwrap())
}
fn fib(call: Call) -> JsResult<JsInteger> {
    let scope = call.scope;
    let option_num = call.arguments.get(scope,0);
    let mut num:i32 = 0;
    if let Some(x1) = option_num {
        if let Variant::Integer(x2) = x1.variant() {
            num = x2.value() as i32;
        }
    }
    Ok(JsInteger::new(scope, my_fib(num)))
}

fn my_fib(num:i32) -> i32
{
    if num < 2
    {
        return 1;
    } else {
        return my_fib(num-1) + my_fib(num-2);
    }
}

register_module!(m, {
    try!(m.export("hello", hello));
    try!(m.export("fib", fib));
    Ok(())
});
```
### 接下来是lib的index.js文件修改为
```node
var addon = require('../native');

console.log(addon.hello());
console.log(addon.fib(30));
```


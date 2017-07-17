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

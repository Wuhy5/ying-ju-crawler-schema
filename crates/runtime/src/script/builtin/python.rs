// TODO: Python 内置函数注册
//
// 使用 RustPython 的 py_module! 宏或手动注册函数
//
// 需要实现的函数:
// 1. trim(text: str) -> str
// 2. json_parse(text: str) -> Any
// 3. base64_encode(text: str) -> str
// 4. base64_decode(text: str) -> str
// 5. url_encode(text: str) -> str
// 6. url_decode(text: str) -> str
// 7. md5(text: str) -> str
// 8. regex_match(pattern: str, text: str) -> List[str]
//
// 示例代码:
// ```python
// vm.add_native_module("builtins".to_owned(), Box::new(builtin_module));
// ```

pub fn register_builtins() {
    // TODO: 实现 Python 内置函数注册
    unimplemented!("Python builtins registration")
}

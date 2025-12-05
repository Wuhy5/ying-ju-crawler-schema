//! JavaScript (Boa) 引擎内置函数适配器
//!
//! 将核心层的内置函数绑定到 Boa JavaScript 引擎

use super::core;
use boa_engine::{
    Context,
    JsNativeError,
    JsResult,
    JsValue,
    NativeFunction,
    js_string,
    object::builtins::JsArray,
};

/// 为 Boa 引擎注册内置函数
pub fn register_builtin_functions(context: &mut Context) -> JsResult<()> {
    // 字符串处理函数
    register_fn(context, "trim", 1, trim)?;
    register_fn(context, "trim_start", 1, trim_start)?;
    register_fn(context, "trim_end", 1, trim_end)?;
    register_fn(context, "lower", 1, lower)?;
    register_fn(context, "upper", 1, upper)?;
    register_fn(context, "replace", 3, replace)?;
    register_fn(context, "split", 2, split)?;
    register_fn(context, "substring", 3, substring)?;
    register_fn(context, "contains", 2, contains)?;
    register_fn(context, "starts_with", 2, starts_with)?;
    register_fn(context, "ends_with", 2, ends_with)?;
    register_fn(context, "length", 1, str_length)?;
    register_fn(context, "index_of", 2, index_of)?;
    register_fn(context, "repeat_str", 2, repeat_str)?;
    register_fn(context, "reverse", 1, reverse_str)?;

    // 正则表达式函数
    register_fn(context, "regex_match", 2, regex_match)?;
    register_fn(context, "regex_replace", 3, regex_replace)?;
    register_fn(context, "regex_find", 2, regex_find)?;
    register_fn(context, "regex_find_all", 2, regex_find_all)?;

    // 编码函数
    register_fn(context, "base64_encode", 1, base64_encode)?;
    register_fn(context, "base64_decode", 1, base64_decode)?;
    register_fn(context, "url_encode", 1, url_encode)?;
    register_fn(context, "url_decode", 1, url_decode)?;
    register_fn(context, "html_encode", 1, html_encode)?;
    register_fn(context, "html_decode", 1, html_decode)?;
    register_fn(context, "hex_encode", 1, hex_encode)?;
    register_fn(context, "hex_decode", 1, hex_decode)?;

    // 哈希函数
    register_fn(context, "md5", 1, md5)?;
    register_fn(context, "sha1", 1, sha1)?;
    register_fn(context, "sha256", 1, sha256)?;

    // 中文处理函数
    register_fn(context, "t2s", 1, t2s)?;
    register_fn(context, "s2t", 1, s2t)?;
    register_fn(context, "to_zh_cn", 1, to_zh_cn)?;
    register_fn(context, "to_zh_tw", 1, to_zh_tw)?;
    register_fn(context, "to_zh_hk", 1, to_zh_hk)?;
    register_fn(context, "to_zh_hant", 1, to_zh_hant)?;
    register_fn(context, "to_zh_hans", 1, to_zh_hans)?;
    register_fn(context, "is_hans", 1, is_hans)?;
    register_fn(context, "to_num_chapter", 1, to_num_chapter)?;
    register_fn(context, "cn_to_num", 1, cn_to_num)?;

    // JSON 处理函数
    register_fn(context, "json_parse", 1, json_parse)?;
    register_fn(context, "json_stringify", 1, json_stringify)?;

    // URL 处理函数
    register_fn(context, "join_url", 2, join_url)?;
    register_fn(context, "get_query_param", 2, get_query_param)?;

    // 工具函数
    register_fn(context, "uuid", 0, uuid)?;
    register_fn(context, "timestamp", 0, timestamp)?;
    register_fn(context, "timestamp_millis", 0, timestamp_millis)?;
    register_fn(context, "log", 1, log)?;

    Ok(())
}

/// 辅助函数: 注册全局函数
fn register_fn(
    context: &mut Context,
    name: &str,
    length: usize,
    func: fn(&JsValue, &[JsValue], &mut Context) -> JsResult<JsValue>,
) -> JsResult<()> {
    context.register_global_builtin_callable(
        js_string!(name),
        length,
        NativeFunction::from_fn_ptr(func),
    )?;
    Ok(())
}

/// 辅助函数: 从参数获取字符串
fn get_string_arg(args: &[JsValue], index: usize, context: &mut Context) -> JsResult<String> {
    args.get(index)
        .ok_or_else(|| JsNativeError::typ().with_message("Missing argument").into())
        .and_then(|v| v.to_string(context))
        .map(|s| s.to_std_string_escaped())
}

/// 辅助函数: 从参数获取可选字符串
fn get_optional_string_arg(
    args: &[JsValue],
    index: usize,
    context: &mut Context,
) -> JsResult<Option<String>> {
    match args.get(index) {
        Some(v) if !v.is_undefined() => {
            let s = v.to_string(context)?.to_std_string_escaped();
            Ok(Some(s))
        }
        _ => Ok(None),
    }
}

/// 辅助函数: 从参数获取整数
fn get_int_arg(args: &[JsValue], index: usize, context: &mut Context) -> JsResult<i64> {
    args.get(index)
        .ok_or_else(|| JsNativeError::typ().with_message("Missing argument").into())
        .and_then(|v| v.to_i32(context))
        .map(|n| n as i64)
}

// ============================================
// 字符串处理函数实现
// ============================================

fn trim(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::trim(&s))))
}

fn trim_start(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::trim_start(&s))))
}

fn trim_end(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::trim_end(&s))))
}

fn lower(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::lower(&s))))
}

fn upper(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::upper(&s))))
}

fn replace(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let from = get_string_arg(args, 1, ctx)?;
    let to = get_string_arg(args, 2, ctx)?;
    Ok(JsValue::from(js_string!(core::replace(&s, &from, &to))))
}

fn split(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let sep = get_string_arg(args, 1, ctx)?;
    let parts = core::split(&s, &sep);
    let arr = JsArray::new(ctx);
    for part in parts {
        arr.push(JsValue::from(js_string!(part)), ctx)?;
    }
    Ok(arr.into())
}

fn substring(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let start = get_int_arg(args, 1, ctx)? as usize;
    let end = get_optional_string_arg(args, 2, ctx)?.and_then(|e| e.parse().ok());
    Ok(JsValue::from(js_string!(core::substring(&s, start, end))))
}

fn contains(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let pattern = get_string_arg(args, 1, ctx)?;
    Ok(JsValue::from(core::contains(&s, &pattern)))
}

fn starts_with(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let prefix = get_string_arg(args, 1, ctx)?;
    Ok(JsValue::from(core::starts_with(&s, &prefix)))
}

fn ends_with(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let suffix = get_string_arg(args, 1, ctx)?;
    Ok(JsValue::from(core::ends_with(&s, &suffix)))
}

fn str_length(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(core::length(&s) as i32))
}

fn index_of(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let pattern = get_string_arg(args, 1, ctx)?;
    Ok(JsValue::from(core::index_of(&s, &pattern) as i32))
}

fn repeat_str(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    let count = get_int_arg(args, 1, ctx)? as usize;
    Ok(JsValue::from(js_string!(core::repeat(&s, count))))
}

fn reverse_str(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::reverse(&s))))
}

// ============================================
// 正则表达式函数实现
// ============================================

fn regex_match(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let pattern = get_string_arg(args, 0, ctx)?;
    let text = get_string_arg(args, 1, ctx)?;
    Ok(JsValue::from(core::regex_match(&pattern, &text)))
}

fn regex_replace(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let text = get_string_arg(args, 0, ctx)?;
    let pattern = get_string_arg(args, 1, ctx)?;
    let replacement = get_string_arg(args, 2, ctx)?;
    Ok(JsValue::from(js_string!(core::regex_replace(
        &text,
        &pattern,
        &replacement
    ))))
}

fn regex_find(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let text = get_string_arg(args, 0, ctx)?;
    let pattern = get_string_arg(args, 1, ctx)?;
    match core::regex_find(&text, &pattern) {
        Some(s) => Ok(JsValue::from(js_string!(s))),
        None => Ok(JsValue::null()),
    }
}

fn regex_find_all(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let text = get_string_arg(args, 0, ctx)?;
    let pattern = get_string_arg(args, 1, ctx)?;
    let results = core::regex_find_all(&text, &pattern);
    let arr = JsArray::new(ctx);
    for s in results {
        arr.push(JsValue::from(js_string!(s)), ctx)?;
    }
    Ok(arr.into())
}

// ============================================
// 编码函数实现
// ============================================

fn base64_encode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::base64_encode(&s))))
}

fn base64_decode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    match core::base64_decode(&s) {
        Ok(decoded) => Ok(JsValue::from(js_string!(decoded))),
        Err(e) => Err(JsNativeError::error().with_message(e).into()),
    }
}

fn url_encode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::url_encode(&s))))
}

fn url_decode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    match core::url_decode(&s) {
        Ok(decoded) => Ok(JsValue::from(js_string!(decoded))),
        Err(e) => Err(JsNativeError::error().with_message(e).into()),
    }
}

fn html_encode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::html_encode(&s))))
}

fn html_decode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::html_decode(&s))))
}

fn hex_encode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::hex_encode(&s))))
}

fn hex_decode(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    match core::hex_decode(&s) {
        Ok(decoded) => Ok(JsValue::from(js_string!(decoded))),
        Err(e) => Err(JsNativeError::error().with_message(e).into()),
    }
}

// ============================================
// 哈希函数实现
// ============================================

fn md5(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::md5(&s))))
}

fn sha1(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::sha1(&s))))
}

fn sha256(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::sha256(&s))))
}

// ============================================
// 中文处理函数实现
// ============================================

fn t2s(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::t2s(&s))))
}

fn s2t(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::s2t(&s))))
}

fn to_zh_cn(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::to_zh_cn(&s))))
}

fn to_zh_tw(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::to_zh_tw(&s))))
}

fn to_zh_hk(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::to_zh_hk(&s))))
}

fn to_zh_hant(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::to_zh_hant(&s))))
}

fn to_zh_hans(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::to_zh_hans(&s))))
}

fn is_hans(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(core::is_hans(&s)))
}

fn to_num_chapter(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(js_string!(core::to_num_chapter(&s))))
}

fn cn_to_num(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    Ok(JsValue::from(core::cn_to_num(&s) as i32))
}

// ============================================
// JSON 处理函数实现
// ============================================

fn json_parse(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let s = get_string_arg(args, 0, ctx)?;
    match serde_json::from_str::<serde_json::Value>(&s) {
        Ok(value) => json_to_js(ctx, &value),
        Err(e) => Err(JsNativeError::error()
            .with_message(format!("JSON parse error: {}", e))
            .into()),
    }
}

fn json_stringify(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let value = args
        .first()
        .ok_or_else(|| JsNativeError::typ().with_message("Missing argument"))?;
    let json_value = js_to_json(value, ctx)?;
    Ok(JsValue::from(js_string!(core::json_stringify(&json_value))))
}

// ============================================
// URL 处理函数实现
// ============================================

fn join_url(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let base = get_string_arg(args, 0, ctx)?;
    let path = get_string_arg(args, 1, ctx)?;
    Ok(JsValue::from(js_string!(core::join_url(&base, &path))))
}

fn get_query_param(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let url = get_string_arg(args, 0, ctx)?;
    let key = get_string_arg(args, 1, ctx)?;
    match core::get_query_param(&url, &key) {
        Some(value) => Ok(JsValue::from(js_string!(value))),
        None => Ok(JsValue::null()),
    }
}

// ============================================
// 工具函数实现
// ============================================

fn uuid(_: &JsValue, _args: &[JsValue], _ctx: &mut Context) -> JsResult<JsValue> {
    Ok(JsValue::from(js_string!(core::uuid())))
}

fn timestamp(_: &JsValue, _args: &[JsValue], _ctx: &mut Context) -> JsResult<JsValue> {
    Ok(JsValue::from(core::timestamp() as i32))
}

fn timestamp_millis(_: &JsValue, _args: &[JsValue], _ctx: &mut Context) -> JsResult<JsValue> {
    // 注意: JavaScript 的 number 可能无法精确表示大数字
    // 但对于当前时间戳来说应该是安全的
    Ok(JsValue::from(core::timestamp_millis() as f64))
}

fn log(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let msg = get_string_arg(args, 0, ctx)?;
    core::log(&msg);
    Ok(JsValue::undefined())
}

// ============================================
// JSON 转换辅助函数
// ============================================

/// 将 serde_json::Value 转换为 Boa JsValue
fn json_to_js(context: &mut Context, value: &serde_json::Value) -> JsResult<JsValue> {
    match value {
        serde_json::Value::Null => Ok(JsValue::null()),
        serde_json::Value::Bool(b) => Ok(JsValue::from(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(JsValue::from(i as i32))
            } else if let Some(f) = n.as_f64() {
                Ok(JsValue::from(f))
            } else {
                Ok(JsValue::null())
            }
        }
        serde_json::Value::String(s) => Ok(JsValue::from(js_string!(s.clone()))),
        serde_json::Value::Array(arr) => {
            let js_arr = JsArray::new(context);
            for item in arr {
                let js_item = json_to_js(context, item)?;
                js_arr.push(js_item, context)?;
            }
            Ok(js_arr.into())
        }
        serde_json::Value::Object(obj) => {
            let js_obj = boa_engine::JsObject::with_null_proto();
            for (key, val) in obj {
                let js_val = json_to_js(context, val)?;
                js_obj.set(js_string!(key.clone()), js_val, false, context)?;
            }
            Ok(js_obj.into())
        }
    }
}

/// 将 Boa JsValue 转换为 serde_json::Value
fn js_to_json(value: &JsValue, context: &mut Context) -> JsResult<serde_json::Value> {
    if value.is_null() || value.is_undefined() {
        Ok(serde_json::Value::Null)
    } else if let Some(b) = value.as_boolean() {
        Ok(serde_json::Value::Bool(b))
    } else if value.is_number() {
        let n = value.to_number(context)?;
        Ok(serde_json::json!(n))
    } else if value.is_string() {
        let s = value.to_string(context)?.to_std_string_escaped();
        Ok(serde_json::Value::String(s))
    } else if value.is_object() {
        let obj = value.as_object().unwrap();
        if obj.is_array() {
            let arr = JsArray::from_object(obj.clone())?;
            let len = arr.length(context)?;
            let mut vec = Vec::with_capacity(len as usize);
            for i in 0..len {
                let item = arr.get(i, context)?;
                vec.push(js_to_json(&item, context)?);
            }
            Ok(serde_json::Value::Array(vec))
        } else {
            let keys = obj.own_property_keys(context)?;
            let mut map = serde_json::Map::new();
            for key in keys {
                // PropertyKey 直接使用 to_string() 获取字符串表示
                let key_str = key.to_string();
                let val = obj.get(key, context)?;
                map.insert(key_str, js_to_json(&val, context)?);
            }
            Ok(serde_json::Value::Object(map))
        }
    } else {
        Ok(serde_json::Value::String(
            value.to_string(context)?.to_std_string_escaped(),
        ))
    }
}

//! Rhai 引擎内置函数适配器
//!
//! 将核心层的内置函数绑定到 Rhai 引擎

use super::core;
use rhai::{Dynamic, Engine, EvalAltResult, Map};

/// 注册所有内置函数到 Rhai 引擎
pub fn register_all(engine: &mut Engine) {
    register_string_functions(engine);
    register_regex_functions(engine);
    register_encoding_functions(engine);
    register_hash_functions(engine);
    register_chinese_functions(engine);
    register_json_functions(engine);
    register_array_functions(engine);
    register_type_functions(engine);
    register_datetime_functions(engine);
    register_url_functions(engine);
    register_util_functions(engine);
}

/// 注册字符串处理函数
fn register_string_functions(engine: &mut Engine) {
    // 基础字符串操作
    engine.register_fn("trim", |s: &str| core::trim(s));
    engine.register_fn("trim_start", |s: &str| core::trim_start(s));
    engine.register_fn("trim_end", |s: &str| core::trim_end(s));
    engine.register_fn("lower", |s: &str| core::lower(s));
    engine.register_fn("upper", |s: &str| core::upper(s));
    engine.register_fn("replace", |s: &str, from: &str, to: &str| {
        core::replace(s, from, to)
    });
    engine.register_fn("split", |s: &str, sep: &str| core::split(s, sep));
    engine.register_fn("join", |arr: rhai::Array, sep: &str| {
        let strs: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
        core::join(&strs, sep)
    });
    engine.register_fn("substring", |s: &str, start: i64| {
        core::substring(s, start as usize, None)
    });
    engine.register_fn("substring", |s: &str, start: i64, end: i64| {
        core::substring(s, start as usize, Some(end as usize))
    });
    engine.register_fn("contains", |s: &str, pattern: &str| {
        core::contains(s, pattern)
    });
    engine.register_fn("starts_with", |s: &str, prefix: &str| {
        core::starts_with(s, prefix)
    });
    engine.register_fn("ends_with", |s: &str, suffix: &str| {
        core::ends_with(s, suffix)
    });
    engine.register_fn("length", |s: &str| core::length(s) as i64);
    engine.register_fn("index_of", |s: &str, pattern: &str| {
        core::index_of(s, pattern)
    });
    engine.register_fn("repeat_str", |s: &str, count: i64| {
        core::repeat(s, count as usize)
    });
    engine.register_fn("reverse", |s: &str| core::reverse(s));
    engine.register_fn("pad_start", |s: &str, len: i64, pad: &str| {
        core::pad_start(s, len as usize, pad)
    });
    engine.register_fn("pad_end", |s: &str, len: i64, pad: &str| {
        core::pad_end(s, len as usize, pad)
    });
}

/// 注册正则表达式函数
fn register_regex_functions(engine: &mut Engine) {
    engine.register_fn("regex_match", |pattern: &str, text: &str| {
        core::regex_match(pattern, text)
    });
    engine.register_fn(
        "regex_replace",
        |text: &str, pattern: &str, replacement: &str| {
            core::regex_replace(text, pattern, replacement)
        },
    );
    engine.register_fn("regex_find", |text: &str, pattern: &str| -> Dynamic {
        core::regex_find(text, pattern)
            .map(Dynamic::from)
            .unwrap_or(Dynamic::UNIT)
    });
    engine.register_fn("regex_find_all", |text: &str, pattern: &str| {
        core::regex_find_all(text, pattern)
    });
    engine.register_fn("regex_captures", |text: &str, pattern: &str| {
        core::regex_captures(text, pattern)
    });
}

/// 注册编码/解码函数
fn register_encoding_functions(engine: &mut Engine) {
    engine.register_fn("base64_encode", |s: &str| core::base64_encode(s));
    engine.register_fn(
        "base64_decode",
        |s: &str| -> Result<String, Box<EvalAltResult>> {
            core::base64_decode(s).map_err(|e| e.into())
        },
    );
    engine.register_fn("url_encode", |s: &str| core::url_encode(s));
    engine.register_fn(
        "url_decode",
        |s: &str| -> Result<String, Box<EvalAltResult>> {
            core::url_decode(s).map_err(|e| e.into())
        },
    );
    engine.register_fn("html_encode", |s: &str| core::html_encode(s));
    engine.register_fn("html_decode", |s: &str| core::html_decode(s));
    engine.register_fn("hex_encode", |s: &str| core::hex_encode(s));
    engine.register_fn(
        "hex_decode",
        |s: &str| -> Result<String, Box<EvalAltResult>> {
            core::hex_decode(s).map_err(|e| e.into())
        },
    );
}

/// 注册哈希/加密函数
fn register_hash_functions(engine: &mut Engine) {
    engine.register_fn("md5", |s: &str| core::md5(s));
    engine.register_fn("sha256", |s: &str| core::sha256(s));
    engine.register_fn("sha1", |s: &str| core::sha1(s));
}

/// 注册中文处理函数
fn register_chinese_functions(engine: &mut Engine) {
    engine.register_fn("t2s", |s: &str| core::t2s(s));
    engine.register_fn("s2t", |s: &str| core::s2t(s));
    engine.register_fn("to_zh_cn", |s: &str| core::to_zh_cn(s));
    engine.register_fn("to_zh_tw", |s: &str| core::to_zh_tw(s));
    engine.register_fn("to_zh_hk", |s: &str| core::to_zh_hk(s));
    engine.register_fn("to_zh_hant", |s: &str| core::to_zh_hant(s));
    engine.register_fn("to_zh_hans", |s: &str| core::to_zh_hans(s));
    engine.register_fn("is_hans", |s: &str| core::is_hans(s));
    engine.register_fn("to_num_chapter", |s: &str| core::to_num_chapter(s));
    engine.register_fn("cn_to_num", |s: &str| core::cn_to_num(s));
    engine.register_fn("num_to_cn", |n: i64| core::num_to_cn(n));
}

/// 注册 JSON 处理函数
fn register_json_functions(engine: &mut Engine) {
    engine.register_fn(
        "json_parse",
        |s: &str| -> Result<Dynamic, Box<EvalAltResult>> {
            core::json_parse(s)
                .map(dynamic_from_json)
                .map_err(|e| e.into())
        },
    );
    engine.register_fn("json_stringify", |d: Dynamic| {
        let value = json_from_dynamic(d);
        core::json_stringify(&value)
    });
    engine.register_fn("json_stringify_pretty", |d: Dynamic| {
        let value = json_from_dynamic(d);
        core::json_stringify_pretty(&value)
    });
}

/// 注册数组处理函数
fn register_array_functions(engine: &mut Engine) {
    engine.register_fn("array_first", |arr: rhai::Array| -> Dynamic {
        arr.first().cloned().unwrap_or(Dynamic::UNIT)
    });
    engine.register_fn("array_last", |arr: rhai::Array| -> Dynamic {
        arr.last().cloned().unwrap_or(Dynamic::UNIT)
    });
    engine.register_fn("array_at", |arr: rhai::Array, index: i64| -> Dynamic {
        let len = arr.len() as i64;
        let idx = if index < 0 { len + index } else { index };
        if idx >= 0 && idx < len {
            arr.get(idx as usize).cloned().unwrap_or(Dynamic::UNIT)
        } else {
            Dynamic::UNIT
        }
    });
    engine.register_fn(
        "array_slice",
        |arr: rhai::Array, start: i64, end: i64| -> rhai::Array {
            let len = arr.len() as i64;
            let start = if start < 0 {
                (len + start).max(0) as usize
            } else {
                start.min(len) as usize
            };
            let end = if end < 0 {
                (len + end).max(0) as usize
            } else {
                end.min(len) as usize
            };
            if start >= end {
                vec![]
            } else {
                arr[start..end].to_vec()
            }
        },
    );
    engine.register_fn("array_unique", |arr: rhai::Array| -> rhai::Array {
        let mut seen = std::collections::HashSet::new();
        arr.into_iter()
            .filter(|v| seen.insert(v.to_string()))
            .collect()
    });
}

/// 注册类型转换函数
fn register_type_functions(engine: &mut Engine) {
    engine.register_fn("to_int", |s: &str| -> Dynamic {
        core::to_int(s).map(Dynamic::from).unwrap_or(Dynamic::UNIT)
    });
    engine.register_fn("to_float", |s: &str| -> Dynamic {
        core::to_float(s)
            .map(Dynamic::from)
            .unwrap_or(Dynamic::UNIT)
    });
    engine.register_fn("to_string", |d: Dynamic| d.to_string());
    engine.register_fn("to_bool", |s: &str| core::to_bool(s));
}

/// 注册日期时间函数
fn register_datetime_functions(engine: &mut Engine) {
    engine.register_fn("timestamp", core::timestamp);
    engine.register_fn("timestamp_millis", core::timestamp_millis);
    engine.register_fn("format_timestamp", |ts: i64, format: &str| {
        core::format_timestamp(ts, format)
    });
    engine.register_fn("parse_date", |s: &str, format: &str| -> Dynamic {
        core::parse_date(s, format)
            .map(Dynamic::from)
            .unwrap_or(Dynamic::UNIT)
    });
}

/// 注册 URL 处理函数
fn register_url_functions(engine: &mut Engine) {
    engine.register_fn("parse_url", |url: &str| -> Map {
        let result = core::parse_url(url);
        result
            .into_iter()
            .map(|(k, v)| (k.into(), Dynamic::from(v)))
            .collect()
    });
    engine.register_fn("join_url", |base: &str, path: &str| {
        core::join_url(base, path)
    });
    engine.register_fn("get_query_param", |url: &str, key: &str| -> Dynamic {
        core::get_query_param(url, key)
            .map(Dynamic::from)
            .unwrap_or(Dynamic::UNIT)
    });
    engine.register_fn("set_query_param", |url: &str, key: &str, value: &str| {
        core::set_query_param(url, key, value)
    });
}

/// 注册工具函数
fn register_util_functions(engine: &mut Engine) {
    engine.register_fn("uuid", core::uuid);
    engine.register_fn("random_int", |min: i64, max: i64| {
        core::random_int(min, max)
    });
    engine.register_fn("log", |msg: &str| core::log(msg));
    engine.register_fn("warn", |msg: &str| core::warn(msg));
    engine.register_fn("error", |msg: &str| core::error(msg));
}

/// 将 serde_json::Value 转换为 Rhai Dynamic
fn dynamic_from_json(value: serde_json::Value) -> Dynamic {
    match value {
        serde_json::Value::Null => Dynamic::UNIT,
        serde_json::Value::Bool(b) => Dynamic::from(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Dynamic::from(i)
            } else if let Some(f) = n.as_f64() {
                Dynamic::from(f)
            } else {
                Dynamic::UNIT
            }
        }
        serde_json::Value::String(s) => Dynamic::from(s),
        serde_json::Value::Array(arr) => {
            Dynamic::from(arr.into_iter().map(dynamic_from_json).collect::<Vec<_>>())
        }
        serde_json::Value::Object(obj) => {
            let map: Map = obj
                .into_iter()
                .map(|(k, v)| (k.into(), dynamic_from_json(v)))
                .collect();
            Dynamic::from(map)
        }
    }
}

/// 将 Rhai Dynamic 转换为 serde_json::Value
fn json_from_dynamic(value: Dynamic) -> serde_json::Value {
    if value.is_unit() {
        serde_json::Value::Null
    } else if let Ok(b) = value.as_bool() {
        serde_json::Value::Bool(b)
    } else if let Ok(i) = value.as_int() {
        serde_json::Value::Number(i.into())
    } else if let Ok(f) = value.as_float() {
        serde_json::json!(f)
    } else if let Ok(s) = value.clone().into_string() {
        serde_json::Value::String(s)
    } else if let Ok(arr) = value.clone().into_typed_array::<Dynamic>() {
        serde_json::Value::Array(arr.into_iter().map(json_from_dynamic).collect())
    } else if let Some(map) = value.clone().try_cast::<Map>() {
        let obj: serde_json::Map<String, serde_json::Value> = map
            .into_iter()
            .map(|(k, v)| (k.to_string(), json_from_dynamic(v)))
            .collect();
        serde_json::Value::Object(obj)
    } else {
        serde_json::Value::String(value.to_string())
    }
}

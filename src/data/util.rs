use http::header::{USER_AGENT, ACCEPT_ENCODING, ACCEPT, HOST, ACCEPT_LANGUAGE, REFERER, ACCEPT_CHARSET};

pub fn make_headers() -> http::header::HeaderMap {
    let mut headers = http::header::HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".parse().unwrap());
    headers.insert(ACCEPT_ENCODING, "gzip, deflate, sdch".parse().unwrap());
    headers.insert(ACCEPT, "*/*".parse().unwrap());
    //headers.insert(ACCEPT_LANGUAGE, "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".parse().unwrap());
    headers.insert(HOST, "hq.sinajs.cn".parse().unwrap());
    headers.insert(REFERER, "https://finance.sina.com.cn/realstock/company/sh600905/nc.shtml".parse().unwrap());
    headers.insert(ACCEPT_CHARSET, "utf-8".parse().unwrap());
    headers
}
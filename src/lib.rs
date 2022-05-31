use std::io::{self, BufRead, Write, BufWriter};
use sxd_document::parser;
use sxd_xpath::{evaluate_xpath};

type MyResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> MyResult {
    let xpaths = get_xpaths();
    let input = io::stdin();
    let mut input = io::BufReader::new(input.lock());
    let mut xml = String::new();
    loop {
        let mut line = String::new();
        let bytes = input.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        xml.push_str(&line);
    }

    let package = parser::parse(&xml)?;
    let document = package.as_document();
    let mut is_first_elm = true; // print が2個めの要素になったらはじめて改行を加える
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    for xpath in xpaths.iter() {
        let value = evaluate_xpath(&document, &xpath)?;
        if is_first_elm {
            is_first_elm = false;
        } else {
            out.write(b"\n")?;
        }
        out.write(value.string().as_bytes())?;
    }
    out.flush()?;
    Ok(())
}

fn get_xpaths() -> Vec<String> {
    let mut xpaths: Vec<String> = Vec::new();
    let args = std::env::args();
    if args.len() < 2 {
        return xpaths;
    }
    let mut is_cmd_name = true;
    for arg in args {
        if is_cmd_name {
            is_cmd_name = false;
            continue;
        }
        let arg = if arg.starts_with("/") {
            arg.to_string()
        } else {
            // 先頭に `"//"` を補って検索用Xpath指定にする
            // コマンドラインでWindowsでの使用も考えると、
            // 引数に (直感的には)`"//message"` と書きたいときでも 
            // `"///message"` を書かなければならない
            // かと思うと `"//image[1]"` はこのままでよい。
            // これはただ困惑のみ呼ぶ
            // であれば引数で `"message" "image[1]"` と書けたほうが直感的
            format!("//{}", arg)
        };
        xpaths.push(arg);
    }
    xpaths
}

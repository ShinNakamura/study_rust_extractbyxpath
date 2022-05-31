# XMLからXpath指定でデータを抜き取る

Rust 勉強中の自習課題。

いろいろ雑です。
クローン、利用は自己責任でお願い致します。

## 概要

こんなXML(UTF8)に対してXpathを指定して該当する要素の文字列表現を得たい。

`tests/input.xml`
```xml
<?xml version="1.0" encoding="UTF-8"?>
<result>
    <message>OK</message>
    <description>
        <![CDATA[<p>
            <div class="foo">foo</div>
        </p>]]></description>
    <comment>&lt;p>bar&lt;/p></comment>
    <images><image>image_1.jpg</image><image>image_2.jpg</image></images>
</result>
```

コマンドラインの引数でXpathを指定。
その際、Xpathの先頭につける`//`は省略する。

```sh
cargo run -- "message" <tests/input.xml
# OK

cargo run -- "image[1]" <tests/input.xml
# image_1.jpg
```

## 参考

[Crate sxd_xpath](https://docs.rs/sxd-xpath/0.4.2/sxd_xpath/)

[ShinNakamura/Rust_xpath_test](https://github.com/ShinNakamura/Rust_xpath_test/blob/master/src/main.rs)

[Command-Line Rust (English Edition)](https://www.amazon.co.jp/gp/product/B09QFQ3Y64/)

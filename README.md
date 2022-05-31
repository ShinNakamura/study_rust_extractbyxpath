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
    <categories>
        <categoryInfo>
            <categoryId>100</categoryId>
            <categoryName>hoge</categoryName>
        </categoryInfo>
        <categoryInfo>
            <categoryId>101</categoryId>
            <categoryName>piyo</categoryName>
        </categoryInfo>
    </categories>
</result>
```

コマンドラインの引数でXpathを指定。
その際、Xpathの先頭につける`//`は省略可能とする。

```sh
cargo run -- "message" <tests/input.xml
# OK

cargo run -- "image[1]" <tests/input.xml
# image_1.jpg

cargo run -- "categoryInfo[2]/categoryName" <tests/input.xml
# piyo
```

なお、下記のように存在しない要素や範囲外の番号を指定しても
コマンドラインでの実行上は

- 正常終了
- 単になにも返らない(空欄が返る)

とする。

```sh
cargo run -- "aaa" <tests/input.xml
cargo run -- "image[1000]" <tests/input.xml
```

## 参考

[Crate sxd_xpath](https://docs.rs/sxd-xpath/0.4.2/sxd_xpath/)

[ShinNakamura/Rust_xpath_test](https://github.com/ShinNakamura/Rust_xpath_test/blob/master/src/main.rs)

[Command-Line Rust (English Edition)](https://www.amazon.co.jp/gp/product/B09QFQ3Y64/)


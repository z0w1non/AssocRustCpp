# AssocRustCpp

## 概要
これはC++に慣れ親しんだ開発者がRustを勉強するために開発した、言語間単体テストツールである。
C++とRustのコードブロックを含むマークダウン形式のファイルを元に、C++とRustをそれぞれビルド・実行し、その実行結果が同一となるか検証する。
つまり「C++のコードは、Rustのこのようなコードと対応する」という関連性を定義したファイルを元に単体テストを実行し、それらの実行結果の整合性を実証的に検証する。
本プロジェクトはRustで開発されており、その開発自体がRustの学習を兼ねている。

例えば以下のようなマークダウン形式のファイルを元に単体テストを実行する。

````txt(hello_world.txt)
```rs
fn main() {
    println!("Hello, World!");
}
```

```cpp
#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
	return 0;
}
```
````

それぞれの単体テストは `<test_name>.txt` という名前のファイルにより定義される。
一連の単体テストは、テスト名で昇順ソートした順番で実行される。

## 技術的要素
* pulldown_cmark によるマークダウンファイル解析
* tempfile によるRAIIに基づく一時ファイル管理
* WalkDir による再帰的なファイルツリー探索
* vcvars64.bat によりネイティブC++プログラムを実行時コンパイル

## 実行方法
testフォルダ配下に含まれる全てのテストを実行
```bash
cargo run
```

特定のフォルダ配下に含まれる全てのテストを実行
```bash
cargo run -- test/foo
```

特定のテスト実行
```bash
cargo run -- test/hello_world
```

## 実行結果
```bash
PS C:\AssocRustCpp> cargo run
   Compiling AssocRustCpp v0.1.0 (C:\AssocRustCpp)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.68s                                                  
     Running `target\debug\AssocRustCpp.exe`
Test hello_world [PASSED]
Output: Hello, World!
```

## 環境構築
Windows環境でVS Codeによる開発を想定する。

[rustup](https://rust-lang.org/ja/tools/install/) をインストールする。

VS Codeに以下の拡張機能をインストールする。
* rust-analyzer
* Even Better TOML

VS Codeのsettings.jsonに下記を設定してターミナルの文字化けを回避する。

```
{
    "terminal.integrated.defaultProfile.windows": "PowerShell",
    "terminal.integrated.profiles.windows": {
        "PowerShell": {
            "source": "PowerShell",
            "args": [
                "-NoExit",
                "-Command",
                "chcp 65001"
            ],
        }
    }
}
```

## 備忘録
* フォーマッター適用: Shift + Alt + F
* `WalkDir::sort_by_file_name` による兄弟間の名前ソートはメモリ効率が良い。

## TODO
単体テストの拡充
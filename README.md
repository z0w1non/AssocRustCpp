# AssocRustCpp

## 概要
これはC++に慣れ親しんだ開発者がRustを勉強するために開発したコードである。
testフォルダ配下に存在する、C++とRustのコードブロックを含むマークダウン形式のファイルを元に、C++とRustをそれぞれビルド・実行し、その実行結果が同一となるか検証する。
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

[Rustup](https://rust-lang.org/ja/tools/install/) をインストールする。

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
フォーマッター適用: Shift + Alt + F

## TODO
単体テストの拡充
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

### 対応関係
| Rust の概念 | C++ の概念 | 役割 |
| :-- | :-- | :-- |
| `new()` | コンストラクタ | 慣習的な名前で定義された関数。ヒープにメモリ確保するnewとは根本的に意味が異なる。戻り値は型により様々であり、参照と値どちらの場合もある。 |
| `from()` | `explicit` な変換コンストラクタ | 異なる型の引数を元に新しく書き換え可能な実体をヒープに作成する。戻り値は所有権ありの値。Rustには暗黙の型変換が基本的にない。`From Trait` として標準化されている。 |
| `into` | `static_cast<T>()` | `from()` を利用して型変換を行う。戻り値が何の `into()` を呼び出せばいいかは、左辺の型注釈や、関数の引数のシグネチャに基づいて、逆算的に判断される。 `Into Trait` として標準化されている。 |
| `String` | `std::string` | ヒープを所有する可変長文字列 |
| `&str` | `std::string_view`, `const char *` | 既存の文字列の一部に対する参照 |
| `Vec<T>` | `std::vector<T>` | ヒープを所有する可変長配列 |
| `&[T]` | `std::span<T>` (C++20以降) | 既存の配列の一部に対する参照 |
| `Option<T>` | `std::optional` | 正常系として無効値になる可能性がある値を表現する。 |
| `None` | `nullptr`, `std::nullopt` | 無効値を表現する。 |
| `Result<T, E>` | `std::variant<T, E>`, `std::expected<T, E>` (C++23以降) | エラーが発生する可能性のある結果に最適化された列挙型。 |
| `Box<T>` | `std::unique_ptr<T>` | 単一の所有者が所有権を管理する。 |
| `Rc<T>` | `std::shared_ptr<T>` | 参照カウンタ方式で複数の所有者が所有権を管理する。非スレッドセーフ。 |
| `Arc<T>` | `std::shared_ptr<T>` | アトミックな参照カウンタ方式で複数の所有者が所有権を管理する。スレッドセーフ。 |
| `drop(x)` | `delete x`, あるいはスコープ離脱に伴う暗黙的なデストラクタ呼び出し | リソースを明示的・暗黙的に解放する。 |
| `struct`, `impl` | `class` | データ構造の定義と、そのメンバ関数の定義 |
| `Trait` | 純粋仮想関数のみを含む抽象クラス | 振る舞いの定義 |
| `impl Trait` | 静的ポリモーフィズム, テンプレート | コンパイル時に静的に型が決定する高速な多態性 |
| `dyn Trait` | 動的ポリモーフィズム, 仮想関数テーブル(vtable) | 実行時にポインタを経由してメンバ関数を呼び出す多態性 |
| `?` (エラー委譲) | `T op(std::optional<T> & opt) { if (!opt) return std::nullopt; return *opt; }` (擬似コード) | `Option`, `Result` などのから有効値を取り出し、無効値であれば早期リターンする。`?` を適用する対象の型は、関数全体の戻り値の型に一致している必要がある。内部では `std::ops::Try Trait` を利用している。 |

* フォーマッター適用: Shift + Alt + F
* 所有権の不要な要求を避けるため、以下の置き換えを検討する。さもなければ、関数を呼び出す側はムーブして所有権を関数に渡すか、わざわざ `.clone()` して複製を渡さなければならなくなる。
    * `PathBuf` -> `&Path`
    * `String` -> `&str`
    * `OsString` -> `&OsStr`
    * `Vec<T>` -> `&[T]`
* `From` が実装されていれば、`Into` はコンパイラにより自動的に実装されるため、`From` を優先して実装する。

* `WalkDir::sort_by_file_name` による兄弟間の名前ソートはメモリ効率が良い。
* into関連のパラダイムの相違
    * C++では戻り値の型だけが異なる関数のオーバーロードは許容されずコンパイルエラーになる。C++のオーバーロード解決は原則的に引数の型からどの関数を呼び出すか決める。すなわち、左辺が何であろうと右辺の解決に影響しない。
    * 対するRustは、全ての変数の型を未知数とした連立方程式を解くことで型推論を行う。これに伴い左辺や後続の処理の型情報が、どの関数を呼び出すかに遡って決定することがある。特に `into()` 関数のオーバーロード解決においてそれが顕著である。
    * C++からRustのこの仕様を見ると、未来の出来事が過去を決めるかのような奇妙な感覚に陥る。

## TODO
単体テストの拡充
### テストの準備
- `cargo compete init atcoder` をルートで実行し、compete.toml などの必要ファイルを作成
- `cargo compete new abc189` でテストごとにフォルダを作成. この時テストケースやフォルダを自動作成.
- 作成したフォルダでテストを実行. `cargo compete test a`
- `cargo compete submit a` で提出
- [参考](https://qiita.com/okaponta_/items/7e82de5d1f78f547fe4b)

### Compro useful link
- [Overflow など](https://qiita.com/garkimasera/items/c5e06de1a7c66aa7652a)
- [Option](https://doc.rust-lang.org/std/option/) and [usecase](https://zenn.dev/toga/books/rust-atcoder/viewer/37-option)

### Error
- `Blocking waiting for file lock on package cache`(再コンパイルで辛いが)以下のコマンドを叩く:
  - `$ cargo clean`
  - `$ rm ~/.cargo/.package-cache`
- Try this first: `$ sudo pkill rls cargo`

### Check List
- [itertools](https://docs.rs/itertools/0.10.3/itertools/)

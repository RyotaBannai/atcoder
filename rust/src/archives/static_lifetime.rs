// Make a constant with `'static` lifetime.
// `'static`ライフタイムを持つ定数を作成

static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

pub fn main() {
    {
        // Make a `string` literal and print it:
        // 文字列リテラルを用いて変数を作成し、プリントする
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference
        // can no longer be used, but the data remains in the binary.
        // `static_string`がスコープから抜けると、参照は使用することが
        // できなくなるが、データはバイナリ中に残る。
    }

    {
        // Make an integer to use for `coerce_static`:
        // `coerce_static`関数を呼び出すために、整数を作成
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        // `NUM`を`lifetime_num`のライフタイムへと圧縮. ここで使用する用途において、NUM の lifetime を短くすることができる
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM); // ここでも使える
}

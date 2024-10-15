// TODO: マクロの定義をモジュール外に出すことなくコンパイルエラーを修正してください。
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}

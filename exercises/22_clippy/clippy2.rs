fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: この部分をClippyに従い修正してください。
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}

use unic_langid_macros::langid;
fn main() {
    let id = langid!("de-Latn-DE");
    println!("{:?}", id);
}
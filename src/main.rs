use extism::{Manifest, Plugin, Wasm};

fn main() {
    println!("Let's greet");
    let url = Wasm::url(
        "https://github.com/jaybarbhaiya/my-plugin/releases/download/v1.0.0/my_plugin.wasm",
    );
    let manifest = Manifest::new([url]);
    let mut plugin = Plugin::new(&manifest, [], true).unwrap();

    let res: String = plugin.call("greet", "Jay").unwrap();
    println!("{}", res);
}

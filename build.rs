extern crate cc;

fn main() {
    let debug = match std::env::var("PROFILE") {
        Ok(s) => match s.as_str() {
            "debug" => true,
            _ => false,
        },
        _ => false,
    };

    println!("cargo:rerun-if-changed=src/runtime");

    cc::Build::new()
        .cpp(true)
        .file("src/runtime/lib.cc")
        .debug(debug)
        .compile("runtime");
}

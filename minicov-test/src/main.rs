fn foo() {
    println!("a");
}
fn bar() {
    println!("b");
}

fn do_stuff(x: bool) {
    if x {
        foo()
    } else {
        bar()
    }
}

fn main() {
    do_stuff(false);
    let coverage = minicov::capture_coverage();
    std::fs::write("output.profraw", coverage).unwrap();
}

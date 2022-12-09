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
    let mut coverage = vec![];
    unsafe {
        minicov::capture_coverage(&mut coverage).unwrap();
    }
    std::fs::write("output.profraw", coverage).unwrap();
}

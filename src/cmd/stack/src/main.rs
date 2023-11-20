use backtrace::Backtrace;
use std::fmt::Debug;
use stdext::function_name;

fn main() {
    // test1();
    // test2();
    // test3();
    test4();
}

fn test4() {
    let info = std::panic::Location::caller();
    println!("{info}");
}

fn test3() {
    let stack = std::backtrace::Backtrace::capture();
    let status = stack.status();
    println!("{:?}", status);
    println!("{}", stack);
}

fn dump_stack_test() {
    let bt = Backtrace::new();
    println!("backtrace dump start ===============");
    println!("{:?}", bt);
}

fn test_func() {
    println!("function name is={}", function_name!());
    dump_stack_test();
}

fn test1() {
    test_func();
}

fn test2() {
    foo();
}

fn foo() {
    bar()
}
fn bar() {
    baz()
}
fn baz() {
    print()
}

#[cfg(target_pointer_width = "32")]
const HEX_WIDTH: usize = 10;
#[cfg(target_pointer_width = "64")]
const HEX_WIDTH: usize = 20;

fn print() {
    let mut cnt = 0;
    backtrace::trace(|frame| {
        let ip = frame.ip();
        print!("frame #{:<2} - {:#02$x}", cnt, ip as usize, HEX_WIDTH);
        cnt += 1;

        let mut resolved = false;
        backtrace::resolve(frame.ip(), |symbol| {
            if !resolved {
                resolved = true;
            } else {
                print!("{}", vec![" "; 7 + 2 + 3 + HEX_WIDTH].join(""));
            }

            if let Some(name) = symbol.name() {
                print!(" - {}", name);
            } else {
                print!(" - <unknown>");
            }
            if let Some(file) = symbol.filename() {
                if let Some(l) = symbol.lineno() {
                    print!("\n{:13}{:4$}@ {}:{}", "", "", file.display(), l, HEX_WIDTH);
                }
            }
            println!("");
        });
        if !resolved {
            println!(" - <no info>");
        }
        true // keep going
    });
}

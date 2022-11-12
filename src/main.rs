fn main() {
    println!("Hello, world!");
}

thread_local! {
    static CURRENT: Current = Current::new();
}

struct Current {
    aaa: u64,
}

impl Current {
    fn new() -> Self {
        println!("--> Current new");
        Self { aaa: 1111 }
    }

    fn do_something(&self) {
        CONTEXT.with(|context| {
            println!("call CONTEXT: {} do something", context.bbb);
        });
    }
}

impl Drop for Current {
    fn drop(&mut self) {
        println!("Current drop -->");

        CONTEXT.with(|context| {
            println!("in Current drop, call CONTEXT: {}", context.bbb);
        });
    }
}

thread_local! {
    static CONTEXT: Context = Context::new();
}

struct Context {
    bbb: u64,
}

impl Context {
    fn new() -> Self {
        println!("--> Context new");
        Self { bbb: 2222 }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Context drop -->");
    }
}

#[cfg(test)]
mod tests {
    use crate::CURRENT;

    #[test]
    fn test_local_thread() {
        CURRENT.with(|current| {
            println!("current:{}", current.aaa);
            current.do_something();
        });
    }
}

#![feature(fn_delegation)]
//~^ WARN the feature `fn_delegation` is incomplete and may not be safe to use and/or cause compiler crashes

extern "C" {
    fn a() {
    //~^ ERROR incorrect function inside `extern` block
        reuse foo {}
        //~^ ERROR `reuse` is not allowed here
    }
}

fn main() {}

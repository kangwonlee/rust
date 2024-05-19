//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

fn changer<'a>(mut things: Box<dyn Iterator<Item=&'a mut u8>>) {
    for item in *things { *item = 0 }
    //[current]~^ ERROR `dyn Iterator<Item = &'a mut u8>` is not an iterator
    //[next]~^^ ERROR `dyn Iterator<Item = &'a mut u8>` is not an iterator
    //[next]~| ERROR type `<dyn Iterator<Item = &'a mut u8> as IntoIterator>::Item` cannot be dereferenced

    // FIXME(-Znext-solver): these error messages are horrible and have to be
    // improved before we stabilize the new solver.
}

fn main() {}

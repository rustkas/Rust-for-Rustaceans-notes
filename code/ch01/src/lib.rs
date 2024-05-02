#[test]
fn ex01() {
    fn cache(input: &i32, sum: &mut i32) {
        *sum = *input + *input;
        assert_eq!(*sum, 2 * *input);
    }
    cache(&2, &mut 2);
}

#[test]
fn ex02() {
    fn noalias(input: &i32, output: &mut i32) {
        if *input == 1 {
            *output = 2;
        }
        if *input != 1 {
            *output = 3;
        }
    }
    let mut output = 2;
    noalias(&2, &mut output);
    println!("output = {output}");
}

#[test]
fn ex03() {
    fn replace_with_84(s: &mut Box<i32>) {
        // this is not okay, as *s would be empty:
        // let was = *s;
        // but this is:
        let was = std::mem::take(s);
        // so is this:
        *s = was;
        // we can exchange values behind &mut:
        let mut r = Box::new(84);
        std::mem::swap(s, &mut r);
        assert_ne!(*r, 84);
    }
    let mut s = Box::new(42);
    replace_with_84(&mut s);
    println!("s = {}", s);
}

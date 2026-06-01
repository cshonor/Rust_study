fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[derive(Debug, PartialEq, Eq)]
struct Wrapper(u32);

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    println!("--- function pointer `fn` ---");
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    println!("--- map: closure vs function/method item ---");
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings1: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings2: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(list_of_strings1, list_of_strings2);
    println!("map(ToString::to_string) ok: {:?}", list_of_strings2);

    println!("--- tuple struct / enum variant constructors as functions ---");
    let wrappers: Vec<Wrapper> = (0u32..3).map(Wrapper).collect();
    assert_eq!(wrappers, vec![Wrapper(0), Wrapper(1), Wrapper(2)]);
    println!("wrappers = {:?}", wrappers);

    let statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    let _ = Status::Stop;
    assert_eq!(
        statuses,
        vec![
            Status::Value(0),
            Status::Value(1),
            Status::Value(2),
            Status::Value(3),
            Status::Value(4)
        ]
    );
    println!("statuses = {:?}", statuses);

    println!("--- returning a closure (trait object) ---");
    let f = returns_closure();
    println!("f(10) = {}", f(10));
}


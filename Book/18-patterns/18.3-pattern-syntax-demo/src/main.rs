//! 第 18 章「模式语法」示例汇总（与书中编号对应处见注释）。

fn literals() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn example_18_11_match_shadow() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y:?}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

fn or_and_range() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructure_struct() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn example_18_14_point_axes() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn example_18_15_message() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move x={x} y={y}"),
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => println!("Change color r={r} g={g} b={b}"),
    }
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn example_18_16_nested_enum() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB {r},{g},{b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV {h},{s},{v}");
        }
        _ => {}
    }
}

fn nested_tuple_and_struct() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet={feet}, inches={inches}, x={x}, y={y}");
}

fn foo(_: i32, y: i32) {
    println!("foo: only uses y={y}");
}

fn example_18_18_settings() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");
}

fn example_18_19_tuple_ignore() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}"),
    }
}

fn example_18_22_if_let_underscore_no_move() {
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}

fn example_18_23_point_rest() {
    #[allow(dead_code)]
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {x}"),
    }
}

fn example_18_24_tuple_first_last() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("Some numbers: {first}, {last}"),
    }
}

fn match_guards() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {x}"),
        Some(x) => println!("{x}"),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

enum HelloMsg {
    Hello { id: i32 },
}

fn example_18_29_at_binding() {
    let msg = HelloMsg::Hello { id: 5 };
    match msg {
        HelloMsg::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        HelloMsg::Hello { id: 10..=12 } => println!("Found an id in another range"),
        HelloMsg::Hello { id } => println!("Found some other id: {id}"),
    }
}

fn main() {
    println!("--- literals ---");
    literals();

    println!("--- 18-11 ---");
    example_18_11_match_shadow();

    println!("--- | and ..= ---");
    or_and_range();

    println!("--- struct ---");
    destructure_struct();
    example_18_14_point_axes();

    println!("--- 18-15 Message ---");
    example_18_15_message();

    println!("--- 18-16 nested ---");
    example_18_16_nested_enum();

    println!("--- nested tuple ---");
    nested_tuple_and_struct();

    println!("--- foo(_, y) ---");
    foo(3, 4);

    println!("--- 18-18 ---");
    example_18_18_settings();

    println!("--- 18-19 ---");
    example_18_19_tuple_ignore();

    println!("--- 18-22 Some(_) ---");
    example_18_22_if_let_underscore_no_move();

    println!("--- 18-23 Point {{ x, .. }} ---");
    example_18_23_point_rest();

    println!("--- 18-24 (first, .., last) ---");
    example_18_24_tuple_first_last();

    println!("--- match guards ---");
    match_guards();

    println!("--- 18-29 @ ---");
    example_18_29_at_binding();
}

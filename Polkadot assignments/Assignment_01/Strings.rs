fn main() {
    q1();
    q2();
    q2_2();
    q3();
    q4();
    q5();
    q6();
    q7();
    q7_2();
    q8();
    q8_2();
    q8_3();
    q9();
    q10();
    q11();
    q12();
}


fn q1() {
    let s: &str = "hello, world";
    println!("Success!");
}

fn q2() {
    let s: Box<str> = "hello, world".into();
    greetings_1(&s)
}
fn greetings_1(s: &str) {
    println!("{}",s)
}
fn q2_2() {
    let s: Box<&str> = "hello, world".into();
    greetings(*s)
}
fn greetings_2(s: &str) {
    println!("{}", s);
}

fn q3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');
 
    assert_eq!(s, "hello, world!");
    println!("Success!");
}
fn q4() {
    let mut s = String::from("hello");
     s.push(',');
     s.push_str(" world");
     s += "!";
 
     println!("{}", s)
}
fn q5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");
 
    assert_eq!(s1, "I like cats");
    println!("Success!");
}
fn q6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}
fn q7() {
    let s = "hello, world".to_string();
    greetings_3(s)
}
fn greetings_3(s: String) {
    println!("{}",s)
}
fn q7_2() {
    let s = String::from("hello, world");
    greetings_4(s)
}
fn greetings_4(s: String) {
    println!("{}",s)
}
fn q8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;
    println!("Success!");
}
fn q8_2() {
    let s = "hello, world";
    let s1: &str = s;
}
fn q8_3() {
    let s = "hello, world".to_string();
    let s1: String = s;
}
fn q9() {
    // You can use escapes to write bytes by their hexadecimal values
    // fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

   let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
fn q10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    // modify below line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let  delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // fill the blank
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");
    println!("Success!");
}
fn q11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; 
    assert_eq!(h, "h");

    let h1 = &s1[3..6];
    assert_eq!(h1, "中");
    println!("Success!");
}
fn q12() {
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
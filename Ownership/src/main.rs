fn main() {
    {
        //sはここでは有効でない
        let s = "hello"; //sはここから有効
                         //sで作業する
    } //このスコープは終わり。もうsは有効出ない

    {
        let s = String::from("hello");

        let mut s = String::from("hello");
        s.push_str(", world!"); //push_str()関数は、リテラルをStringに付け加える
        println!("{}", s);
    }

    {
        let s = String::from("hello"); // sがスコープに入る

        takes_ownership(s); // sの値が関数にムーブされここではもう有効ではない

        let x = 5; // xがスコープに入る

        makes_copy(x); // xも関数にムーブされるが、
                       // i32はCopyなので、この後にxを使っても問題ない
    }

    {
        let s1 = gives_ownership(); // gives_ownershipは、戻り値をs1にムーブする

        let s2 = String::from("hello"); // s2がスコープに入る

        let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}", s2, len);
    }

    {
        let s1 = String::from("hello");

        let len = calculate_length_point(&s1);

        println!("The length of '{}' is  {} ", s1, len);
    }

    {
        let mut s = String::from("hello");

        change(&mut s);
    }

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
    }

    {
        let my_string = String::from("hello world");

        let word = first_word(&my_string[..]);

        let my_string_literal = "hello world";

        let word = first_word(&my_string_literal[..]);

        let word = first_word(my_string_literal);
    }
}

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn gives_ownership() -> String {
    // gives_ownershipは、戻り値を
    // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string // some_stringが返され、呼び出し元関数にムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String {
    // a_stringがスコープに入る。

    a_string // a_stringが返され、呼び出し元関数にムーブされる
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_point(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

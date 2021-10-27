use std::fs::File;
use std::io::Write;

const NUM_NAME: [char; 5] = ['个', '十', '百', '千', '万'];

fn generate(i: i32) -> String {
    let s = i.to_string();
    let l = s.len();
    let mut r = String::new();
    r += format!("    case {}:\n", i).as_str();
    r += format!("        cout << \"是 {} 位数\" << endl;\n", l).as_str();
    for (l, c) in s.chars().rev().enumerate() {
        r += format!("        cout << \"{}位数是 {}\" << endl;\n", NUM_NAME[l], c).as_str();
    }
    r += format!(
        "        cout << \"倒过来是 {}\" << endl;\n",
        s.chars().rev().collect::<String>()
    )
    .as_str();
    r += format!("        break;\n").as_str();
    r
}

fn main() {
    let mut file = File::create("main.cpp").expect("文件创建失败!");
    file.write_all(
        "#include <iostream>
using namespace std;
int main()
{
    cout << \"请给出一个不多于5位数的整数:\";
    int x;
    cin >> x;
    switch (x) {\n"
            .as_bytes(),
    )
    .expect("文件写入错误!");
    for i in 0..100_000 {
        file.write_all(generate(i).as_bytes())
            .expect("文件写入错误!");
    }
    file.write_all(
        "    }
    cout << \"Code generated by rust.\" << endl;
}\n"
        .as_bytes(),
    )
    .expect("文件写入错误!");
    println!("Code generated.");
}

fn main() {
    //ตัวแปรแบบ Immutable
    let x = 5;
    println!("ค่าของ x: {}", x);

    //ตัวแปรแบบ Mutable (เปลี่ยนค่าได้)
    let mut y = 10;
    println!("ค่าเริ่มต้นของ y: {}", y);
    y = 15;
    println!("ค่าใหม่ของ y: {}", y);

    //Data Type
    //ตัวแปรชนิด Integer (จำนวนเต็ม) = Rust รองรับตัวแปร integer หลายประเภท เช่น i32, u8, i64 เป็นต้น
    let a: i32 = 100;
    let b: u8 = 255;
    println!("a: {}, b: {}", a, b);

    //ตัวแปรชนิด Floating Point (จำนวนทศนิยม) เช่น f32, f64
    let pi: f64 = 3.14159;
    println!("ค่าของ pi: {}", pi);

    //ตัวแปรชนิด Boolean
    let is_rust_fun: bool = true;
    println!("Rust สนุกไหม: {}", is_rust_fun);

    //ตัวแปรชนิด Character
    let letter: char = 'R';
    println!("ตัวอักษร: {}", letter);

    //Compound Types
    //ตัวแปรชนิด Tuple = กลุ่มของค่า “หลายประเภท” สามารถเก็บค่าหลายตัวพร้อมกัน
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // สามารถ Destructuring ได้
    println!("ค่าจาก tuple: {}, {}, {}", x, y, z);

    //ตัวแปรชนิด Array = เก็บค่าหลายค่าในลำดับเดียวกัน โดยต้องเป็น “ประเภทเดียวกัน”
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("ค่าของ array: {}", arr[0]);

    //String Types ไม่ได้ถูกเขียนในภาษาโดยตรง มาจาก Standard Library ของ Rust
    //String เป็นประเภทข้อความที่ Growable (สามารถเพิ่มความยาวได้), Mutable (เปลี่ยนแปลงได้) และรองรับการเข้ารหัสแบบ UTF-8 ซึ่งเหมาะสำหรับการจัดเก็บข้อความที่ไม่สามารถทราบความยาวได้ในขณะ Compile Time
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("ข้อความ: {}", s);

    //&str (String Slice) เป็นการอ้างอิง String แบบความยาวคงที่ ไม่สามารถเปลี่ยนค่าได้ เช่น
    // Slice = เป็นการอ้างอิง String แบบยาวคงที่ ไม่สามารถเปลี่ยนค่าได้
    let greeting: &str = "Hello, Rust!";
    println!("ข้อความ: {}", greeting);

    //Slice (การอ้างอิงบางส่วนของ array) Slice เป็นการอ้างอิงบางส่วนของข้อมูล เช่นการอ้างอิงช่วงของ array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3]; // จะอ้างถึง [2, 3]
    println!("ช่วงของ array: {:?}", slice);

    //Vector ใช้สำหรับจัดเก็บค่าหลายค่าที่สามารถปรับเปลี่ยนจำนวนได้
    //โดย Vec<T> เป็นประเภทข้อมูลใน Standard Library ที่รองรับค่าทุกประเภท หาก Vector ใดเก็บค่าประเภทเฉพาะ จะต้องระบุประเภทข้อมูลนั้นภายในเครื่องหมาย < > ตัวอย่างเช่น Vec<i32> หมายถึง Vector ที่เก็บค่าเฉพาะประเภท i32
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for i in &v {
        println!("{}", i);
    }

    //Operator
    //Arithmetic Operators (ตัวดำเนินการทางคณิตศาสตร์) ใช้ในการคำนวณทางคณิตศาสตร์ระหว่างตัวเลข
    let sum = 5 + 10;       // ผลลัพธ์: 15
    let difference = 95.5 - 4.3;  // ผลลัพธ์: 91.2
    let product = 4 * 30;    // ผลลัพธ์: 120
    let quotient = 56.7 / 32.2;  // ผลลัพธ์: 1.76
    let remainder = 43 % 5;  // ผลลัพธ์: 3
    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", sum, difference, product, quotient, remainder);

    //Comparison Operators (ตัวดำเนินการเปรียบเทียบ) ใช้เปรียบเทียบค่าระหว่างตัวแปรหรือค่าต่าง ๆ และให้ผลลัพธ์เป็น true หรือ false
    let a = 5;
    let b = 10;
    let is_equal = a == b;  // ผลลัพธ์: false
    let is_greater = b > a; // ผลลัพธ์: true
    println!("is_equal: {}, is_greater: {}", is_equal, is_greater);

    //Logical Operators (ตัวดำเนินการทางตรรกะ) ใช้ในการตรวจสอบเงื่อนไขหลาย ๆ เงื่อนไข และให้ผลลัพธ์เป็น true หรือ false
    let is_rust_fun = true;
    let is_rust_easy = false;
    let is_rust_fun_and_easy = is_rust_fun && is_rust_easy; // ผลลัพธ์: false
    let is_rust_fun_or_easy = is_rust_fun || is_rust_easy; // ผลลัพธ์: true
    let is_not_rust_fun = !is_rust_fun; // ผลลัพธ์: false
    println!("is_rust_fun_and_easy: {}, is_rust_fun_or_easy: {}, is_not_rust_fun: {}", is_rust_fun_and_easy, is_rust_fun_or_easy, is_not_rust_fun);

    //Bitwise Operators (ตัวดำเนินการระดับบิต) ใช้ในการทำงานกับข้อมูลในระดับบิต
    let a = 2;   // 10 ในฐาน 2
    let b = 3;   // 11 ในฐาน 2

    let result_and = a & b;  // ผลลัพธ์: 2 (10 ในฐาน 2)
    let result_or = a | b;   // ผลลัพธ์: 3 (11 ในฐาน 2)
    let result_xor = a ^ b;  // ผลลัพธ์: 1 (01 ในฐาน 2)
    println!("result_and: {}, result_or: {}, result_xor: {}", result_and, result_or, result_xor);

    //Assignment Operators (ตัวดำเนินการกำหนดค่า) ใช้ในการกำหนดค่าลงในตัวแปร
    let mut x = 5;
    x += 2;  // ตอนนี้ x มีค่าเป็น 7
    x *= 3;  // ตอนนี้ x มีค่าเป็น 21
    println!("x: {}", x);

    //Range Operators (ตัวดำเนินการช่วง) ใช้ในการกำหนดช่วงตัวเลข
    for i in 0..5 {
        println!("{}", i);  // ผลลัพธ์: 0, 1, 2, 3, 4
    }
    
    for i in 0..=5 {
        println!("{}", i);  // ผลลัพธ์: 0, 1, 2, 3, 4, 5
    }

 
}


//Function
// ตัวอย่างในการประกาศและการใช้งาน Function
   //Function ไม่มีพารามิเตอร์และไม่ส่งค่ากลับ
   fn greet() {
       println!("Hello, world!");
   }
   
//    fn main() {
//        greet();  // เรียกใช้ function
//    }
   
   //Function รับพารามิเตอร์
   fn print_sum(a: i32, b: i32) {
       println!("ผลรวมของ {} และ {} คือ {}", a, b, a + b);
   }
   
//    fn main() {
//        print_sum(5, 10);  // ผลลัพธ์: ผลรวมของ 5 และ 10 คือ 15
//    }
   
   //Function ที่มีการส่งค่ากลับ
   fn square(x: i32) -> i32 {
       x * x  // ไม่มีเครื่องหมาย ; ในบรรทัดสุดท้าย เพราะต้องการส่งค่ากลับ
   }
   
//    fn main() {
//        let result = square(4);  // result จะมีค่าเป็น 16
//        println!("ผลลัพธ์: {}", result);
//    }
   
   //Function ที่มีหลายพารามิเตอร์และส่งค่ากลับ
   fn add(a: i32, b: i32) -> i32 {
       return a + b;  // ใช้ return อย่างชัดเจน (ไม่จำเป็นต้องใช้ในบรรทัดสุดท้าย)
   }
   
//    fn main() {
//        let sum = add(3, 7);
//        println!("ผลรวมคือ {}", sum);  // ผลลัพธ์: ผลรวมคือ 10
//    }
   
   //Function ที่ไม่มีค่าที่ส่งกลับ (ใช้ () หรือ “unit”) หาก function ไม่ส่งค่ากลับ function นั้นจะมีประเภทการส่งกลับเป็น () หรือที่เรียกว่า “unit type” ใน Rust
   fn no_return() {
       println!("This function does not return a value.");
   }

//================================================================================================================================================
//Control Flow
//if, else if, และ else
fn flow_1() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }
}

//match เป็นโครงสร้างควบคุมที่ทรงพลังใน Rust มันทำงานคล้ายกับ switch ในภาษาอื่น ๆ แต่สามารถจัดการเงื่อนไขได้หลากหลายแบบมากกว่า
fn match_1() {
    let number = 3;

    match number {
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3 => println!("The number is 3"),
        _ => println!("The number is something else"),
    }
}
//match จะตรวจสอบค่า number และดำเนินการตามเงื่อนไขที่ตรงกัน ถ้าไม่ตรงกับกรณีใด ๆ จะใช้ _ เป็นตัวแทนของ “ค่าอื่น ๆ” และพิมพ์ข้อความสุดท้าย

//รวมถึง ใน Rust เองเราสามารถใช้ match เพื่อ return ค่ากลับมาจากแต่ละเงื่อนไขได้ แล้วนำค่าที่ return มาเก็บไว้ในตัวแปรใหม่ เช่น

fn match_2() {
    let number = 2;

    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };

    println!("The result is: {}", result);
}

//loop เป็นการวนซ้ำอย่างไม่สิ้นสุดจนกว่าจะมีคำสั่ง break เพื่อออกจากลูป
fn loop_1() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count is: {}", count);

        if count == 5 {
            break;
        }
    }

    println!("Loop finished");
}

//while ทำการวนลูปจนกว่าเงื่อนไขที่กำหนดจะเป็นเท็จ
fn while_1() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!");
}

//for ใช้เพื่อวนลูปผ่านค่าหรือช่วงของตัวเลข
fn for_1() {
    for number in 1..4 {
        println!("The number is: {}", number);
    }

    println!("Done!");
}
//=========================================================================================================================

//Ownership
fn Ownership_1() {
    let s1 = String::from("Hello");
    let s2 = s1;  // s1 ถูกย้ายไปยัง s2

    // println!("{}", s1);  // Error! s1 ไม่สามารถใช้งานได้อีก
    println!("{}", s2);
}
//*** s1 เป็นเจ้าของ String เมื่อ s2 = s1; Rust จะย้าย (move) ความเป็นเจ้าของไปที่ s2 ทำให้ s1 ไม่สามารถใช้งานได้อีก

fn Borrowing_1() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);  // ยืม s1 แต่ไม่ได้ย้าย

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()  // เราสามารถใช้ s ได้โดยไม่ต้องย้ายความเป็นเจ้าของ
}
//*** ในตัวอย่างนี้ เราใช้ &s1 เพื่อยืมค่า s1 โดยไม่ย้าย ownership และยังสามารถใช้งาน s1 ได้ภายหลัง

fn References_1() {
    let mut s = String::from("Hello");

    let r1 = &s;  // immutable reference
    let r2 = &s;  // immutable reference
    // let r3 = &mut s;  // Error! ไม่สามารถมี mutable reference ร่วมกับ immutable references

    println!("{} and {}", r1, r2);
}

//สรุปทั้งหมด 3 concept
// - Ownership: แต่ละค่าจะมีเจ้าของที่ชัดเจน เมื่อ scope ของเจ้าของสิ้นสุด ค่านั้นจะถูกปล่อยจากหน่วยความจำ
// - Borrowing: การยืมค่าทำได้ผ่านการอ้างอิง โดยเราสามารถยืมได้ทั้งแบบ immutable และ mutable แต่ต้องปฏิบัติตามกฎของการยืม
// - References: คือการยืมค่าชั่วคราวโดยไม่ต้องย้ายความเป็นเจ้าของ
//========================================================================================================================================================

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn classic_struct() {
    let user1 = User {
        username: String::from("johndoe"),
        email: String::from("johndoe@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Username: {}", user1.username);
}
//จาก code
// - Struct User ถูกประกาศด้วยฟิลด์ 4 ฟิลด์: username, email, sign_in_count, และ active
// - สร้างตัวแปร user1 ที่มีข้อมูลเหล่านี้
// - เราสามารถเข้าถึงฟิลด์ใน Struct ผ่านเครื่องหมายจุด (.) เช่น user1.username

//Tuple Structs: Struct แบบ Tuple ที่สามารถเข้าถึงฟิลด์ได้ผ่าน index
struct Color(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    println!("Black color RGB: {}, {}, {}", black.0, black.1, black.2);
}
//จาก code
// - Color เป็น Tuple Struct ที่มีฟิลด์ 3 ฟิลด์คือ i32
// - การเข้าถึงฟิลด์ใช้เลข index เช่น black.0, black.1, และ black.2

//Unit Structs: Struct ที่ไม่มีฟิลด์ใด ๆ
struct AlwaysEqual;

fn unit_struct() {
    let subject = AlwaysEqual;
}
//AlwaysEqual เป็น Unit Struct ที่ไม่มีฟิลด์ สามารถใช้งานเมื่อเราต้องการสร้างประเภทข้อมูลใหม่ที่ไม่ต้องการเก็บข้อมูลใด ๆ
//========================================================================================================================================================


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_function() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 255, 0);

    print_message(msg3);
}

fn print_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to coordinates: {}, {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB: {}, {}, {}", r, g, b),
    }
}
// จาก code
// - Enum Message ประกอบด้วย 4 รูปแบบ: Quit, Move, Write, และ ChangeColor
// 1. Quit ไม่มีข้อมูลใด ๆ
// 2. Move มีฟิลด์ {x, y} แบบ object
// 3. Write เก็บค่า String
// 4. ChangeColor เก็บข้อมูล 3 ค่าในรูปแบบ i32 ใช้ match เพื่อจัดการกับค่าที่แตกต่างกันใน Message
//========================================================================================================================================================

// Error Handling

// Result<T, E>
// - Result เป็น enum ที่มีสองค่าหลัก
// -- Ok(T) สำหรับกรณีที่การทำงานสำเร็จ
// -- Err(E) สำหรับกรณีที่เกิดข้อผิดพลาด

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn error_handling_match() {
    let result = divide(10.0, 0.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
//จาก code
// -function divide จะคืนค่า Result ซึ่งอาจเป็น Ok(f64) ในกรณีที่การหารสำเร็จ หรือ Err(String) หากมีการหารด้วยศูนย์
// -ใน function main เราใช้ match เพื่อตรวจสอบค่าผลลัพธ์ว่าเป็น Ok หรือ Err และจัดการกับกรณีที่เกิดข้อผิดพลาด

//วิธีการจัดการ Result
// นอกจากใช้ match แล้ว Rust ยังมีวิธีที่สะดวกในการจัดการกับ Result เช่น:

// unwrap: ใช้เพื่อดึงค่าจาก Ok ถ้าเป็น Err จะทำให้โปรแกรมเกิด panic (เหมาะสำหรับกรณีที่มั่นใจว่าจะไม่มีข้อผิดพลาด)
// expect: คล้ายกับ unwrap แต่เราสามารถกำหนดข้อความข้อผิดพลาดเองได้ ? operator: ใช้เพื่อส่งต่อข้อผิดพลาดขึ้นไปยัง function ที่เรียกใช้งาน (เหมาะสำหรับการเขียน code แบบสั้นและเรียบง่าย)

// - ตัวอย่าง unwrap
fn error_handling_unwrap() {
    let result = divide(10.0, 2.0);
    let value = result.unwrap();  // จะ panic ถ้าผลลัพธ์เป็น Err
    println!("Value: {}", value);
}

// - ตัวอย่าง ? operator
fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;  // ส่งต่อข้อผิดพลาดถ้าเกิดขึ้น
    Ok(content)
}

fn error_handling_expect() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
//เครื่องหมาย ? ถูกใช้เพื่อตรวจสอบว่าค่าที่คืนมาจาก std::fs::read_to_string เป็น Ok หรือ Err
//ถ้าเป็น Err, function จะคืนข้อผิดพลาดไปยัง function ที่เรียกใช้งานโดยอัตโนมัติ

//panic
// - panic! เป็นวิธีการจัดการข้อผิดพลาดที่ไม่สามารถกู้คืนได้ เช่น เมื่อเกิดข้อผิดพลาดร้ายแรงที่ไม่สามารถดำเนินการต่อได้ โปรแกรมจะหยุดทำงานและทำการ “panic” 
// โดยแสดงข้อความข้อผิดพลาดและ trace stack

fn panic_example() {
    let v = vec![1, 2, 3];

    // จะ panic เพราะ index ที่เรียกเกินขอบเขตของเวกเตอร์
    println!("{}", v[99]);
}
//ในตัวอย่างนี้ เมื่อพยายามเข้าถึง index ที่เกินขอบเขตของเวกเตอร์ โปรแกรมจะเกิด panic และหยุดการทำงาน
//========================================================================================================================================================

// Option
//รูปแบบของ Option<T>

enum Option<T> {
    Some(T),
    None,
}
// - Some(T) หมายถึงมีค่า T ที่ถูกต้องอยู่
// - None หมายถึงไม่มีค่าใด ๆ (คล้าย null)

fn find_square_root(number: f64) -> Option<f64> {
    if number >= 0.0 {
        Some(number.sqrt())  // คืนค่าคำตอบถ้าคำนวณได้
    } else {
        None  // คืนค่า None ถ้าหาคำตอบไม่ได้ (เช่นเลขติดลบ)
    }
}

fn option_fn() {
    let result = find_square_root(9.0);

    match result {
        Some(value) => println!("Square root is: {}", value),
        None => println!("No square root found."),
    }
}
//จาก code
// -- function find_square_root จะคืนค่า Option<f64> ถ้าคำนวณ square root ได้ก็จะคืนค่า Some(value) หากเป็นเลขติดลบซึ่งไม่สามารถคำนวณ square root ได้ จะคืนค่า None
// เราใช้ match เพื่อจัดการกับกรณีทั้ง Some และ None

// - ความแตกต่างระหว่าง Option กับ Result

// -- Option<T>: ใช้สำหรับค่าที่อาจจะมีหรือไม่มี ไม่ได้มีข้อผิดพลาดชัดเจน เช่น การหา element ใน list ถ้าไม่เจอก็คืนค่า None
// -- Result<T, E>: ใช้สำหรับการจัดการกับข้อผิดพลาดที่สามารถคาดการณ์ได้ โดยจะมีข้อมูลเกี่ยวกับข้อผิดพลาดในกรณีที่การทำงานไม่สำเร็จ เช่น การอ่านไฟล์ ถ้าไฟล์ไม่สามารถเปิดได้จะคืนค่า Err(e)

// ตัวอย่างการใช้ Option<T> ในสถานการณ์ทั่วไป
// - เป็นตัวอย่าง การดึงค่าจากเวกเตอร์ v.get(1) คืนค่า Option<&i32> ถ้าดึงค่าได้จะเป็น Some ถ้า index เกินขอบเขตจะเป็น None
fn option_vector() {
    let v = vec![10, 20, 30];

    let maybe_value = v.get(1);  // ดึงค่าจาก index 1

    match maybe_value {
        Some(value) => println!("Found: {}", value),
        None => println!("No value found at that index"),
    }
}

// -การใช้ unwrap กับ Option<T>
// หากเรามั่นใจว่าค่าที่ได้จะไม่เป็น None เราสามารถใช้ unwrap() เพื่อดึงค่าออกมาได้โดยตรง
// ข้อควรระวัง: ถ้าใช้ unwrap() กับ None โปรแกรมจะเกิด panic ทันที ดังนั้นควรใช้เฉพาะกรณีที่มั่นใจว่าจะมีค่าเสมอ
fn option_unwarp() {
    let some_number = Some(5);
    let number = some_number.unwrap();  // ดึงค่า 5 ออกมาได้
    println!("The number is: {}", number);
}

// -การใช้ ? กับ Option<T>
//เครื่องหมาย ? ใช้ได้กับ Option เช่นเดียวกับ Result เพื่อช่วยส่งค่า None กลับออกไปเมื่อไม่พบค่า โดยไม่ต้องเขียน match หรือ if let เอง
fn get_value(maybe_value: Option<i32>) -> Option<i32> {
    let value = maybe_value?;  // ถ้าเป็น None จะ return None ทันที
    Some(value + 10)  // ถ้ามีค่า ก็จะเพิ่ม 10 และคืนค่าใหม่
}

fn option_operator() {
    let result = get_value(Some(5));
    println!("Result: {:?}", result);
}

//สรุประหว่าง Result<T, E> และ Option<T>
// - Option<T>: ใช้เมื่อค่าที่เรากำลังจัดการอาจจะ “มี” หรือ “ไม่มี” แต่ไม่มีข้อผิดพลาดเฉพาะเจาะจง
// - Result<T, E>: ใช้เมื่อเราจำเป็นต้องจัดการข้อผิดพลาดที่อาจเกิดขึ้นจากการทำงานของโปรแกรม
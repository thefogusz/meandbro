// fn double(n: i32) -> i32 {
//     n * 2
// }
// fn main() {
//     let result = double(20);
//     println!("ผลลัพธ์คือ {}", result);
// }
// --------------------1--------------------
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn main() {
//     let result = add(10, 5);
//     println!("ผลรวมคือ {}", result);
// }
// ---------------------2--------------------
// fn is_even(a: i32) -> bool {
//     a%2 == 0
// }
// fn main() {
//     let number = is_even(400);
//     if number == true {
//         println!("{} เป็นเลขคู่", number);
//     }
//     else {
//         println!("{} เป็นเลขคี่", number);
//     }
// }
// -----------------------3-------------------

// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn is_square(&self) -> bool {
//         self.width == self.height
//     }
// }
// fn main() {
//     let rectangle = Rectangle {
//         width: 15,
//         height: 30
//     };   

//     let result = rectangle.area();
//     println!("พื้นที่คือ {}", result);

//     if rectangle.is_square() {
//         println!("เป็นสี่เหลี่ยมจัตุรัส");
//     }
//     else {
//         println!("ไม่ใช่สี่เหลี่ยมจัตุรัส");
//     }
// }
// ---------------------4-------------------

// struct Exam {
//     score: u32
// }

// impl Exam {
//     fn passed(&self) -> bool {
//         self.score >= 50
//     }
// }
// fn main() {
//     let exam = Exam {
//     score: 60
//     };
//     if exam.passed() {
//         println!("คุณผ่าน",);
//     }  
//     else {
//         println!("คุณไม่ผ่าน");
//     }
// }
// ----------------------5-----------------------

// struct Room {
//     temperature: i32
// }
// impl Room {
// fn status(&self) -> &str {
//     if self.temperature < 18 {
//         "หนาว"
//     } else if self.temperature <= 30 {
//         "กำลังดี"
//     } else {
//         "ร้อน"
//     }
//  }
// }
// fn main() {
//     let room = Room {temperature: 25};
//     println!("สถานะห้อง {}", room.status());
// }
// ---------------------6---------------------

// struct Person {
//     name: String,
//     age: u32
// }

// impl Person {
//     fn introduce(&self) {
//     println!("สวัสดี ฉันชื่อ {} อายุ {} ปี", self.name, self.age);
//     }
// }
// fn main () {
//     let person = Person {
//         name: "กัส".to_string(),
//         age: 27,
//     };
//     person.introduce();
// }
//  -----------------------7------------------------

// struct Pet {
//     name: String,
//     kind: String,
//     age: u32
// }
// impl Pet {
//     fn describe(&self) {
//         println!("สัตว์เลี้ยงของฉันชื่อ {} เป็น {} อายุ {} ปี", self.name, self.kind, self.age);
//     }
// }
// fn main() {
//     let pet = Pet {
//         name: "แม่แมว".to_string(),
//         kind: "แมว".to_string(),
//         age: 16
//     };
//     pet.describe();
// }
// ------------------------8---------------------------

// struct Book {
//     title: String,
//     author: String,
//     page: u32
// }
// impl Book {
//     fn summary(&self) {
//         println!(" หนังสือชื่อ {} ผู้แต่ง {} มีทั้งหมด {} หน้า", self.title, self.author, self.page);
//     }
//     fn is_thick(&self) -> bool {
//         self.page >= 300
//     }
// }
// fn main() {
//     let book = Book {
//         title: "ปิดตำนานอารยชน".to_string(),
//         author: "mr.horn".to_string(),
//         page: 530
//     };
//     if book.is_thick() {
//         println!("เล่มหนา");
//     }
//     else {
//         println!("เล่มบาง");
//     }
//     book.summary();
// }
// -------------------------9-------------------------

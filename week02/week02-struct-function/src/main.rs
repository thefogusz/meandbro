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

// fn main () {
//     let a = 10;
//     println!("{}", a);
//     {
//     let b = a;
//     println!("{}", b);

//     }
// }
// --------------------------10---------------------------
// #[derive(Debug)]

// struct Bank {
//     account: String,
//     balance: u32
// }
// impl Bank {
//     fn rich(&self) -> bool {
//     self.balance >= 10000
//     }
// }
// fn main () {
//     let bank = Bank {
//     account: "กัส".to_string(),
//     balance: 12000
//     };
//     // println!("{} , {} , {}", bank.account,bank.balance, bank.rich()); 

//     let other_bank = &bank;
//     println!("{:#?}", other_bank);
//     println!("{:#?}", bank);
// }
// // -----------------------------11----------------------------
// struct Songs {
//     title: String,
//     artist: String,
//     minutes: u32
// }

// impl Songs {
//     fn is_long(&self) -> bool {
//         self.minutes >= 5
//     }
// }

// fn main() {
//     let songs = vec![
//             Songs {   
//                 title: "Bohemian Rhapsody".to_string(),
//                 artist: "Queen".to_string(),
//                 minutes: 6 
//             }, 
//             Songs {   
//                 title: "Bad Guy".to_string(),
//                 artist: "Billie Eilish".to_string(),
//                 minutes: 3 
//             },
//             Songs {   
//                 title: "Stairway to Heaven".to_string(),
//                 artist: "Led Zeppelin".to_string(),
//                 minutes: 8 
//             }, 
//             Songs {   
//                 title: "Blinding Lights".to_string(),
//                 artist: "The Weeknd".to_string(),
//                 minutes: 4 
//             },
//             Songs {   
//                 title: "November Rain".to_string(),
//                 artist: "Guns N' Roses".to_string(),
//                 minutes: 9 
//             }];
//     for song in &songs {
//         println!("ชื่อเพลง: {} ชื่อศิลปิน: {} ความยาวเพลง: {} นาที {} ", 
//         song.title,
//         song.artist, 
//         song.minutes,
//         if song.is_long() {"เพลงยาว"} else {""}
//     );

//  }

// }

// struct Restaurants {
//     name: String,
//     food_type: String,
//     time_visited: u32
// }

// impl Restaurants {
//     fn is_favorite(&self) -> bool {
//         self.time_visited > 5
//     }
// }

// fn main() {
//     let restaurants = vec![
//         Restaurants {
//             name: "Shabu Indy".to_string(),
//             food_type: "ชาบู".to_string(),
//             time_visited: 12
//         },
//         Restaurants {
//             name: "แม่อ้อยตามสั่ง".to_string(),
//             food_type: "อาหารไทย".to_string(),
//             time_visited: 3
//         },
//         Restaurants {
//             name: "Sushi Masa".to_string(),
//             food_type: "ญี่ปุ่น".to_string(),
//             time_visited: 8
//         },
//         Restaurants {
//             name: "KFC".to_string(),
//             food_type: "ฟาสต์ฟู้ด".to_string(),
//             time_visited: 2
//         },
//         Restaurants {
//             name: "หมูกระทะลุงโหน่ง".to_string(),
//             food_type: "หมูกระทะ".to_string(),
//             time_visited: 9
//         }];

//     for restaurant in &restaurants {
//         println!("{} ({}) {}",
//         restaurant.name,
//         restaurant.food_type,
//         if restaurant.is_favorite() {"★ ร้านโปรด"} else {"ธรรมดา"});
//     } 
// }


// struct Movies {
//     title: String,
//     genre: String,
//     rating: u32
// }

// impl Movies {
//     fn is_masterpiece(&self) -> bool {
//         self.rating >= 90
//     }
// }

// fn main() {
//         let movies = vec![
//             Movies {
//             title: "Interstellar".to_string(),
//             genre: "Sci-Fi".to_string(),
//             rating: 95
//             },
//             Movies {
//             title: "Twilight".to_string(),
//             genre: "Romance".to_string(),
//             rating: 61
//             },
//             Movies {
//             title: "The Godfather	".to_string(),
//             genre: "Crime".to_string(),
//             rating: 98
//             },
//             Movies {
//             title: "Fast & Furious 9".to_string(),
//             genre: "Action".to_string(),
//             rating: 55
//             },
//             Movies {
//             title: "La La Land".to_string(),
//             genre: "Musical".to_string(),
//             rating: 91
//             }];
//         for movie in movies  {
//             println!("{} {} {}",movie.title, movie.genre, 
//         if movie.is_masterpiece() {"★ ยอดเยี่ยม"} else {"ธรรมดา"});
//     }
// }

// struct  Celsius {
//     degree: f64
// }
// impl Celsius {
//     fn to_fahrenheit(&self) -> f64 {
//         (self.degree * 1.8) + 32.0 
//     }
// }
// fn main() {
//     let celsius = Celsius {
//         degree: 60.0
//     };
//     println!("{}°C = {}°F", celsius.degree, celsius.to_fahrenheit());
// }// struct Student {
//     name: String,
//     score: u32
// }
// impl Student {
//     fn grade(&self) -> &str {
//         if self.score >=80 {
//             "เกรด A"
//         }
//         else if self.score >= 60 && self.score <80 {
//             "เกรด B"
//         }
//         else {
//             "เกรด F"
//         }
//     }
// }
// fn main() {
// let student = vec![
//     Student {
//         name: "กัส".to_string(),
//         score: 52,
//     },
//     Student {
//         name: "ฮอร์น".to_string(),
//         score: 89,
//     },
//     Student {
//         name: "โอเว่น".to_string(),
//         score: 65,
//     },
//   ];
// for student in &student {
//     println!("{}: {}", student.name, student.grade());}
    
// }

// -------------------------1----------------------------

// struct Game {
//     title: String,
//     genre: String,
//     hours_played: u32
// }
// impl Game {
//     fn is_favorite(&self) -> bool {
//     self.hours_played >= 100    
//     }
// }
// fn main () {
//     let game = vec![
//     Game {
//         title: "Poe2".to_string(),
//         genre: "Action RPG".to_string(),
//         hours_played: 295
//     },
//     Game {
//         title: "Valorant".to_string(),
//         genre: "FPS Shooter".to_string(),
//         hours_played: 700
//     },
//     Game {
//         title: "CS2".to_string(),
//         genre: "FPS Shooter".to_string(),
//         hours_played: 599
//     }];
//     for game in &game {
//         if game.is_favorite() {
//             println!("{} ({}): ★ เกมโปรด", game.title, game.genre);
//         }
//         else {
//             println!("{} ({}): ธรรมดา", game.title, game.genre);
//         }
//     }
    
// }
// ---------------------------2-------------------------------------

// struct Song {
//     title: String,
//     artist: String,
//     minutes: u32
// }
// impl Song {
//     fn is_long(&self) -> bool {
//         self.minutes >= 5
//     }
// }
// struct Cars {
//     license_plate: String,
//     hours_parked: u32,
//     is_electric: bool
// }

// impl Cars {
//     fn  fee(&self) -> u32 {
//         if self.is_electric {
//            self.hours_parked * 10
//         }
//         else {
//             self.hours_parked *20
//         }
//     }
//     fn  is_long_term(&self) -> bool {
//         self.hours_parked >= 24
//     }
// }
// fn main () {
//     let cars = vec![
//         Cars {   license_plate: "ABC123".to_string(),
//                  hours_parked: 24,
//                  is_electric: false },
//         Cars {   license_plate: "XYZ999".to_string(),
//                  hours_parked: 9,
//                  is_electric: true },
//         Cars {   license_plate: "QWE456".to_string(),
//                  hours_parked: 30,
//                  is_electric: true },
//         Cars {   license_plate: "MNO888".to_string(),
//                  hours_parked: 4,
//                  is_electric: false },
//         Cars {   license_plate: "ELEC999".to_string(),
//                  hours_parked: 10,
//                  is_electric: true }];

//     for car in cars {
//         println!("ทะเบียน {}: ค่าจอด {} บาท {}", 
//         car.license_plate,
//         car.fee(),
//             if car.is_long_term() { " (จอดค้างคืน)" } else { "" }
//         );
//     }
// }
//---------------------------3-------------------------------------
// struct Devices {
//     name: String,
//     battery_percent: f64,
//     charging: bool
// }
// impl Devices {
//     fn status(&self) -> String {
//         if self.battery_percent <20.0 && !self.charging {
//             "! ใกล็หมด".to_string()
//         }
//         else if self.charging {
//             "กำลังใช้งาน".to_string()
//         }
//         else if self.battery_percent > 80.0 && !self.charging {
//             "พร้อมใช้งาน".to_string()
//         }
//         else {
//             "".to_string()
//         }
//     }
// }
// ---------------------------4-------------------------------------

// fn main() {
//         let devices = vec![
//             Devices {
//             name: "Iphone13".to_string(),
//             battery_percent: 15.0,
//             charging: false
//         },
//             Devices {
//             name: "MacBook Air".to_string(),
//             battery_percent: 82.5,
//             charging: false
//         },
//             Devices {
//             name: "Galaxy Tab S7".to_string(),
//             battery_percent: 50.0,
//             charging: true
//         },
//             Devices {
//             name: "AirPods Pro".to_string(),
//             battery_percent: 12.5,
//             charging: false
//         },
//         Devices {
//             name: "Pad Mini".to_string(),
//             battery_percent: 100.0,
//             charging: true
//         }];
//         for device in devices {
//             println!("{} {} ", device.name, device.status())
//         }
// }


// struct Animal {
//     name: String,
//     leg_count: u32
// }

// impl Animal {
//         fn is_insect(&self) -> bool {
//            self.leg_count == 6
//         }
// }

// fn main() {
//         let animals = vec![
//         Animal {
//         name: "ant".to_string(),
//         leg_count: 6    
//         },
//         Animal {
//         name: "cat".to_string(),
//         leg_count: 4    
//         },
//         Animal {
//         name: "chicken".to_string(),
//         leg_count: 2    
//         },
//         Animal {
//         name: "spider".to_string(),
//         leg_count: 8    
//         }];
//             for animal in animals {
//                 println!("{} {} ", animal.name, 
//             if animal.is_insect() {("✅ แมลง")} else {("❌ ไม่ใช่แมลง")}
//         )
//     }
// }
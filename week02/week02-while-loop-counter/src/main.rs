fn main() {
    // // A counter variable 3ค่าเริ่มต้น 6เงื่อนไข 10เพิ่มค่า/ลดค่า
    // let mut n = 1;

    // // Loop while `n` is less than 101
    // while n <= 10 {
    //     println!("{}", n);

    //     // Increment counter
    //     n += 1;      //n = n+1      
    // }
    // for wood in 1..=20 {
    // println!("{}", wood); 
        
    //  }
  // ----------------------------1-----------------------
  //     let items: Vec<String> = vec!["น้ำเปล่า".to_string(), "ขนม".to_string(), "สบู่".to_string()];
  //     for (i, item) in items.iter().enumerate() {
  //     println!("index = {}, ค่า = {}", i, item);  
  //  } 
  //     println!("{}", items.len());
  //     println!("{:?}", items.join(", ")); //ต่อฟังชั่น .join ใน array String th
  // -----------------------2----------------------------
  // let items = vec!["น้ำ", "ขนม", "สบู่"];
  // let price = vec![10, 20, 15];
  // let mut total = 0;

  // for i in 0..items.len() {
  //     println!("{} ราคา {} บาท", items[i], price[i]);
  //     total += price[i];
  // }

  //     println!("รวมทั้งหมด: {} บาท", total);
  //--------------------------3--------------------

  // let foods = vec!["ข้าวมันไก่", "ผัดกะเพรา", "ต้มยำ"];
  // let prices = vec![45, 50, 60];
  // let mut sum = 0;

  // for i in 0..foods.len() {
  //     println!("สินค้า: {} - {} บาท", foods[i], prices[i]);
  //     sum += prices[i];
  // }
  // println!("รวมราคาทั้งหมด = {}", sum);
//---------------------------4--------------------------

  // let score = vec![90, 72, 65, 49, 30];
  // for i in 0..score.len() {
  // let s: i32 = score[i];
  //     println!("คนที่ {} = {} ", i + 1, s);
  // if s >=80 {
  //     println!("คุณได้เกรด A");
  // }
  // else if s >=70 {
  //     println!("คุณได้เกรด B");
  // }
  // else if s >=60 {
  //     println!("คุณได้เกรด C");
  // }
  // else if s >=50 {
  //     println!("คุณได้เกรด D");
  // }
  // else {
  //     println!("คุณได้เกรด F");
  //   }
  // }
//--------------------------6--------------------

    // let foods = vec!["ราเมง", "ซูชิ" ,"ส้มตำ"];
    // let prices = vec![100, 120, 60];
    // let money = 355;
    // let mut total: i32 = 0;

    // for i in 0..prices.len() {
    //   total += prices[i];
    // }
    // if money >= total {
    //   let change = money - total;
    //   println!("คุณจ่ายพอ! ทอน {} บาท", change);
    // } else {
    //     let  short = total - money;
    //     println!("เงินไม่พอ! ขาดอีก {} บาท", short);
    // }
    // --------------------7---------------------

    // let items = vec!["ข้าวกล่อง", "น้ำเปล่า", "นม", "โค้ก"];
    // let prices = vec![50, 10, 15, 20];
    // let mut total = 0;
    
    // println!("คุณซื้อ");
    // for i in 0..items.len() {
    //     println!("- {} ราคา {} บาท",items[i], prices[i]);
    //     total += prices[i];
    // }
    // println!("ราคารวมทั้งหมด {} บาท", total);
    // println!("จำนวนชิ้นคือ {} ", items.len());

    // if items.len() >= 3 && total >=80 {
    // println!("✅ คุณได้รับขนมฟรี 1 ชิ้น!");
    // }
    // else {
    //     println!("❌ ขออภัย คุณยังไม่ผ่านเงื่อนไขรับขนมฟรี")
    // }
    // --------------------8--------------------------------
    // let catagories = vec!["ค่าอาหาร", "ค่ารถ", "ค่าของใช้"];
    // let expenses = vec![120, 45, 90];
    // let mut total = 0;
    // for i in 0..catagories.len() {
    //     println!("หมวด: {} = {} บาท", catagories[i], expenses[i]);
    //     total += expenses[i];
    // }
    
    // println!("รวมทั้งหมด {} บาท", total);
    //---------------------9-------------------------------

    // let subject = vec!["คณิต", "อังกฤษ", "วิทย์", "สังคม"];
    // let scores = vec![80, 75, 90, 85];
    // let mut total = 0;
    //     for i in 0..scores.len() {
    //         println!(" {}: {} คะแนน", subject[i], scores[i]); 
    //         total += scores[i];
    //     }
    // let avg:f64 = total as f64 / scores.len() as f64;
    //     println!("คะแนนเฉลี่ย : {}", avg);
    
    // let students = vec!["ปื่น", "นัท", "มุก", "แนน", "แพรว"];
    // let scores = vec![45, 64, 75, 84, 55];
    
    // for i in 0..students.len() {
    //     print!("{} ได้คะแนน {} ", students[i], scores[i]); if scores[i] >= 50 {println!("ผ่าน");} else {println!("ไม่ผ่าน");}}
    // ------------------------10----------------------------

    // let weights = vec![0.5, 2.3, 6.0, 1.0, 4.5];

    // for i in 0..weights.len() {
    //     print!("พัสดุชิ้นที่ {} หนัก {} กก.", i+1 , weights[i]);

    //     if weights[i] <= 1.0 {
    //     println!("ค่าส่ง 30 บาท");
    //     } 
    //     else if weights[i] <= 5.0 {
    //     println!("ค่าส่ง 50 บาท");  
    //     }
    //     else {
    //     println!("ค่าส่ง 100 บาท");
    //     }
    // }
    // ----------------------------11--------------------------

    let balls = vec!["แดง", "เขียว", "น้ำเงิน", "เหลือง", "ขาว"];
    for i in 0..balls.len()  {
        println!("รอบที่ {} ได้สี {}", i+1, balls[i % balls.len()]);
    }

}
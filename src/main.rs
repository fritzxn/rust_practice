//struct practice------------------------------

/*fn main() {
    struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    }

    let user2 = User {
        email: string::from("another@gmail.com"),
        ..user1
    
    };
}

fn build_user(email: String, username: string) -> User {
    User { active: true,
         username, 
         email,
          sign_in_count: 1, }
    }
*/
//----------------
/* 
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "the area of the rectangle is {} square pixels.", area(width1, height1)
    );
}
fn area(width: u32, height: u32) -> u32{
    width * height
}
*/
/* 
fn main() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
      ❶ area(rect1)
   );
}
fn area(dimensions: (u32, u32)) -> u32 {
      dimensions.0 * dimensions.1
}
*/

/*
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the area is:{}",area(&rect1));
    println!("rect1 is {rect1:?}");
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
    */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main () {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!(
        "the area oif the rectangle is {} square pixels.",
        rect1.area()
    );
}
*/

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
   }
}
/* fn main () {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    if rect.width() {
        println!(
            "the rectangle has a non zero width {}", 
            rect.width
        );
    }
} */
/* 
impl Rectangle { this is an example of an associated function
    fn square(size: u32) -> ❶ Self {
      ❷ Self {
            width: size,
            height: size,
        }
   }
} use it by calling Rectangle::square()
 
*/
fn hello() -> i32 {
    3
}
//---------------------------------------------------------------
//enumerations and pattern matching

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //write blah blah
    }
}
fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    
    let some_char = Some(5);
    let some_number = Some('e');
    let absent_number: Option<i32> = None;

}
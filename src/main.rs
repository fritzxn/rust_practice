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
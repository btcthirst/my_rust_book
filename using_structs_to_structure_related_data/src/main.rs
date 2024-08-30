#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;

fn main() {
    let user1 = User{
        active:true,
        email: String::from("some@some.com"),
        username: String::from("someuser1"),
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("someone@som.com"),String::from("username2"));
    let user21 = User{
        active: user2.active,
        email: user2.email.clone(),
        sign_in_count: user1.sign_in_count,
        username: String::from("value_user")
    };
    let _subject = AlwaysEqual;
    println!("Hello, {:?} & {:#?} & {:#?}!",user1.username, user2,user21);

    area_game()
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        email,
        username,
        sign_in_count:1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle{
            height,
            width,
        }
    }
    fn area(&self) -> u32 {
        self.width*self.height
    }

    fn can_hold(&self, rectangle: &Rectangle,) -> bool {
        if self.width> rectangle.width && self.height > rectangle.height {
            return  true;
        };
        false
    }
}

fn area_game() {
    let rect1 = Rectangle{
        width:dbg!(15*2),//використовується щоб швидко показати результат при розробці
        height:50,
    };
    let rect2 = Rectangle::new(15,50);
    // dbg!(rect1); ----------- value moved here
    println!("The function area of the rectangle is {} square pixels.", area(&rect1));
    println!("The method area of the rectangle is {} square pixels.", rect1.area());
    println!("is can ract1 hold 2: {}",rect1.can_hold(&rect2));
    dbg!(rect1);
}



fn area(rectangle: &Rectangle) -> u32{
    rectangle.width*rectangle.height
}
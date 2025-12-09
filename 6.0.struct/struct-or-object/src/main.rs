fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("=================USER1=================");
    println!("User1 debug print: {:?}", user1);
    println!("User1 pretty print: {:#?}", user1);
    println!("=================USER1 PROPERTIES=================");
    println!("User1 username: {}", user1.username);
    println!("User1 email: {}", user1.email);
    println!("User1 sign_in_count: {}", user1.sign_in_count);
    println!("User1 active: {}", user1.active);

    let mut user2 = User {
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("=================befor USER 2 UPDATE EMAIL=================");
    println!("User2 debug print: {:?}", user2);

    println!("=================after update USER 2=======");
    user2.email = String::from("user2@newexample.com");
    println!("User2 debug print: {:?}", user2);

    let user3: User = User{
        email: String::from("user3@example.com"),
        ..user2
    };
    println!("User2 debug print: {:?}", user3);



}
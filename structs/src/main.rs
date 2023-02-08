
#[derive(Debug)]
struct User {
    active: bool,
    name: String,
    age: u32,
    email: String,
    address: String
}
impl User {
    fn say_hello(&self) {
        println!("hello i'm {}", self.name);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 0
    }
}



fn init_user( name: String, email: String, address: String) -> User {
    User {
        active: true,
        name,
        age: 18,
        email,
        address
    }
}

fn main() {
    // 创建结构体
    let user1 = init_user(
        String::from("xiao hei"),
        String::from("123@123.com"), 
        String::from("xxx/xxx/xxx")
    );

    println!("user.name: {}", user1.name);
    println!("user.age: {}", user1.age);
    println!("user.email: {}", user1.email);
    println!("user.address: {}", user1.address);
    println!("user.active: {}", user1.active);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user.name: {}", user2.name);
    println!("user.age: {}", user2.age);
    println!("user.email: {}", user2.email);
    println!("user.address: {}", user2.address);
    println!("user.active: {}", user2.active);
    println!("user.active: {}", user1.email);

    user2.say_hello();
    println!("{:#?}", user2);
    println!("===================== ");

    // 测试struct function
    let re = Rectangle {
        width: 100,
        height: 200
    };

    if re.height() && re.width() {
        println!("rectangle area is : {}", re.area());
    }

    /*
    总结
    结构体让你可以创建出在你的领域中有意义的自定义类型。
    通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。
    在 impl 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。
    但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具。
    */
}


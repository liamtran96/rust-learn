fn main() {
    let dog = Dog {
        id: 1,
        name: String::from("HEHE"),
    };
    let cat = Cat {
        id: 1,
        name: String::from("lili"),
    };
    println!("Dog #{}: {}", dog.id, dog.describe());
    println!("Cat #{}: {}", cat.id, cat.describe());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dog() {
        let dog = Dog {
            id: 1,
            name: String::from("HEHE"),
        };
        assert_eq!(dog.name(), "HEHE");
        assert_eq!(dog.sound(), "woof");
        assert_eq!(dog.describe(), "HEHE says woof");
    }
    #[test]
    fn test_cat() {
        let cat = Cat {
            id: 1,
            name: String::from("lili"),
        };
        assert_eq!(cat.sound(), "Meow");
        assert_eq!(cat.name(), "lili");
        assert_eq!(cat.describe(), "lili says Meow");
    }
}
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> String;
    fn describe(&self) -> String {
        format!("{} says {}", self.name(), self.sound())
    }
}
struct Dog {
    id: u32,
    name: String,
}
struct Cat {
    id: u32,
    name: String,
}
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    fn sound(&self) -> String {
        String::from("woof")
    }
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    fn sound(&self) -> String {
        String::from("Meow")
    }
}

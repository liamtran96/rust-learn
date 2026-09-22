fn main() {
    let example = Example {
        text: String::from("helllo"),
    };
    println!("this is {}", example.summarize());
    notify(&example);
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Example {
            text: String::from("hello"),
        }),
        Box::new(Article {
            title: String::from("Learning Rust"),
        }),
    ];
    notify_dyn(&items);
}

struct Example {
    text: String,
}

struct Article {
    title: String,
}
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}
trait Summary {
    fn summarize(&self) -> String;
}
fn notify<T: Summary>(s: &T) {
    println!("{}", s.summarize());
}

fn notify_dyn(items: &[Box<dyn Summary>]) {
    for item in items {
        println!("{}", item.summarize());
    }
}

impl Summary for Example {
    fn summarize(&self) -> String {
        String::from(&self.text)
    }
}

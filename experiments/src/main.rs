mod scraps;
mod rc_arc;
mod patterns;
mod http;

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

struct Person {
    name: String,
    birth: i32,
}

struct PersonOpt {
    name: Option<String>,
    birth: i32,
}

fn use_opt() {
    let mut composers = Vec::new();
    composers.push(PersonOpt {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    });

    let first_name = std::mem::replace(&mut composers[0].name, None);
    composers[0].name.take();
}

fn take_owner(p: Person) {
    scraps::check_it_out()
}

fn route(kind: IpAddrKind) {
    let mut v = Vec::new();
    v.push(1);
    let first = v[0];
}

#[cfg(test)]
#[test]
fn testing() {
    println!("Hey!",)
}

fn main() {
    let four = IpAddrKind::V4("1.1.1.1".to_string());
    let another = IpAddrKind::V4("2.2.2.2".to_string());
    match another {
        IpAddrKind::V4(value) => println!("Got you! {}", value),
        _ => println!("Oh well",),
    }

    {
        if let IpAddrKind::V4(val) = four {
            println!("Eh? {}", val)
        }
    }
    println!("Hello, world! {:?}", 1);

    let p1 = Person {
        name: "".to_string(),
        birth: 123,
    };

    let mut v = Vec::new();
    v.push(p1);

    let first = &v[0];

    let mut what = "Hey".to_string();
    what.push('!');
    println!("{:?}", what);

    use_opt()
}

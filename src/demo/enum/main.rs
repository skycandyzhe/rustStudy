#[derive(Debug)]
enum Book {
    Papery(u32),
    Electronic(String),
}

enum Book2 {
    // 将struct当为枚举对象
    Index(u32),
    Papery {index: u32},
    Electronic {url: String},
}
fn main() {
    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url://..."));
    println!("{:?} {:?}", book,ebook);
    let _book = Book2::Papery{index:1000};
    let _book=Book2::Index(12);
    let _ebook = Book2::Electronic{ url:String::from("url://...")};
    match _book {
        Book2::Index(i)=>{
            println!("Book2::Index {:?}",i);
        }
        Book2::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book2::Electronic { url } => {
            println!("E-book {}", url);
        }
    }



    let mut opttest: Option<&str> =None;
    // opttest=Option::Some("Hello");
    match opttest{
        Option::Some(something) => {
            println!("{}", something);
        },
        None => {
            println!("opt is nothing");
        }
        _ => {
            println!("opt is nothing");
        }
        
    }
    // let book = Book2::Electronic{url:String::from("url")};
    // 不好用
    if let 12= 12 {
        println!("Papery {}", "url");
    } else {
        println!("Not papery book");
    }
}
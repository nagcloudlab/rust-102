/// declarative macros

trait MyTrait {
    fn do_something(&self);
}

macro_rules! impl_my_trait {
    ($t:ty) => {
        impl MyTrait for $t {
            fn do_something(&self) {
                println!(
                    "do_something() for type: {:?} and value {:?}",
                    stringify!($t),
                    self
                );
            }
        }
    };
    () => {};
}

impl_my_trait!(i32);
impl_my_trait!(String);

macro_rules! html {
    ($($tag:ident { $($content:tt)* })*) => {
       {
         let mut html_string = String::new();
        $(
            html_string.push_str(&format!("<{}>{}</{}>", stringify!($tag), html!($($content)*), stringify!($tag)));
        )*
        html_string
       }
    };
    ($txt:expr) => {
        $txt.to_string()
    };
}

fn main() {
    let x = 10;
    x.do_something();

    let y = String::from("Hello");
    y.do_something();

    let html = html! {
        div {
            h1 { "Hello World" }
            p { "This is a paragraph" }
        }
    };

    println!("{}", html);
}

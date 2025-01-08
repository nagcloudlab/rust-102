/// declarative macros

macro_rules! config_option {
    ($cfg:meta,$block:block) => {
        #[cfg($cfg)]
        {
            $block
        }
    };
}

fn main() {
    // // this code block will only be included in the compiled binary if the debug_assertions
    // #[cfg(debug_assertions)]
    // {
    //     println!("Debug mode is enabled");
    // }

    // // this code block will only be included in the compiled binary if the debug_assertions is not enabled
    // #[cfg(not(debug_assertions))]
    // {
    //     println!("Release mode is enabled");
    // }

    fn foo() {
        println!("foo");
    }
    config_option!(debug_assertions, {
        println!("Debug mode is enabled");
        foo();
    });

    config_option!(not(debug_assertions), {
        println!("Release mode is enabled");
    });
}

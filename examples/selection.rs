use zenity::menu::selection::{Option,SelectionMenu};

fn main() {
    let hello_world = Option{
        text: "Hello World".to_string(),
        notes: "bli bla blub".to_string(),
    };

    let method2 = Option{
        text: "method2 says Hi".to_string(),
        notes: "No!".to_string(),
    };

    SelectionMenu::new(hello_world).add_option(method2);
    
}
use gdnative::prelude::*;

// The HelloWorld Class
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
    fn new(_owner: &Node) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Hello, world!");
    }
}

// Registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);

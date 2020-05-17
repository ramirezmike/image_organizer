use orbtk::prelude::*;

mod identifiers {
    pub const SCREEN_SIZE_LABEL: &str = "screen_size_label";
}

// this enum is used in window update loop to pass data 
// from events to a place where we can
// more easily mutate parts of the window. 
enum Action {
    Resize { width: f64, height: f64 }
}

#[derive(Default, AsAny)] // Default required for widget! macro, AsAny require to impl State
pub struct WindowState {
    action: Option<Action> // TODO: I suspect there is a bug here where only one "action"
                           //       can be handled per update loop. If another action
                           //       occurs before an update iteration, it will likely overwrite
                           //       the previously queued action.
}

impl WindowState {
    // this sets the window's action enum to fire on next call to update
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for WindowState {
    // this update function is called repeatedly while the program runs
    fn update(&mut self, _: &mut Registry, context: &mut Context) {
        // "if let pattern matching" when we only want to handle one enum value 
        // in this case only want to do something when Some not None
        if let Some(action) = &self.action { 
            match action {
                // only handling the one Action enum value we have
                Action::Resize { width:w, height:h } => {
                    // gets label and updates it with current resolution                        
                    let mut child_label = context.child(identifiers::SCREEN_SIZE_LABEL);
                    child_label.set::<String16>("text", String16::from(format!("{}x{}", w, h)));
                    child_label.set::<f64>("font_size", w * 0.15);
                }
            }

            // action handled, setting back to None so nothing happens when Update is called again
            self.action = None; 
        }
    }
}

// uses the widget! macro to generate a MainView widget with our WindowState above
widget!(MainView<WindowState> {});

// template for how the way the MainView widget looks
impl Template for MainView {
    fn template(self, _id: Entity, build_context: &mut BuildContext) -> Self {
        self.name("MainView")
            .width(1280.0)
            .height(800.0)
            .child(TextBlock::create()
                             .width(0.0)
                             .height(14.0)
                             .font_size(72.0)
                             .text("Hello World")
                             .element("text-block")
                             // this is where we hardcode an identifier to grab this textblock
                             // in the action handling to update it later
                             .id(identifiers::SCREEN_SIZE_LABEL) 
                             .vertical_alignment("start")
                             .build(build_context))
    }
}

mod file_io {
    use std::fs;
    use std::path::Path;

    pub fn get_directory_list(directory_path:&str) -> Result<Vec<String>, std::io::Error> {
        let mut found_paths: Vec<String> = Vec::new();
        let path = Path::new(&directory_path);

        for entry in fs::read_dir(path)? {
            let found_path = entry?.path();
            if !found_path.is_dir() {
                if let Some(path) = found_path.to_str() {
                    found_paths.push(String::from(path));
                }
            }
        }

        Ok(found_paths)
    }
}


fn main() {
    println!("Hello, world!");

    let paths = file_io::get_directory_list(".");
    match paths {
        Ok(p) => p.iter().for_each(|x| println!("{}", &x)),
        Err(e) => println!("{:?}", e)
    }

    Application::new()
        .window(|build_context| {
            // this creates an instance of the MainView widget and adds it to the build context
            // build returns an "Entity" which is an identifier that we use later
            // to pull out WindowState
            let main_view_entity = MainView::create().build(build_context);
            Window::create().title("image_organizer")
                            .position((100.0, 100.0))
                            .size(1280.0, 800.0)
                            .insert_handler(WindowEventHandler {
                                // "move" used to keep borrowed main_view_entity alive long enough
                                handler: Rc::new(move |states_context, e| { 
                                    // here using if let pattern match again
                                    if let WindowEvent::Resize { width, height } = e {
                                        // gets a mutable reference to the WindowState object
                                        // in the MainView widget and inserts an Action
                                        // using the resolution we got from the event handler
                                        states_context.get_mut::<WindowState>(Entity::from(main_view_entity))
                                                      .action(Action::Resize { width, height });
                                    }
                                    true  
                                })
                            })
                            .resizeable(true)
                            .child(main_view_entity) // this is adding the MainView widget
                            .build(build_context)
        }).run();
}

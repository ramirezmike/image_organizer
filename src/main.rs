use orbtk::prelude::*;

// this enum is used in window update loop to pass data from events to a place where we can
// more easily mutate parts of the window. 
#[derive(Copy, Clone)] // Copy used in pattern match in fn update, Clone needed for Copy
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
        if let Some(action) = self.action { 
            match action {
                // only handling the one Action enum value we have
                Action::Resize { width:w, height:h } => {
                    // gets label and updates it with current resolution                        
                    let mut child_label = context.child("HelloLabel");
                    let label_value = child_label.get_mut::<String16>("text");
                    label_value.clear(); 
                    label_value.insert_str(0usize, &format!("{}x{}", w, h));
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
                             .text("Hello World")
                             .element("text-block")
                             // this is where we hardcode an identifier to grab this textblock
                             // in the action handling to update it later
                             .id("HelloLabel") 
                             .vertical_alignment("start")
                             .build(build_context))
    }
}

fn main() {
    println!("Hello, world!");

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
                                    if let WindowEvent::Resize { width:w, height:h } = e {
                                        // gets a mutable reference to the WindowState object
                                        // in the MainView widget and inserts an Action
                                        // using the resolution we got from the event handler
                                        states_context.get_mut::<WindowState>(Entity::from(main_view_entity))
                                                      .action(Action::Resize { width: w, height: h });
                                    }
                                    true  
                                })
                            })
                            .resizeable(true)
                            .child(main_view_entity) // this is adding the MainView widget
                            .build(build_context)
        }).run();
}

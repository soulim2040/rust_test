
mod command;

use cursive::{
    view::Nameable,
    views::{Dialog, EditView},
    Cursive,
};

use command::{Command, CutCommand, CopyCommand, PasteCommand};

#[derive(Default)]
struct AppContext {
    clipboard : String,
    history : Vec<Box<dyn Command>>,
}

fn main() {
    let mut app = cursive::default();

    app.set_user_data(AppContext::default());

    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and use Buttons")
            .button("Copy", |s| execute(s, CopyCommand::default()))
            .button("Cut", |s| execute(s, CutCommand::default()))
            .button("Paste", |s| execute(s, PasteCommand::default()))
            .button("Undo", undo)
            .button("Quit", |s| s.quit())
    );

    app.run();
}

fn execute(app : &mut Cursive, mut command : impl Command + 'static){
    if command.execute(app){
        app.with_user_data(|context : &mut AppContext|{
            context.history.push(Box::new(command));
        });
    }
}

fn undo(app : &mut Cursive){
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut cmd) = context.history.pop(){
        cmd.undo(app);
    }
    app.set_user_data(context);
}

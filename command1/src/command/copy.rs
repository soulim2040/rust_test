use cursive::{views::EditView, Cursive};

use super::Command;
use crate::AppContext;

#[derive(Default)]
pub struct CopyCommand;

impl Command for CopyCommand {
    fn execute(&mut self, app : &mut Cursive) -> bool{
        let editor = app.find_name::<EditView>("Editor").unwrap();

        app.with_user_data(|context : &mut AppContext| {
            context.clipboard = editor.get_content().to_string();
        });

        false
    }
    fn undo(&mut self, app : &mut Cursive){}
}
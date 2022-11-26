mod observer;
mod editor;

use editor::Editor;
use observer::Event;
use observer::Publisher;

fn main() {
    // println!("Hello, world!");

    // let mut evts : HashMap<Event, Vec<i32>> = HashMap::new();
    // evts.entry(Event::Save).or_default();
    // let vec = evts.get_mut(&Event::Save).unwrap();
    // vec.push(33);
    // vec.push(44);
    // vec.retain(|&x| x != 33);
    // dbg!(&evts);

    let mut editor = Editor::default();
    let mut publisher = editor.publish_er();

    publisher.subscribe(Event::Load, |path|{
        println!("editor loadl file ==> {}", path);
    });

    publisher.subscribe(Event::Save, save_fn);

    editor.load("txt1.txt".into());
    editor.save();
}

fn save_fn(file_path : String){
    println!("editor save file to  ==> {}", file_path);
}

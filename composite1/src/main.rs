mod fs;

use fs::{Component, File, Folder};
fn main() {
    let file1 = File::new("file1");
    let file2 = File::new("file2");
    let file3 = File::new("file3");

    let mut folder = Folder::new("folder1");
    folder.add(file2);
    folder.add(file3);

    let mut folder2 = Folder::new("folder2");
    folder2.add(file1);
    folder2.add(folder);

    folder2.search("file3");
}

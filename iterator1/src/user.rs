
pub struct UserVec {
    users : [&'static str; 3],
}

pub struct UserIterator<'a> {
    index : usize,
    user_vec : &'a UserVec,
}

impl Iterator for UserIterator<'_> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_vec.users.len(){
            let user = Some(self.user_vec.users[self.index]);
            self.index += 1;
            return user;
        }

        None
    }
}
impl UserVec {
    pub fn new() -> Self {
        Self {
            users : ["benny", "kenny", "carl"],
        }
    }

    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index : 0,
            user_vec : self,
        }
    }
}
pub struct Track{
    pub title : String,
    pub duration : u32,
    cursor : u32,
}

impl Track {
    pub fn new(title : &str, duration : u32) -> Self {
        Self {
            title : title.into(), 
            duration, 
            cursor : 0,
        }
    }
}

pub struct Player {
    playerlist : Vec<Track>,
    current_track : usize,
    _volume : u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            playerlist : vec![
                Track::new("Track 1", 150),
                Track::new("Track 2", 160),
                Track::new("Track 3", 170),
            ],
            current_track : 0,
            _volume : 50,
        }
    }
}

impl Player {
    pub fn next_track(&mut self) {
        self.current_track = (self.current_track + 1) % self.playerlist.len();
    }

    pub fn prev_track(&mut self){
        self.current_track = (self.playerlist.len() + self.current_track - 1) % self.playerlist.len();
    }

    pub fn play(&mut self){
        self.track_mut().cursor = 10;
    }

    pub fn pause(&mut self){
        self.track_mut().cursor = 40;
    }

    pub fn rewind(&mut self){
        self.track_mut().cursor = 0;
    }

    pub fn track(&self) -> &Track{
        &self.playerlist[self.current_track]
    }

    pub fn track_mut(&mut self) -> &mut Track{
        &mut self.playerlist[self.current_track]
    }
}
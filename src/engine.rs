pub struct Buffer {
    filename: &str,
    content: String,
};

pub struct Location {
    line: i64,
    col: i64
};

pub struct Selection {
    start: Location,
    end: Location,
} 


impl Buffer {
    open(&mut self) {
        let file = File::open(self.filename);
        self.content.append(file)
    }

    
    send(&mut self, command: Command) {
    }

}




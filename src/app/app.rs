pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub progress: f64,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { 
            title,
            should_quit: false,
            progress: 0.0,
        }
    }

    pub fn on_up(&mut self) {
        // self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        // self.tasks.next();
    }

    pub fn on_right(&mut self) {
        // self.tabs.next();
    }

    pub fn on_left(&mut self) {
        // self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
        self.progress += 0.001;
        if self.progress > 1.0 {
            self.progress = 0.0;
        }

        // self.sparkline.on_tick();
        // self.signals.on_tick();

        // let log = self.logs.items.pop().unwrap();
        // self.logs.items.insert(0, log);

        // let event = self.barchart.pop().unwrap();
        // self.barchart.insert(0, event);
    }
}
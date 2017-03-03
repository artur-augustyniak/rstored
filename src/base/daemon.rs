#[derive(Debug)]
pub struct Daemon<T: ? Sized> {
    running: bool,
    pub name: T
}

impl<T> Daemon<T> {
    pub fn new(id: T) -> Daemon<T> {
        Daemon { name: id, running: false }
    }

    pub fn start(&mut self) -> bool {
        match self.running {
            false => {
                self.running = true;
                true
            },
            true => {
                false
            }
        }
    }

    pub fn stop(&mut self) -> bool {
        let tmp = self.running;
        self.running = false;
        tmp
    }

    pub fn reload(&mut self) -> bool {
        self.running
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn can_start_when_not_running() {
        let mut d = Daemon::new("some_name");
        assert!(d.start());
    }


    #[test]
    
    fn cant_reload_when_not_running() {
        let mut d = Daemon::new("some_name");
        assert!(!d.reload());
    }


    #[test]
    fn cant_stop_when_not_running() {
        let mut d = Daemon::new("some_name");
        assert!(!d.stop());
    }

    #[test]
    fn can_stop_when_already_running() {
        let mut d = Daemon::new("some_name");
        assert!(d.start());
        assert!(d.stop());
    }

    #[test]
    fn can_reload_when_already_running() {
        let mut d = Daemon::new("some_name");
        assert!(d.start());
        assert!(d.reload());
    }


    #[test]
    fn can_reload_when_already_reloaded() {
        let mut d = Daemon::new("some_name");
        assert!(d.start());
        assert!(d.reload());
        assert!(d.reload());
    }

    #[test]
    fn cant_start_when_already_running() {
        let mut d = Daemon::new("some_name");
        assert!(d.start());
        assert!(!d.start());
    }

    #[test]
    fn cant_start_when_already_reloaded() {
        let mut d = Daemon::new("some_name");
        assert!(d.start());
        assert!(d.reload());
        assert!(!d.start());
    }
}
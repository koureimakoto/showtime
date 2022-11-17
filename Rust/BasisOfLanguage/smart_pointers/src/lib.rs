pub trait
Messenger {
    fn send(&self, msg: &str);
}

pub struct
LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max  : usize
}

impl<'a, T> LimitTracker<'a, T> where 
    T: Messenger {
    pub fn
    new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn
    set_value(&mut self, value: usize) {
        self.value = value;
        
        let perc_of_max: f64 = self.value as f64 / self.max as f64;

        if perc_of_max >= 1.0 {
            self.messenger
                .send("Error: Você ultrapassou sua cota")
        } else
        if  perc_of_max>= 0.9 { 
            self.messenger
                .send("Urgent warning: Você usou mais que 90% da sua cota");
        } else
        if perc_of_max >= 0.5 {
            self.messenger
                .send("warning: Você excedeu 75% da sua cota");
        }
    }
    
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct
    MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }   

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(Vec::new()) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger   : MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new(&mock_messenger, 100);
        
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        
        let x = &mock_messenger.sent_messages.borrow().to_owned()[0] ;
        assert_eq!("warning: Você excedeu 75% da sua cota", x)
    }

    #[test]
    fn it_sends_an_over_90_percent_warning_message() {
        let mock_messenger   : MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new(&mock_messenger, 100);
        
        limit_tracker.set_value(90);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        
        let x = &mock_messenger.sent_messages.borrow().to_owned()[0] ;
        assert_eq!("Urgent warning: Você usou mais que 90% da sua cota", x)
    }
    
    #[test]
    fn it_sends_an_over_100_percent_warning_message() {
        let mock_messenger   : MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new(&mock_messenger, 100);
        
        limit_tracker.set_value(100);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        
        let x = &mock_messenger.sent_messages.borrow().to_owned()[0] ;
        assert_eq!("Error: Você ultrapassou sua cota", x)
    }

}
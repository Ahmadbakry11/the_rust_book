use std::cell::RefCell;
pub trait Messenger {
    fn send(&self, message: String);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a mut  T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    fn new(messenger: &'a mut T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, new_value: usize) {
        self.value = new_value;

        let usage_percentage: f64 = self.value as f64 / self.max as f64;

        if usage_percentage >= 1.0 {
            self.messenger.send(String::from("Err: you are over your quota"));
        } else if usage_percentage >= 0.9 {
            self.messenger.send(String::from("Urgent warning: You've used up over 90% of your quota!"));
        } else if usage_percentage >= 0.75 {
            self.messenger.send(String::from("Warning: You've used up over 75% of your quota!"));
        }
    }
}


#[cfg(test)]


mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> Self {
            Self { sent_messages: RefCell::new(vec![]) }
        }
    }

    /*
        MockMessenger implements the Messenger trait
        which has only one method send().
        send() takes an immutable ref of the mock messenger 
        and it tries to mutate that ref.
        which is not possible according to the BC rules.
        so, instead we need 
    */ 

    impl Messenger for MockMessenger {
        fn send(&self, message: String) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]

    fn it_sends_over_75_percentage() {
        let mut messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mut messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(messenger.sent_messages.borrow().len(), 1);
    }
}
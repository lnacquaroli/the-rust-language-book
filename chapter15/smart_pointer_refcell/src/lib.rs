/*

We’ll create a library that tracks a value against a maximum value and sends messages based
on how close to the maximum value the current value is. This library could be used to keep
track of a user’s quota for the number of API calls they’re allowed to make, for example.

We want to be able to say that if we create a LimitTracker with something that implements
the Messenger trait and a particular value for max, when we pass different numbers for
value, the messenger is told to send the appropriate messages.

We need a mock object that, instead of sending an email or text message when we call send,
will only keep track of the messages it’s told to send.

- When creating immutable and mutable references, we use the & and &mut syntax.
- With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API
that belongs to RefCell<T>.
- The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart
pointer type RefMut<T>.
- Both types implement Deref, so we can treat them like regular references.

*/

// Mock object
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        return LimitTracker {
            messenger,
            value: 0,
            max,
        };
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            return MockMessenger {
                sent_messages: RefCell::new(vec![]),
            };
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

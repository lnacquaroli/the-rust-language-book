/*

First, we’re going to implement the state pattern in a more traditional
object-oriented way, then we’ll use an approach that’s a bit more natural in Rust.
Let’s dig in to incrementally implementing a blog post workflow using the state
pattern.

The final functionality will look like this:
- A blog post starts as an empty draft.
- When the draft is done, a review of the post is requested.
- When the post is approved, it gets published.
- Only published blog posts return content to print, so unapproved posts can’t
accidentally be published.

- You may have been wondering why we didn’t use an enum with the different possible
post states as variants. That’s certainly a possible solution, try it and compare
the end results to see which you prefer! One disadvantage of using an enum is every
place that checks the value of the enum will need a match expression or similar to
handle every possible variant. This could get more repetitive than this trait object
solution.

*/

// Then Post will hold a trait object of Box<dyn State> inside an Option<T> in a
// private field named state to hold the state object. You’ll see why the Option<T>
// is necessary in a bit.
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // We implement this as a method, rather than exposing the content field as pub,
    // so that later we can implement a method that will control how the content
    // field’s data is read.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // The content method on Post delegates to a content method on State
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // Requesting a review of the post changes its state
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // Set state to the value that the current state says it should have when that
    // state is approved
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// Default implementations of the methods:
trait State {
    /*
    This syntax means the method is only valid when called on a Box holding the
    type. This syntax takes ownership of Box<Self>, invalidating the old state so
    the state value of the Post can transform into a new state.
    To consume the old state, the request_review method needs to take ownership of
    the state value. This is where the Option in the state field of Post comes in:
    we call the take method to take the Some value out of the state field and leave
    a None in its place, because Rust doesn’t let us have unpopulated fields in
    structs. This lets us move the state value out of Post rather than borrowing it.
    */
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return "";
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {});
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

struct PendingReview {}

impl State for PendingReview {
    // when we request a review on a post already in the PendingReview state,
    // it should stay in the PendingReview state, that's why returns self.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Published {});
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
}

/*

- This new implementation won't have a content method in the draft post.

- Both the Post and DraftPost structs have a private content field that stores the
blog post text.
- The structs no longer have the state field because we’re moving the encoding of
the state to the types of the structs.
- The Post struct will represent a published post, and it has a content method that
returns the content.

- The PendingReviewPost struct doesn’t have a content method defined on it, so
attempting to read its content results in a compiler error, as with DraftPost.
- Because the only way to get a published Post instance that does have a content
method defined is to call the approve method on a PendingReviewPost, and the only
way to get a PendingReviewPost is to call the request_review method on a DraftPost,
we’ve now encoded the blog post workflow into the type system.

*/

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        return DraftPost {
            content: String::new(),
        };
    }

    pub fn content(&self) -> &str {
        return &self.content;
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Takes ownership, consuming the DraftPost
    pub fn request_review(self) -> PendingReviewPost {
        return PendingReviewPost {
            content: self.content,
        };
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // Takes ownership, consuming the PendingReviewPost
    pub fn approve(self) -> Post {
        return Post {
            content: self.content,
        };
    }
}

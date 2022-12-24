/*

- We’ll show you how to rethink the state pattern to get a different set of
trade-offs.
- Rather than encapsulating the states and transitions completely so
outside code has no knowledge of them, we’ll encode the states into different types.
Consequently, Rust’s type checking system will prevent attempts to use draft posts
where only published posts are allowed by issuing a compiler error.

- The changes we needed to make to main to reassign post mean that this
implementation doesn’t quite follow the object-oriented state pattern anymore: the
transformations between the states are no longer encapsulated entirely within the
Post implementation. However, our gain is that invalid states are now impossible
because of the type system and the type checking that happens at compile time! This
ensures that certain bugs, such as display of the content of an unpublished post,
will be discovered before they make it to production.

- Object-oriented patterns won’t always be the best solution in Rust due to certain
features, like ownership, that object-oriented languages don’t have.

*/

use blog_rust::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // Shadowing to save the returned instances
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

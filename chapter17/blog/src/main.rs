/*

- Notice that the only type we’re interacting with from the crate is the Post type.
- This type will use the state pattern and will hold a value that will be one of
three state objects representing the various states a post can be in—draft, waiting
for review, or published.
- Changing from one state to another will be managed internally within the Post
type. The states change in response to the methods called by our library’s users on
the Post instance, but they don’t have to manage the state changes directly. Also,
users can’t make a mistake with the states, like publishing a post before it’s
reviewed.

*/

use blog::Post;

fn main() {
    // Allow user to create a new blog post
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // will be ignored
    post.add_text("foo foo foo");

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    // will be ignored
    post.add_text("foo foo foo");

    // Post requires TWO approvals to be approved
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Should be a no-op
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

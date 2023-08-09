use blog_deux::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");


    let post = post.request_review();

    // Post is now DraftPost again, so we need another review requested
    let post = post.reject();
    let post = post.request_review();

    // Requires two approvals
    let post = post.approve();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

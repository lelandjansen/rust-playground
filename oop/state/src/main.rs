mod blog;
mod blog2;
mod blog3;

fn main() {
    let message = "I ate a salad for lunch today";

    let mut post = blog::Post::new();
    post.add_text(message);
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(message, post.content());

    let mut post = blog2::Post::new();
    post.add_text(message);
    let post = post.request_review();
    let post = post.approve();
    assert_eq!(message, post.content());

    let mut post = blog3::Post::new();
    post.add_text(message);
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.reject();
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(message, post.content());
}

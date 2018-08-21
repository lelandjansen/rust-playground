pub struct Post {
    state:Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        println!("Post new");
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        println!("Post add_text");
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        println!("Post content");
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        println!("Post request_review");
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        println!("Post approve");
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        println!("State content");
        ""
    }
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        println!("Draft request_review");
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        println!("Draft approve");
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        println!("State request_review");
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        println!("State approve");
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        println!("Published content");
        &post.content
    }

    fn request_review(self: Box<Self>) -> Box<State> {
        println!("Published request_review");
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        println!("Published approve");
        self
    }
}

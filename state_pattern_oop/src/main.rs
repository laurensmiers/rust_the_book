use std::cell::RefCell;

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

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().is_text_adjust_allowed() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn is_text_adjust_allowed(&self) -> bool {
        false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn is_text_adjust_allowed(&self) -> bool {
        true
    }
}

struct PendingReview {
    approve_count: RefCell<i32>,
}

impl PendingReview {
    pub fn new() -> PendingReview {
        PendingReview {
            approve_count: RefCell::new(0),
        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self.approve_count.replace_with(|&mut old| old + 1);
        if *self.approve_count.borrow() >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.approve(); // Shouldn't cause approval to go forward!

    post.request_review();
    post.add_text("I ate a burger for lunch today"); //Shouldn't do anything
    assert_eq!("", post.content());

    post.approve(); // First valid approval, we need two so the content should still be empty
    assert_eq!("", post.content());

    post.approve(); // Last approval
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text("I ate nothing for lunch today"); //Shouldn't do anything
    assert_eq!("I ate a salad for lunch today", post.content());
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    approve_count: i32,
}

pub struct Post {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_count: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn get_content(&self) -> &str {
        ""
    }

    pub fn approve(&mut self) -> Option<Post> {
        self.approve_count += 1;
        if self.approve_count >= 2 {
            Some(Post {
                content: self.content.clone(),
            })
        } else {
            None
        }
    }
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}

fn _approve_post(post: &mut PendingReviewPost) -> Option<Post> {
    let mut approve_count = 0;
    for _ in 0..10 {
        approve_count += 1;
        match post.approve() {
            Some(x) => {
                println!("Post approved after {} approvals", approve_count);
                return Some(x);
            }
            None => println!("Not enough approval counts"),
        };
    }

    None
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let mut post = post.request_review();

    let approv_res = _approve_post(&mut post);

    match approv_res {
        Some(post) => assert_eq!("I ate a salad for lunch today", post.get_content()),
        None => panic!("Failed to approve post!"),
    }
}

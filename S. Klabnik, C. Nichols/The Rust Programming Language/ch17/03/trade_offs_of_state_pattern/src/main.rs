pub const MIN_APPROVALS: u8 = 2;

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approvals: u8,
}

impl PendingReviewPost {
    pub fn approve(&mut self) {
        self.approvals += 1;
    }

    pub fn publish(self) -> Option<Post> {
        if self.approvals >= MIN_APPROVALS {
            Some(Post {
                content: self.content,
            })
        } else {
            None
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn get_approvals(&self) -> u8 {
        self.approvals
    }
}

fn main() {
    let start = std::time::Instant::now();
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.reject();
    let mut post = post.request_review();

    while post.get_approvals() < MIN_APPROVALS {
        post.approve();
    }
    let post = post.publish().unwrap();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Time: {:?}", start.elapsed());
}

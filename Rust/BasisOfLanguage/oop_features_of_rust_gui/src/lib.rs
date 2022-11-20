pub trait Draw {
    fn draw(&self);
}

pub trait OnClick {
    fn click(&self);
}

pub struct
Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn
    run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


pub struct
Button {
    pub height: u32,
    pub width : u32,
    pub label : String
}

impl Draw for Button {
    fn draw(&self) {
        todo!()
    }
}

impl OnClick for Button {
    fn click(&self) {

    }
}



pub mod blog {
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn reject(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {""}
    }

    struct Draft {}
    struct PendingReview {}

    struct Published {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview{})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published{})
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft{})
        }
    }
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    pub struct
    Post {
        state: Option<Box<dyn State>>,
        content: String
    }

    impl Post {
        pub fn
        add_text(&mut self, text: &str) {
            self.content.push_str(text);
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

        pub fn reject(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.reject())
            }
        }
    }
}
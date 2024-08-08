pub mod layouts {
    mod center;
    mod row;

    pub use center::CenterBuilder as Center;
    pub use row::RowBuilder as Row;
}

pub mod buttons {
    mod button;

    pub use button::ButtonBuilder as Button;
}

mod text;

pub use text::Text;

#[macro_export]
macro_rules! children {
    ($($x:expr),+ $(,)?) => {
        vec!($(Arc::new($x),)*)
    };
    ($x:expr; $n:expr) => {
        vec!(Arc::new($x); $n)
    };
}

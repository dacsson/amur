pub type Location = (i32, i32);

pub trait Spannable {
    fn get_loc(&self) -> Location;
}

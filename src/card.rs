use crate::face::Face;

#[derive(Debug)]
pub struct Card {
    pub front: Option<Face>,
    pub back: Option<Face>,
}
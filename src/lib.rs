#[macro_use] mod index_vec;

newtype_index!{
    #[derive(Serialize, Deserialize)]
    Layer
}

// (For some reason, both this and the impl seem to be needed for the lockup)
pub trait MetaSift<Targets, Indices> {
    fn sift(&self) -> Targets;
}

impl<List, Targets, Indices> MetaSift<Targets, Indices> for List
    where
        List: Clone,
        List: ::frunk::hlist::Sculptor<Targets, Indices>,
{
    fn sift(&self) -> Targets
    { self.clone().sculpt().0 }
}

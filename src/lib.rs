// (a separate module is necessary for the lockup for some reason)
#[macro_use] mod index_vec;

newtype_index!{
    #[derive(Serialize, Deserialize)]
    Layer
}


pub trait View {
    /// User provided unique identifier for this view. The identifier can be used to navigate using next and previous value.
    fn id(&self) -> usize;
}

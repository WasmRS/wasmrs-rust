pub trait Observer<Item, Err>
where
    Item: Send,
    Err: Send,
{
    fn send(&self, item: Item) -> Result<(), u8>;

    fn error(&self, err: Err) -> Result<(), u8>;

    fn complete(&self);
}

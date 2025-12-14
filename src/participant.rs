pub trait Participant {
    fn id(&self) -> &str;
    fn display_name(&self) -> &str;
    fn avatar_url(&self) -> Option<&str>;
}

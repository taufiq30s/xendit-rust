use serde::Deserialize;

#[derive(Deserialize)]
pub struct Links {
    href: String,
    method: String,
    rel: String,
}
impl Links {
    pub fn get_href(&self) -> &str {
        &self.href
    }
    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn get_rel(&self) -> &str {
        &self.rel
    }
}

#[derive(Deserialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub links: Option<Vec<Links>>,
}
impl<T> ListResponse<T> {
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn has_more(&self) -> bool {
        self.has_more
    }
    pub fn get_links(&self) -> &Option<Vec<Links>> {
        &self.links
    }
}
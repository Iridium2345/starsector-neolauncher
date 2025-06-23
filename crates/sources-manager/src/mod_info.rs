
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ModInfo {
    name: String,
    version: String,
    url: String,
    date: String,
    author: String,
}
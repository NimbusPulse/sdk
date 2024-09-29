use uuid::Uuid;

#[derive(Debug)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub port: u32,
    pub max_cpu: u32,
    pub max_ram: u32,
}

//! libvirt model — virtualization management API.

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub state: String,
    pub memory_kb: u64,
    pub vcpus: u32,
    pub autostart: bool,
}

#[derive(Debug, Clone)]
pub struct Network {
    pub name: String,
    pub bridge: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub domains: Vec<Domain>,
    pub networks: Vec<Network>,
}

impl Default for Model {
    fn default() -> Self { Self { domains: vec![], networks: vec![
        Network { name: "default".into(), bridge: "virbr0".into(), active: true },
    ] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn define(&mut self, name: &str, mem: u64, vcpus: u32) -> bool {
        self.domains.push(Domain { name: name.into(), state: "shut off".into(), memory_kb: mem, vcpus, autostart: false });
        true
    }

    pub fn list_domains(&self) -> &[Domain] { &self.domains }
    pub fn list_networks(&self) -> &[Network] { &self.networks }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn define_list() {
        let mut l = Model::new();
        assert!(l.define("vm1", 2097152, 2));
        assert_eq!(l.list_domains().len(), 1);
        assert_eq!(l.list_networks().len(), 1);
    }
}

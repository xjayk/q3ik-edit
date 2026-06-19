pub struct SshConnectionOptions;

pub enum SshPortForwardOption {
    Local(Vec<String>),
    Remote(Vec<String>),
    Dynamic(Vec<String>),
}

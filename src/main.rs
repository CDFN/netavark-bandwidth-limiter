use netavark::{
    network::types::{self},
    plugin::{Info, Plugin, PluginExec, API_VERSION},
};

fn main() {
    // change the version to the version of your plugin
    let info = Info::new("0.1.0".to_owned(), API_VERSION.to_owned(), None);

    PluginExec::new(Exec {}, info).exec();
}

struct Exec {}  

impl Plugin for Exec {
    fn create(
        &self,
        network: types::Network,
    ) -> Result<types::Network, Box<dyn std::error::Error>> {
        return Ok(network)
    }

    fn setup(
        &self,
        _netns: String,
        _opts: types::NetworkPluginExec,
    ) -> Result<types::StatusBlock, Box<dyn std::error::Error>> {
        return Ok(types::StatusBlock {
            dns_search_domains: None,
            dns_server_ips: None,
            interfaces: None,
        });
    }

    fn teardown(
        &self,
        _netns: String,
        _opts: types::NetworkPluginExec,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // your logic here
        return Ok(());
    }
}

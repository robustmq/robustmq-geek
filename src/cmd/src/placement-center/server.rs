use clap::command;
use clap::Parser;
use common_base::config::placement_center::init_placement_center_conf_by_path;
use common_base::config::placement_center::placement_center_conf;

pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";

#[derive(Parser, Debug)]
#[command(author="robustmq-geek", version="0.0.1", about=" RobustMQ: Next generation cloud-native converged high-performance message queue.", long_about = None)]
#[command(next_line_help = true)]

struct ArgsParams {
    /// MetaService Indicates the path of the configuration file
    #[arg(short, long, default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf: String,
}

fn main() {
    let args = ArgsParams::parse();
    init_placement_center_conf_by_path(&args.conf);
    println!("{:?}",placement_center_conf());
}

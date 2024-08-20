// Copyright 2023 RobustMQ Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::info;

use crate::{
    config::placement_center::placement_center_conf,
    tools::{create_fold, file_exists, read_file},
};

pub fn init_placement_center_log(path: String, segment_log_size: u64, log_fie_count: u32) {
    let conf = placement_center_conf();
    if !file_exists(&conf.log.log_config) {
        panic!(
            "Logging configuration file {} does not exist",
            conf.log.log_config
        );
    }

    match create_fold(&conf.log.log_path) {
        Ok(()) => {}
        Err(e) => {
            panic!("Failed to initialize log directory {}", conf.log.log_path);
        }
    }

    let content = match read_file(&conf.log.log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    let config_content = content.replace("{$path}", &path);
    println!("{}", config_content);

    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!(
                "Failed to parse the contents of the config file {} with error message :{}",
                conf.log.log_config,
                e.to_string()
            );
        }
    };

    match log4rs::init_raw_config(config) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}

// pub fn init_placement_center_log(path: String, segment_log_size: u64, log_fie_count: u32) {
//     let stdout = ConsoleAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(
//             "{d(%Y-%m-%d %H:%M:%S)} {f}:{L} {h({l})} {m}{n}",
//         )))
//         .build();

//     let server_log = RollingFileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(
//             "{d(%Y-%m-%d %H:%M:%S)} {f}:{L} {h({l})} {m}{n}",
//         )))
//         .append(true)
//         .build(
//             format!("{}/server.log", path),
//             Box::new(CompoundPolicy::new(
//                 Box::new(SizeTrigger::new(segment_log_size)),
//                 Box::new(
//                     FixedWindowRoller::builder()
//                         .base(0)
//                         .build(&format!("{}/server.{}.log", path, "{}"), log_fie_count)
//                         .unwrap(),
//                 ),
//             )),
//         )
//         .unwrap();

//     let requests_log = RollingFileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new(
//             "{d(%Y-%m-%d %H:%M:%S)} {f}:{L} {h({l})} {m}{n}",
//         )))
//         .append(true)
//         .build(
//             format!("{}/requests-log.log", path),
//             Box::new(CompoundPolicy::new(
//                 Box::new(SizeTrigger::new(segment_log_size)),
//                 Box::new(
//                     FixedWindowRoller::builder()
//                         .base(0)
//                         .build(
//                             &format!("{}/requests-log.{}.log", path, "{}"),
//                             log_fie_count,
//                         )
//                         .unwrap(),
//                 ),
//             )),
//         )
//         .unwrap();

//     let config = Config::builder()
//         .appender(Appender::builder().build("stdout", Box::new(stdout)))
//         .appender(Appender::builder().build("server", Box::new(server_log)))
//         .appender(Appender::builder().build("requests-log", Box::new(requests_log)))
//         .logger(
//             Logger::builder()
//                 .appender("server")
//                 .appender("stdout")
//                 .additive(false)
//                 .build("placement_center::server", LevelFilter::Info),
//         )
//         .logger(
//             Logger::builder()
//                 .appender("requests-log")
//                 .appender("stdout")
//                 .additive(false)
//                 .build("placement_center::requests", LevelFilter::Info),
//         )
//         .build(
//             Root::builder()
//                 .appender("stdout")
//                 .appender("server")
//                 .build(LevelFilter::Info),
//         )
//         .unwrap();
//     match log4rs::init_config(config) {
//         Ok(_) => {}
//         Err(e) => {
//             panic!("{}", e.to_string());
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_print() {}
}

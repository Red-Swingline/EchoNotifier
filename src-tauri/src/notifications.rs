use dbus::arg::ArgType;
use dbus::{
    blocking::Connection,
    message::{MatchRule, Message},
};
use log::{error, info};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::config;

pub struct NotificationHandler {
    last_notifications: HashMap<String, (String, Instant)>,
    debounce_duration: Duration,
}

impl NotificationHandler {
    pub fn new(debounce_duration: Duration) -> Self {
        NotificationHandler {
            last_notifications: HashMap::new(),
            debounce_duration,
        }
    }

    pub fn handle_notification(&mut self, app_name: &str, content: &str, sound_path: &str) {
        let now = Instant::now();
        let notification_key = format!("{}:{}", app_name, content);

        if let Some((_, last_time)) = self.last_notifications.get(&notification_key) {
            if now.duration_since(*last_time) < self.debounce_duration {
                info!("Skipping sound for '{}', debounce period.", app_name);
                return;
            }
        }

        info!("Playing sound for {}: {}", app_name, sound_path);
        self.play_sound(sound_path);

        self.last_notifications
            .insert(notification_key, (content.to_string(), now));
    }

    fn play_sound(&self, sound_path: &str) {
        let sound_extension = Path::new(sound_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match sound_extension {
            "wav" => {
                Command::new("aplay")
                    .arg(sound_path)
                    .output()
                    .expect("Failed to play WAV sound");
            }
            "mp3" => {
                Command::new("mpg321")
                    .arg(sound_path)
                    .output()
                    .expect("Failed to play MP3 sound");
            }
            _ => {
                error!("Unsupported sound format for file: {}", sound_path);
            }
        }
    }
}

fn get_app_name_from_message(msg: &Message) -> Option<String> {
    let mut iter = msg.iter_init();
    if iter.arg_type() == ArgType::String {
        iter.get::<&str>().map(|s| s.to_string())
    } else {
        None
    }
}

fn get_notification_content(_msg: &Message) -> String {
    "notification content".to_string()
}
pub fn start_notification_listener(config_path: PathBuf) {
    let handler = Arc::new(Mutex::new(NotificationHandler::new(Duration::from_secs(3)))); 

    thread::spawn(move || {
        let conn = Connection::new_session().expect("Failed to start D-Bus session");
        let rule = MatchRule::new_method_call()
            .with_interface("org.freedesktop.Notifications")
            .with_eavesdrop();

        conn.add_match(rule, move |_: (), _, msg: &Message| {
            let config = match config::read_config(&config_path.to_string_lossy()) {
                Ok(cfg) => cfg,
                Err(err) => {
                    eprintln!("Failed to read config: {}", err);
                    return true;
                }
            };

            let sound_map: HashMap<_, _> = config
                .apps
                .iter()
                .map(|asc| (asc.app.clone(), asc.sound_path.clone()))
                .collect();

            let mut handler = handler.lock().unwrap();
            if handler.debounce_duration != Duration::from_secs(config.app_settings.debounce_period) {
                handler.debounce_duration = Duration::from_secs(config.app_settings.debounce_period);
            }

            if let Some(app_name) = get_app_name_from_message(msg) {
                if let Some(sound_path) = sound_map.get(&app_name) {
                    let content = get_notification_content(msg);
                    handler.handle_notification(&app_name, &content, sound_path);
                }
            }

            true
        })
        .expect("Failed to add match rule");

        loop {
            conn.process(Duration::from_millis(100))
                .expect("Failed to process D-Bus connection");
        }
    });
}

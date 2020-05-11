use telegram_bot::{prelude::*, Api, Message, MessageKind, ReplyKeyboardMarkup, Error, ParseMode, KeyboardButton, UserId};
use std::env;
use crate::word_generator::WordGenerator;
use std::collections::HashMap;
use chrono::{Utc, Datelike};
use crate::stats::Stats;

pub struct Client {
    pub(crate) api: Api,
    word_generator: WordGenerator,
    open_replies: HashMap<UserId, (usize, String)>,
    statistics: HashMap<UserId, Stats>,
}

impl Client {
    pub async fn new() -> Self {
        let api_key = env::var("API_KEY").expect("Missing API_KEY.");
        let file_path = env::var("WORDS_PATH").expect("Missing WORDS_PATH");
        Client {
            api: Api::new(api_key),
            word_generator: WordGenerator::from_file(file_path),
            open_replies: HashMap::new(),
            statistics: HashMap::new(),
        }
    }

    pub(crate) async fn process_msg(&mut self, msg: Message) -> Result<(), Error> {
        if let MessageKind::Text { ref data, .. } = msg.kind {
            let user: i64 = msg.from.id.into();

            println!("{}: {}", user, data);

            let response: Vec<&str> = data.as_str().splitn(2, ' ').collect();
            match response[0] {
                "/new" | "/next" => {
                    let word = self.word_generator.clone().random_word().to_uppercase();
                    let chars = String::from(WordGenerator::word_shuffle(&word))
                        .chars()
                        .map(|c| format!("{}", c))
                        .collect::<Vec<String>>()
                        .join(" ");
                    let (options, correct_option) = WordGenerator::answer_options(&word);
                    //self.db.start_working(user as i32, date).await.expect("Insert failed");
                    let keyboard = ReplyKeyboardMarkup::from(vec![
                        vec![KeyboardButton::new(format!("/A {}", options[0]))],
                        vec![KeyboardButton::new(format!("/B {}", options[1]))],
                        vec![KeyboardButton::new(format!("/C {}", options[2]))],
                        vec![KeyboardButton::new(format!("/D {}", options[3]))],
                        vec![KeyboardButton::new("/E Keine der genannten Optionen")],
                    ]);

                    self.api.send(msg
                        .text_reply(format!("*{}*", chars))
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(keyboard)
                    ).await.expect("Msg could not be send.");

                    self.open_replies.insert(msg.from.id, (correct_option, word));
                    if !self.statistics.contains_key(&msg.from.id) {
                        self.statistics.insert(msg.from.id.clone(), Stats::default());
                    }
                }
                "/A" => self.respond_to_answer(0, &msg).await,
                "/B" => self.respond_to_answer(1, &msg).await,
                "/C" => self.respond_to_answer(2, &msg).await,
                "/D" => self.respond_to_answer(3, &msg).await,
                "/E" => self.respond_to_answer(4, &msg).await,
                "/stats" => {
                    if !self.statistics.contains_key(&msg.from.id) {
                        self.statistics.insert(msg.from.id.clone(), Stats::default());
                    }

                    let stats = self.statistics.get(&msg.from.id).unwrap();
                    self.api.send(msg
                        .text_reply(stats.stats())
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(Self::standard_keyboard())
                    ).await.expect("Msg could not be send.");
                }
                "/reset_stats" => {
                    if let Some(stats) = self.statistics.get_mut(&msg.from.id) {
                        stats.reset();
                    }
                    self.api.send(msg
                        .text_reply("Statistiken sind zurückgesetzt!")
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(Self::standard_keyboard())
                    ).await.expect("Msg could not be send.");
                }
                "/help" => {
                    self.api.send(msg
                        .text_reply(Self::help())
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(Self::standard_keyboard())
                    ).await.expect("Msg could not be send.");
                }
                "/version" => {
                    self.api.send(
                        msg.text_reply(Self::version())
                            .parse_mode(ParseMode::Markdown)
                            .reply_markup(Self::standard_keyboard())
                    ).await.expect("Msg could not be send.");
                }
                _ => {
                    self.api.send(msg
                        .text_reply(format!("Unbekanntes Kommando: {}", data))
                        .reply_markup(Self::standard_keyboard())
                    ).await.expect("Msg could not be send.");
                }
            }
        }
        Ok(())
    }

    async fn respond_to_answer(&mut self, option: usize, msg: &Message) {
        if let MessageKind::Text { data: _, .. } = msg.kind {
            let correct_value = self.open_replies.get(&msg.from.id);
            let answer = if let Some((correct_option, correct_word)) = correct_value {
                let stats = self.statistics.get_mut(&msg.from.id).unwrap();
                if *correct_option == option {
                    stats.add_correct();
                    format!("*Korrekt!*\nDas Wort war: {}", correct_word)
                } else {
                    stats.add_wrong();
                    format!("*Falsch!*\nDas Wort war: {}", correct_word)
                }
            } else {
                format!("Error!")
            };

            let keyboard = ReplyKeyboardMarkup::from(vec![
                vec![KeyboardButton::new("/next")],
                vec![KeyboardButton::new("/stats"), KeyboardButton::new("/help")],
            ]);

            self.api.send(msg
                .text_reply(answer)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(keyboard)
            ).await.expect("Msg could not be send.");
        }
    }

    fn standard_keyboard() -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::from(vec![
            vec![KeyboardButton::new("/new")],
            vec![KeyboardButton::new("/stats"), KeyboardButton::new("/reset_stats")],
            vec![KeyboardButton::new("/help"), KeyboardButton::new("/version")],
        ])
    }

    fn help() -> String {
        "Dieser Bot bietet folgende Kommandos:\n\
        - /new, /next: Nächstes Wort fragen\n\
        - /stats: Statistiken anzeigen\n\
        - /reset\\_stats: Statistiken zurücksetzen\n\
        - /help: Ausgabe dieses Hilfe-Textes\n\
        - /version: Versions-Informationen".to_string()
    }

    fn version() -> String {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
        //const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
        format!("Version: *{}*\n(c) {} by {}", VERSION, Utc::now().date().year(), AUTHORS)
    }
}
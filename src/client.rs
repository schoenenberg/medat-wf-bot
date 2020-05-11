use telegram_bot::{prelude::*, Api, Message, MessageKind, ReplyKeyboardMarkup, Error, ParseMode, KeyboardButton, UserId};
use std::env;
use crate::word_generator::WordGenerator;
use std::collections::HashMap;

pub struct Client {
    pub(crate) api: Api,
    word_generator: WordGenerator,
    open_replies: HashMap<UserId, (usize, String)>,
}

impl Client {
    pub async fn new() -> Self {
        let api_key = env::var("API_KEY").expect("Missing API_KEY.");
        let file_path = env::var("WORDS_PATH").expect("Missing WORDS_PATH");
        Client {
            api: Api::new(api_key),
            word_generator: WordGenerator::from_file(file_path),
            open_replies: HashMap::new(),
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

                    let status = self.api.send(msg
                        .text_reply(format!("*{}*", chars))
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(keyboard)
                    ).await;

                    self.open_replies.insert(msg.from.id, (correct_option, word));

                    println!("{:?}", status)
                }
                "/A" => {
                    self.respond_to_answer(0, &msg).await;
                }
                "/B" => {
                    self.respond_to_answer(1, &msg).await;
                }
                "/C" => {
                    self.respond_to_answer(2, &msg).await;
                }
                "/D" => {
                    self.respond_to_answer(3, &msg).await;
                }
                "/E" => {
                    self.respond_to_answer(4, &msg).await;
                }
                "/help" => {
                    let keyboard = ReplyKeyboardMarkup::from(vec![
                        vec![KeyboardButton::new("/new")],
                        vec![KeyboardButton::new("/stats"), KeyboardButton::new("/reset-stats")],
                        vec![KeyboardButton::new("/help"), KeyboardButton::new("/version")],
                    ]);

                    let status = self.api.send(msg
                        .text_reply(
                            "Dieser Bot bietet folgende Kommandos:\n\
                            - /new, /next: Nächstes Wort fragen\n\
                            - /stats: Statistiken anzeigen\n\
                            - /reset-stats: Statistiken zurücksetzen\n\
                            - /help: Ausgabe dieses Hilfe-Textes\n\
                            - /version: Versions-Informationen")
                        .parse_mode(ParseMode::Markdown)
                        .reply_markup(keyboard)
                    ).await;
                    println!("{:?}", status)
                }
                _ => {
                    self.api.send(msg.text_reply(
                        format!("Unknown command: {}", data))
                    ).await?;
                }
            }
        }
        Ok(())
    }

    async fn respond_to_answer(&self, option: usize, msg: &Message) {
        if let MessageKind::Text { data: _, .. } = msg.kind {
            let correct_value = self.open_replies.get(&msg.from.id);
            let answer = if let Some((correct_option, correct_word)) = correct_value {
                if *correct_option == option {
                    format!("*Korrekt!*\nDas Wort war: {}", correct_word)
                } else {
                    format!("*Falsch!*\nDas Wort war: {}", correct_word)
                }
            } else {
                format!("Error!")
            };

            let keyboard = ReplyKeyboardMarkup::from(vec![
                vec![KeyboardButton::new("/next")],
                vec![KeyboardButton::new("/stats"), KeyboardButton::new("/help")],
            ]);

            let status = self.api.send(msg
                .text_reply(answer)
                .parse_mode(ParseMode::Markdown)
                .reply_markup(keyboard)
            ).await;

            println!("{:?}", status);
        }
    }
}
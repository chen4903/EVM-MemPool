use std::{
    env,
    io::{stdin, stdout, Write},
};
use colored::*;
use dotenv::dotenv;

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};

pub async fn openai() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().ok();
    set_key(env::var("OPENAI_KEY").unwrap());
    set_base_url(env::var("OPENAI_BASE_URL").unwrap_or_default());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You are a large language model built into a command line interface as an example of what the `openai` Rust library made by Valentine Briese can do.".to_string()),
        name: None,
        function_call: None,
    }];

    loop {
        println!();
        print!("{}","User: ".green());
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(user_message_content),
            name: None,
            function_call: None,
        });

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .create()
            .await
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        println!(
            "ChatGPT: {}",
            &returned_message.content.clone().unwrap().trim()
        );

        messages.push(returned_message);
    }
}
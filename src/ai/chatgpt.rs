use std::io::{stdin, stdout, Write};
use colored::*;
use dotenv::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};

pub struct AI {
    pub OPENAI_KEY: String,
    pub OPENAI_BASE_URL: String,
}

impl AI {
    
    /// @param openai_key OpenAI API KEY
    /// @param openai_base_url A domestic(For china) proxy springboard for accessing OpenAI
    pub fn new(openai_key: String, openai_base_url: String) -> Self {
        AI {
            OPENAI_KEY: openai_key,
            OPENAI_BASE_URL: openai_base_url,
        }
    }

    /// @dev Create a chatgpt client
    pub async fn chatgpt(&self) {
        dotenv().ok();
        set_key(self.OPENAI_KEY.clone());
        set_base_url(self.OPENAI_BASE_URL.clone());
    
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
}

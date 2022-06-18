use serenity::builder::{
    CreateActionRow, CreateButton, CreateComponents, CreateInputText, CreateSelectMenu
};
use serenity::model::interactions::message_component::{ButtonStyle, InputTextStyle};
use serenity::prelude::TypeMapKey;

impl TypeMapKey for UI {
    type Value = UI;
}

use crate::god::GodMemoryConfig;
use std::collections::hash_map::Values;

pub struct UI {
    main_menu: CreateComponents,
    change_name: CreateComponents,
    change_context: CreateComponents,
    add_interaction: CreateComponents,
    change_config: CreateComponents,
}

impl UI {
    fn build_main_menu() -> CreateComponents {
        let mut change_name_btn = CreateButton::default();
        change_name_btn.style(ButtonStyle::Primary);
        change_name_btn.label("Change name");
        change_name_btn.custom_id("change_name");

        let mut change_context_btn = CreateButton::default();
        change_context_btn.style(ButtonStyle::Primary);
        change_context_btn.label("Change context");
        change_context_btn.custom_id("change_context");

        let mut add_interactin_btn = CreateButton::default();
        add_interactin_btn.style(ButtonStyle::Primary);
        add_interactin_btn.label("Add an interaction");
        add_interactin_btn.custom_id("add_interaction");

        let mut clear_interactions_btn = CreateButton::default();
        clear_interactions_btn.style(ButtonStyle::Danger);
        clear_interactions_btn.label("Clear interactions");
        clear_interactions_btn.custom_id("clear_interactions");

        let mut save_btn = CreateButton::default();
        save_btn.style(ButtonStyle::Success);
        save_btn.label("Save the god");
        save_btn.custom_id("save");

        let mut action_select = CreateActionRow::default();
        action_select.add_button(change_name_btn);
        action_select.add_button(change_context_btn);
        action_select.add_button(add_interactin_btn);
        action_select.add_button(clear_interactions_btn);
        action_select.add_button(save_btn);

        let mut c = CreateComponents::default();
        c.add_action_row(action_select);
        c
    }

    pub fn get_main_menu(&self) -> CreateComponents {
        self.main_menu.clone()
    }

    fn build_change_name() -> CreateComponents {
        let mut name_request = CreateInputText::default();
        name_request.label("New god name");
        name_request.placeholder("Kirby");
        name_request.style(InputTextStyle::Short);
        name_request.max_length(50);
        name_request.custom_id("god_name");

        let mut ar_name_change = CreateActionRow::default();
        ar_name_change.add_input_text(name_request);

        let mut c = CreateComponents::default();
        c.add_action_row(ar_name_change);
        c
    }

    pub fn get_change_name(&self) -> CreateComponents {
        self.change_name.clone()
    }

    fn build_change_context() -> CreateComponents {
        let mut context_request = CreateInputText::default();
        context_request.label("Context:");
        context_request.placeholder("Kirby is the god of all beings. Yet, he is the most lovely god and answers in a very complete.");
        context_request.style(InputTextStyle::Paragraph);
        context_request.min_length(3);
        context_request.max_length(500);
        context_request.custom_id("context");

        let mut ar_context = CreateActionRow::default();
        ar_context.add_input_text(context_request);

        let mut c = CreateComponents::default();
        c.add_action_row(ar_context);
        c
    }

    pub fn get_change_context(&self) -> CreateComponents {
        self.change_context.clone()
    }

    fn build_add_interaction() -> CreateComponents {
        let mut author = CreateInputText::default();
        author.label("Author:");
        author.placeholder("AlexisTM");
        author.style(InputTextStyle::Short);
        author.max_length(50);
        author.required(true);
        author.custom_id("author");

        let mut prompt = CreateInputText::default();
        prompt.label("Request:");
        prompt.placeholder("Who is god?");
        prompt.style(InputTextStyle::Paragraph);
        prompt.min_length(3);
        prompt.max_length(500);
        prompt.required(true);
        prompt.custom_id("prompt");

        let mut answer = CreateInputText::default();
        answer.label("Answer:");
        answer.placeholder("Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!");
        answer.style(InputTextStyle::Paragraph);
        answer.min_length(3);
        answer.max_length(500);
        answer.required(true);
        answer.custom_id("answer");

        let mut ar_author = CreateActionRow::default();
        ar_author.add_input_text(author);
        let mut ar_prompt = CreateActionRow::default();
        ar_prompt.add_input_text(prompt);
        let mut ar_response = CreateActionRow::default();
        ar_response.add_input_text(answer);

        let mut c = CreateComponents::default();
        c.add_action_row(ar_author);
        c.add_action_row(ar_prompt);
        c.add_action_row(ar_response);
        c
    }

    pub fn get_add_interaction(&self) -> CreateComponents {
        self.add_interaction.clone()
    }

    pub fn build_change_config(&mut self, bots: Values<String, GodMemoryConfig>) {
        let mut select_menu = CreateSelectMenu::default();
        select_menu.custom_id("change_config");
        select_menu.options(|options| {
            for bot in bots {
                options.create_option(|option| {
                    option.label(bot.botname.clone())
                    .value(bot.botname.clone())
                    .description(bot.context.clone())
                });
            }
            options
        });

        let mut context_request = CreateInputText::default();
        context_request.label("Context:");
        context_request.placeholder("Kirby is the god of all beings. Yet, he is the most lovely god and answers in a very complete.");
        context_request.style(InputTextStyle::Paragraph);
        context_request.min_length(3);
        context_request.max_length(500);
        context_request.custom_id("context");

        let mut ar_context = CreateActionRow::default();
        ar_context.add_input_text(context_request);

        let mut c = CreateComponents::default();
        c.add_action_row(ar_context);
        self.change_config = c;
    }

    pub fn get_change_config(&self) -> CreateComponents {
        self.change_config.clone()
    }
}

impl Default for UI {
    fn default() -> Self {
        UI {
            main_menu: Self::build_main_menu(),
            change_name: Self::build_change_name(),
            change_context: Self::build_change_context(),
            add_interaction: Self::build_add_interaction(),
            change_config: CreateComponents::default(),
        }
    }
}

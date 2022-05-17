use serenity::builder::{CreateActionRow, CreateButton, CreateComponents, CreateInputText};
use serenity::model::interactions::message_component::{ButtonStyle, InputTextStyle};
use serenity::prelude::TypeMapKey;

impl TypeMapKey for UI {
    type Value = UI;
}

pub struct UI {
    main_menu: CreateComponents,
    change_name: CreateComponents,
    change_context: CreateComponents,
    add_interaction: CreateComponents,
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
        clear_interactions_btn.style(ButtonStyle::Primary);
        clear_interactions_btn.label("Clear interactions");
        clear_interactions_btn.custom_id("clear_interactions");

        let mut save_btn = CreateButton::default();
        save_btn.style(ButtonStyle::Primary);
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
        name_request.custom_id("god_name");

        let mut name_change_ar = CreateActionRow::default();
        name_change_ar.add_input_text(name_request);

        let mut c = CreateComponents::default();
        c.add_action_row(name_change_ar);
        c
    }

    pub fn get_change_name(&self) -> CreateComponents {
        self.change_name.clone()
    }

    fn build_change_context() -> CreateComponents {
        let mut context_request = CreateInputText::default();
        context_request.label("Context:");
        context_request.placeholder("Kirby is the god of all beings. Yet, he is the most lovely god and answers in a very complete manner.");
        context_request.style(InputTextStyle::Paragraph);
        context_request.custom_id("context");

        let mut name_change_ar = CreateActionRow::default();
        name_change_ar.add_input_text(context_request);

        let mut c = CreateComponents::default();
        c.add_action_row(name_change_ar);
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
        author.custom_id("author");

        let mut request = CreateInputText::default();
        request.label("Request:");
        request.placeholder("Who is god?");
        request.style(InputTextStyle::Paragraph);
        request.custom_id("request");

        let mut answer = CreateInputText::default();
        answer.label("Answer:");
        answer.placeholder("Well, now that you ask, I can tell you. I, Kirby is the great goddess is the god of everybody!");
        answer.style(InputTextStyle::Paragraph);
        answer.custom_id("answer");

        let mut name_change_ar = CreateActionRow::default();
        name_change_ar.add_input_text(author);
        name_change_ar.add_input_text(request);
        name_change_ar.add_input_text(answer);

        let mut c = CreateComponents::default();
        c.add_action_row(name_change_ar);
        c
    }

    pub fn get_add_interaction(&self) -> CreateComponents {
        self.add_interaction.clone()
    }
}

impl Default for UI {
    fn default() -> Self {
        UI {
            main_menu: Self::build_main_menu(),
            change_name: Self::build_change_name(),
            change_context: Self::build_change_context(),
            add_interaction: Self::build_add_interaction(),
        }
    }
}

use tui::{
    backend::Backend,
    text::Spans,
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::service::AppService;

pub struct AppUI {
    service: AppService,
    count: usize,
}

impl AppUI {
    pub fn new(service: AppService) -> AppUI {
        AppUI { service, count: 0 }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let mut text = Vec::new();
        if let Ok(data) = self.service.data.lock() {
            if let Some(articles) = data["articles"].as_array() {
                let size = articles.len();
                for article in articles.iter().cycle().skip(self.count).take(size) {
                    if let Some(title) = article["title"].as_str() {
                        text.push(Spans::from(String::from(title)));
                    }
                    if let Some(description) = article["description"].as_str() {
                        text.push(Spans::from(String::from(description)));
                    }
                    text.push(Spans::default());
                }
            }
        }
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
        f.render_widget(paragraph, f.size());

        self.count += 1;
    }
}

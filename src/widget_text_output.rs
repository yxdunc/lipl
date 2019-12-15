use tui::Frame;
use tui::backend::TermionBackend;
use termion::raw::RawTerminal;
use std::io::Stdout;
use tui::widgets::{Paragraph, Text, Block, Widget};
use std::borrow::Cow;
use tui::layout::Alignment;

pub fn text_output(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, text_result_history: &Vec<(f64, String)>) {
    if !text_result_history.is_empty() {
        let last_result = text_result_history.last().unwrap();
        Paragraph::new(vec!(Text::Raw(Cow::from(&last_result.1))).iter())
            .block(Block::default().title(&format!("Elapsed time: {:.*} seconds", 2, last_result.0)))
            .alignment(Alignment::Left)
            .render(frame, frame.size());
    }
}

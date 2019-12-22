use std::borrow::Cow;
use std::io::Stdout;
use termion::raw::RawTerminal;
use tui::backend::TermionBackend;
use tui::layout::Alignment;
use tui::widgets::{Block, Paragraph, Text, Widget};
use tui::Frame;

pub fn text_output(frame: &mut Frame<TermionBackend<RawTerminal<Stdout>>>, text_result_history: &Vec<(f64, String)>) {
    if let Some(last_result) = text_result_history.last() {
        Paragraph::new(vec![Text::Raw(Cow::from(&last_result.1))].iter())
            .block(
                Block::default().title(&format!("Elapsed time: {:.*} seconds", 2, last_result.0)),
            )
            .alignment(Alignment::Left)
            .render(frame, frame.size());
    }
}

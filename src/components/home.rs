use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
};

#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
}

impl Home {
  pub fn new() -> Self {
    Self::default()
  }
}

impl Component for Home {
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {},
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let chunks = Layout::default()
      .direction(Direction::Horizontal)
      .margin(1)
      .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
      .split(f.size());
    let b = Block::default()
      .borders(Borders::ALL)
      .title(block::Title::from("KOSMONAUT").alignment(Alignment::Center))
      .border_type(BorderType::Rounded);
    f.render_widget(b, chunks[0]);

    let items = [ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    let l = List::new(items)
      .block(Block::default().title("SSH CONNECTIONS").borders(Borders::ALL).border_type(BorderType::Rounded) )
      .style(Style::default().fg(Color::White))
      .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
      .highlight_symbol(">>");
    f.render_widget(l, chunks[1]);
    Ok(())
  }
}

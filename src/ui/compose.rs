// This file is part of Himalaya TUI, a TUI to manage emails.
//
// Copyright (C) 2025-2026 soywod <pimalaya.org@posteo.net>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use edtui::{EditorTheme, EditorView};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

use crate::app::{App, Panel};

use super::get_border_style;

pub fn render_compose(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" Compose (Esc: actions) ")
        .borders(Borders::ALL)
        .border_style(get_border_style(app, Panel::Compose));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Create a theme that matches the TUI styling
    let theme = EditorTheme::default()
        .base(Style::default().bg(Color::Reset).fg(Color::White))
        .cursor_style(Style::default().bg(Color::White).fg(Color::Black))
        .selection_style(Style::default().bg(Color::Cyan).fg(Color::Black))
        .hide_status_line();

    let buf = frame.buffer_mut();
    EditorView::new(&mut app.editor_state)
        .theme(theme)
        .render(inner, buf);
}

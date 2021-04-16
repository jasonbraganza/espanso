/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct RenderingRequestedEvent {
  pub match_id: i32,
  pub trigger: String,
  pub trigger_args: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderedEvent {
  pub trigger: String,
  pub body: String,

  pub cursor_hint_back_count: Option<usize>,
}
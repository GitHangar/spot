use std::convert::Into;
use crate::app::models::*;
use crate::app::state::AppAction;
use super::UpdatableState;

#[derive(Clone, Debug)]
pub enum BrowserAction {
    SetContent(Vec<AlbumDescription>),
    AppendContent(Vec<AlbumDescription>),
    NavigateToDetails,
    SetDetails(AlbumDescription),
    GoBack,
    Search(String)
}

impl Into<AppAction> for BrowserAction {
    fn into(self) -> AppAction {
        AppAction::BrowserAction(self)
    }
}

#[derive(Clone, Debug)]
pub enum BrowserEvent {
    ContentSet,
    ContentAppended(usize),
    NavigatedToDetails,
    DetailsLoaded,
    NavigationPopped,
    NavigatedToSearch
}

#[derive(Clone)]
pub struct DetailsState {
    pub content: Option<AlbumDescription>
}

impl Default for DetailsState {
    fn default() -> Self {
        Self { content: None }
    }
}

impl UpdatableState for DetailsState {

    type Action = BrowserAction;
    type Event = BrowserEvent;

    fn update_with(&mut self, action: Self::Action) -> Vec<Self::Event> {
        match action {
            BrowserAction::SetDetails(album) => {
                self.content = Some(album);
                vec![BrowserEvent::DetailsLoaded]
            },
            _ => vec![]
        }
    }
}

#[derive(Clone)]
pub struct LibraryState {
    pub page: u32,
    pub albums: Vec<AlbumDescription>
}

impl Default for LibraryState {
    fn default() -> Self {
        Self { page: 0, albums: vec![] }
    }
}

impl UpdatableState for LibraryState {

    type Action = BrowserAction;
    type Event = BrowserEvent;

    fn update_with(&mut self, action: Self::Action) -> Vec<Self::Event> {
        match action {
            BrowserAction::SetContent(content) if content != self.albums => {
                self.page = 1;
                self.albums = content;
                vec![BrowserEvent::ContentSet]
            },
            BrowserAction::AppendContent(mut content) => {
                self.page += 1;
                let append_index = self.albums.len();
                self.albums.append(content.as_mut());
                vec![BrowserEvent::ContentAppended(append_index)]
            },
            _ => vec![]
        }
    }
}

#[derive(Clone)]
pub struct SearchState {
    query: String
}

impl SearchState {

    fn new(query: String) -> Self {
        Self { query }
    }
}

impl UpdatableState for SearchState {

    type Action = BrowserAction;
    type Event = BrowserEvent;

    fn update_with(&mut self, action: Self::Action) -> Vec<Self::Event> {
        match action {
            BrowserAction::Search(query) => {
                println!("searching {}", &query[..]);
                self.query = query;
                vec![]
            }
            _ => vec![]
        }
    }
}


pub enum BrowserScreen {
    Library(LibraryState),
    Details(DetailsState),
    Search(SearchState)
}

impl BrowserScreen {

    fn state(&mut self) -> &mut dyn UpdatableState<Action=BrowserAction, Event=BrowserEvent> {
        match self {
            Self::Library(state) => state,
            Self::Details(state) => state,
            Self::Search(state) => state
        }
    }
}

pub struct BrowserState {
    pub navigation: Vec<BrowserScreen>
}

impl BrowserState {

    pub fn new() -> Self {
        Self { navigation: vec![BrowserScreen::Library(Default::default())] }
    }

    pub fn library_state(&self) -> Option<&LibraryState> {
        self.navigation.iter().find_map(|screen| {
            match screen {
                BrowserScreen::Library(state) => Some(state),
                _ => None
            }
        })
    }

    pub fn details_state(&self) -> Option<&DetailsState> {
        self.navigation.iter().find_map(|screen| {
            match screen {
                BrowserScreen::Details(state) => Some(state),
                _ => None
            }
        })
    }

    pub fn search_state(&self) -> Option<&SearchState> {
        self.navigation.iter().find_map(|screen| {
            match screen {
                BrowserScreen::Search(state) => Some(state),
                _ => None
            }
        })
    }
}

impl UpdatableState for BrowserState {

    type Action = BrowserAction;
    type Event = BrowserEvent;

    fn update_with(&mut self, action: Self::Action) -> Vec<Self::Event> {

        let len = self.navigation.len();
        let current = self.navigation.last_mut().unwrap();
        let current_state = current.state();

        match action {
            BrowserAction::Search(query) => {
                if let BrowserScreen::Search(state) = current {
                    state.update_with(BrowserAction::Search(query))
                } else {
                    self.navigation.push(BrowserScreen::Search(SearchState::new(query)));
                    vec![BrowserEvent::NavigatedToSearch]
                }
            },
            BrowserAction::NavigateToDetails => {
                self.navigation.push(BrowserScreen::Details(DetailsState::default()));
                vec![BrowserEvent::NavigatedToDetails]
            },
            BrowserAction::GoBack if len > 1 => {
                self.navigation.pop();
                vec![BrowserEvent::NavigationPopped]
            },
            _ => current_state.update_with(action)
        }
    }
}
use super::SearchResults;
use crate::app::dispatch::{ActionDispatcher, Worker};
use crate::app::models::*;
use crate::app::state::{AppModel, BrowserAction, ScreenName};
use std::ops::Deref;
use std::rc::Rc;

pub struct SearchFactory {
    app_model: Rc<AppModel>,
    dispatcher: Box<dyn ActionDispatcher>,
    worker: Worker,
}

impl SearchFactory {
    pub fn new(
        app_model: Rc<AppModel>,
        dispatcher: Box<dyn ActionDispatcher>,
        worker: Worker,
    ) -> Self {
        Self {
            app_model,
            dispatcher,
            worker,
        }
    }

    pub fn make_search_results(&self) -> SearchResults {
        let model =
            SearchResultsModel::new(Rc::clone(&self.app_model), self.dispatcher.box_clone());
        SearchResults::new(model, self.worker.clone())
    }
}

pub struct SearchResultsModel {
    app_model: Rc<AppModel>,
    dispatcher: Box<dyn ActionDispatcher>,
}

impl SearchResultsModel {
    fn new(app_model: Rc<AppModel>, dispatcher: Box<dyn ActionDispatcher>) -> Self {
        Self {
            app_model,
            dispatcher,
        }
    }

    pub fn get_query(&self) -> Option<impl Deref<Target = String> + '_> {
        self.app_model
            .map_state_opt(|s| Some(&s.browser_state.search_state()?.query))
    }

    pub fn fetch_results(&self) {
        let api = self.app_model.get_spotify();
        if let Some(query) = self.get_query() {
            let query = query.to_owned();
            self.dispatcher.dispatch_async(Box::pin(async move {
                let albums = api.search_albums(&query[..], 0, 5).await?;
                Some(BrowserAction::SetSearchResults(albums).into())
            }))
        }
    }

    pub fn get_current_results(&self) -> Option<impl Deref<Target = Vec<AlbumDescription>> + '_> {
        self.app_model
            .map_state_opt(|s| Some(&s.browser_state.search_state()?.album_results))
    }

    pub fn open_album(&self, uri: &str) {
        if let Some(id) = uri.split(':').last() {
            self.dispatcher.dispatch(
                BrowserAction::NavigationPush(ScreenName::Details(id.to_string())).into(),
            );
        }
    }
}

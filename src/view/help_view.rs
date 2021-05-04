use crate::prelude::*;

/// HelpView is a View displaying a help dialog with a list of key shortcuts and descriptions
pub struct HelpView {
    view: OnEventView<Dialog>,
    // "section description" followed by a vector of ("key", "key description") pairs
    keys: Vec<(&'static str, Vec<(String, &'static str)>)>,
}

impl HelpView {
    pub fn new() -> Self {
        HelpView {
            view: HelpView::construct_help_dialog_event_view(Dialog::new().title("Help Dialog")),
            keys: vec![],
        }
    }

    fn construct_key_view(key: (String, String), max_key_width: usize) -> impl View {
        let key_string = StyledString::styled(
            key.0,
            ColorStyle::new(
                PaletteColor::TitlePrimary,
                get_config_theme().code_block_bg.color,
            ),
        );
        let desc_string = StyledString::plain(key.1);
        LinearLayout::horizontal()
            .child(TextView::new(key_string).fixed_width(max_key_width))
            .child(TextView::new(desc_string))
    }

    fn construct_help_dialog_event_view(view: Dialog) -> OnEventView<Dialog> {
        OnEventView::new(view).on_event(get_global_keymap().close_dialog.clone(), |s| {
            s.pop_layer();
        })
    }

    fn construct_keys_view(&self) -> impl View {
        LinearLayout::vertical()
            .with(|s| {
                self.keys.iter().for_each(|(desc, keys)| {
                    s.add_child(TextView::new(StyledString::styled(
                        desc.to_string(),
                        ColorStyle::from(PaletteColor::TitlePrimary),
                    )));
                    s.add_child({
                        let max_key_len = match keys.iter().max_by_key(|key| key.0.len()) {
                            None => 0,
                            Some(key) => key.0.len(),
                        };

                        PaddedView::lrtb(
                            0,
                            0,
                            0,
                            1,
                            LinearLayout::vertical()
                                .with(|s| {
                                    keys.iter().for_each(|key| {
                                        s.add_child(HelpView::construct_key_view(
                                            (key.0.to_string(), key.1.to_string()),
                                            max_key_len + 1,
                                        ));
                                    });
                                })
                                .fixed_width(64),
                        )
                    });
                });
            })
            .scrollable()
            .max_height(32)
    }

    pub fn keys(mut self, mut keys: Vec<(&'static str, Vec<(String, &'static str)>)>) -> Self {
        self.keys.append(&mut keys);
        let key_view = self.construct_keys_view();
        self.view.get_inner_mut().set_content(key_view);
        self
    }
}

impl ViewWrapper for HelpView {
    wrap_impl!(self.view: OnEventView<Dialog>);
}

#[macro_export]
macro_rules! other_key_shortcuts {
    ($(($k:expr,$d:expr)),*) => {
        (
            "Others",
            vec![
                $(
                    ($k, $d),
                )*
                (get_global_keymap().open_help_dialog.to_string(), "Open the help dialog"),
                (get_global_keymap().quit.to_string(), "Quit the application"),
                (get_global_keymap().close_dialog.to_string(), "Close a dialog"),
            ],
        )
    };
}

#[macro_export]
macro_rules! view_navigation_key_shortcuts {
    ($(($k:expr,$d:expr)),*) => {
        (
            "View Navigation",
            vec![
                $(
                    ($k, $d),
                )*
                    (get_global_keymap().goto_previous_view.to_string(), "Go to the previous view"),
                    (get_global_keymap().goto_front_page_view.to_string(), "Go to front page view"),
                    (get_global_keymap().goto_search_view.to_string(), "Go to search view"),
                    (get_global_keymap().goto_all_stories_view.to_string(), "Go to all stories view"),
                    (get_global_keymap().goto_ask_hn_view.to_string(), "Go to ask HN view"),
                    (get_global_keymap().goto_show_hn_view.to_string(), "Go to show HN view"),
                    (get_global_keymap().goto_jobs_view.to_string(), "Go to jobs view"),
            ],

        )
    };
}

pub trait HasHelpView {
    fn construct_help_view() -> HelpView {
        HelpView::new().keys(vec![
            view_navigation_key_shortcuts!(),
            other_key_shortcuts!(),
        ])
    }
}

/// An empty struct used to construct the default HelpView
pub struct DefaultHelpView {}

impl HasHelpView for DefaultHelpView {}

impl HasHelpView for StoryView {
    fn construct_help_view() -> HelpView {
        let story_view_keymap = get_story_view_keymap();

        HelpView::new().keys(vec![
            (
                "Navigation",
                vec![
                    (
                        story_view_keymap.next_story.to_string(),
                        "Focus the next story",
                    ),
                    (
                        story_view_keymap.prev_story.to_string(),
                        "Focus the previous story",
                    ),
                    (
                        format!("`{{story_id}} {}`", story_view_keymap.goto_story),
                        "Focus the {story_id}-th story",
                    ),
                ],
            ),
            (
                "Paging/Filtering",
                vec![
                    (
                        story_view_keymap.next_page.to_string(),
                        "Go to the next page",
                    ),
                    (
                        story_view_keymap.prev_page.to_string(),
                        "Go the previous page",
                    ),
                    (
                        story_view_keymap.toggle_sort_by.to_string(),
                        "Toggle sort by date/popularity",
                    ),
                    (
                        story_view_keymap.filter_past_day.to_string(),
                        "Filter stories past day",
                    ),
                    (
                        story_view_keymap.filter_past_week.to_string(),
                        "Filter stories past week",
                    ),
                    (
                        story_view_keymap.filter_past_month.to_string(),
                        "Filter stories past month",
                    ),
                    (
                        story_view_keymap.filter_past_year.to_string(),
                        "Filter stories past year",
                    ),
                ],
            ),
            (
                "Open external links",
                vec![
                    (
                        story_view_keymap.open_article_in_browser.to_string(),
                        "Open in browser the article associated with the focused story",
                    ),
                    (
                        story_view_keymap.open_article_in_article_view.to_string(),
                        "Open in article view the article associated with the focused story",
                    ),
                    (
                        story_view_keymap.open_story_in_browser.to_string(),
                        "Open in browser the focused story",
                    ),
                ],
            ),
            view_navigation_key_shortcuts!((
                story_view_keymap.goto_story_comment_view.to_string(),
                "Go to the comment view associated with the focused story"
            )),
            other_key_shortcuts!(),
        ])
    }
}

impl HasHelpView for CommentView {
    fn construct_help_view() -> HelpView {
        let comment_view_keymap = get_comment_view_keymap();
        let story_view_keymap = get_story_view_keymap();

        HelpView::new().keys(vec![
            (
                "Navigation",
                vec![
                    (
                        comment_view_keymap.next_comment.to_string(),
                        "Focus the next comment",
                    ),
                    (
                        comment_view_keymap.prev_comment.to_string(),
                        "Focus the previous comment",
                    ),
                    (
                        comment_view_keymap.next_top_level_comment.to_string(),
                        "Focus the next top level comment",
                    ),
                    (
                        comment_view_keymap.prev_top_level_comment.to_string(),
                        "Focus the previous top level comment",
                    ),
                    (
                        comment_view_keymap.next_leq_level_comment.to_string(),
                        "Focus the next comment at smaller or equal level",
                    ),
                    (
                        comment_view_keymap.prev_leq_level_comment.to_string(),
                        "Focus the previous comment at smaller or equal level",
                    ),
                ],
            ),
            (
                "Scrolling",
                vec![
                    (comment_view_keymap.up.to_string(), "Scroll up"),
                    (comment_view_keymap.down.to_string(), "Scroll down"),
                    (
                        comment_view_keymap.page_up.to_string(),
                        "Scroll up half a page",
                    ),
                    (
                        comment_view_keymap.page_down.to_string(),
                        "Scroll down half a page",
                    ),
                ],
            ),
            (
                "Open external links",
                vec![
                    (
                        story_view_keymap.open_article_in_browser.to_string(),
                        "Open in browser the article associated with the discussed story",
                    ),
                    (
                        story_view_keymap.open_article_in_article_view.to_string(),
                        "Open in article view the article associated with the discussed story",
                    ),
                    (
                        story_view_keymap.open_story_in_browser.to_string(),
                        "Open in browser the discussed story",
                    ),
                    (
                        comment_view_keymap.open_comment_in_browser.to_string(),
                        "Open in browser the focused comment",
                    ),
                    (
                        format!("`{{link_id}} {}`", comment_view_keymap.open_link_in_browser),
                        "Open in browser the {link_id}-th link in the focused comment",
                    ),
                    (
                        format!(
                            "`{{link_id}} {}`",
                            comment_view_keymap.open_link_in_article_view
                        ),
                        "Open in article view the {link_id}-th link in the focused comment",
                    ),
                ],
            ),
            view_navigation_key_shortcuts!(),
            other_key_shortcuts!((
                comment_view_keymap.reload_comment_view.to_string(),
                "Reload the comment view"
            )),
        ])
    }
}

impl HasHelpView for SearchView {
    fn construct_help_view() -> HelpView {
        let search_view_keymap = get_search_view_keymap();
        let story_view_keymap = get_story_view_keymap();

        HelpView::new().keys(vec![
            (
                "Switch Mode",
                vec![
                    (
                        search_view_keymap.to_navigation_mode.to_string(),
                        "Switch to navigation mode",
                    ),
                    (
                        search_view_keymap.to_search_mode.to_string(),
                        "Switch to search mode",
                    ),
                ],
            ),
            (
                "Navigation Mode - Navigation",
                vec![
                    (
                        story_view_keymap.next_story.to_string(),
                        "Focus the next story",
                    ),
                    (
                        story_view_keymap.prev_story.to_string(),
                        "Focus the previous story",
                    ),
                    (
                        format!("`{{story_id}} {}`", story_view_keymap.goto_story),
                        "Focus the {story_id}-th story",
                    ),
                ],
            ),
            (
                "Navigation Mode - Paging/Filtering",
                vec![
                    (
                        story_view_keymap.next_page.to_string(),
                        "Go to the next page",
                    ),
                    (
                        story_view_keymap.prev_page.to_string(),
                        "Go the previous page",
                    ),
                    (
                        story_view_keymap.toggle_sort_by.to_string(),
                        "Toggle sort by date/popularity",
                    ),
                ],
            ),
            (
                "Navigation Mode - Open external links",
                vec![
                    (
                        story_view_keymap.open_article_in_browser.to_string(),
                        "Open in browser the link associated with the focused story",
                    ),
                    (
                        story_view_keymap.open_article_in_article_view.to_string(),
                        "Open in article view the link associated with the focused story",
                    ),
                    (
                        story_view_keymap.open_story_in_browser.to_string(),
                        "Open in browser the focused story",
                    ),
                ],
            ),
            view_navigation_key_shortcuts!((
                story_view_keymap.goto_story_comment_view.to_string(),
                "Go to the comment view associated with the focused story"
            )),
            other_key_shortcuts!(),
        ])
    }
}

impl HasHelpView for ArticleView {
    fn construct_help_view() -> HelpView {
        let article_view_keymap = get_article_view_keymap().clone();
        HelpView::new().keys(vec![
            (
                "Navigation",
                vec![
                    (article_view_keymap.up.to_string(), "Scroll up"),
                    (article_view_keymap.down.to_string(), "Scroll down"),
                    (
                        article_view_keymap.page_up.to_string(),
                        "Scroll up half a page",
                    ),
                    (
                        article_view_keymap.page_down.to_string(),
                        "Scroll down half a page",
                    ),
                    (article_view_keymap.top.to_string(), "Scroll to top"),
                    (article_view_keymap.bottom.to_string(), "Scroll to bottom"),
                ],
            ),
            (
                "Open external links",
                vec![
                    (
                        article_view_keymap.open_article_in_browser.to_string(),
                        "Open article in browser",
                    ),
                    (
                        format!(
                            "`{{link_id}} {}`",
                            article_view_keymap.open_link_in_browser.to_string()
                        ),
                        "Open in browser {link_id}-th link",
                    ),
                    (
                        format!(
                            "`{{link_id}} {}`",
                            article_view_keymap.open_link_in_article_view.to_string()
                        ),
                        "Open in article view {link_id}-th link",
                    ),
                ],
            ),
            (
                "Link dialog",
                vec![
                    (
                        article_view_keymap.open_link_dialog.to_string(),
                        "Open link dialog",
                    ),
                    (
                        article_view_keymap.link_dialog_focus_next.to_string(),
                        "Focus next link",
                    ),
                    (
                        article_view_keymap.link_dialog_focus_prev.to_string(),
                        "Focus previous link",
                    ),
                    (
                        article_view_keymap.open_link_in_browser.to_string(),
                        "Open in browser the focused link",
                    ),
                    (
                        article_view_keymap.open_link_in_article_view.to_string(),
                        "Open in article view the focused link",
                    ),
                ],
            ),
            view_navigation_key_shortcuts!(),
            other_key_shortcuts!((
                article_view_keymap.toggle_raw_markdown_mode.to_string(),
                "Toggle raw markdown mode"
            )),
        ])
    }
}

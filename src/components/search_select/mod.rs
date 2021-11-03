use yew::prelude::*;

mod state;
pub use state::SelectState;
mod selection;
pub use selection::Selection;
mod wrappers;
pub use wrappers::{SelectDisplay, SelectFilter};

/// Bulma-based selection box
/// TODO: document
pub struct Select<T: 'static> {
    focused: bool,
    selection_index: usize,
    search_text: String,

    _ty: std::marker::PhantomData<T>,
}

#[derive(Properties)]
pub struct SelectProps<T> {
    /// Omit selected items from the dropdown list (if false, selected will be
    /// highlighted).
    #[prop_or_default]
    pub omit_selected: bool,

    /// Control whether to show the selected items as tags in the input field
    /// (if in multiple mode). Useful to turn off if you want to control display
    /// of the selected items outside of the field.
    ///
    /// This does nothing in single selection mode.
    #[prop_or(true)]
    pub display_selected: bool,

    pub state: SelectState<T>,
    pub display: SelectDisplay<T>,

    #[prop_or_default]
    pub onselected: Option<Callback<usize>>,
    #[prop_or_default]
    pub onremoved: Option<Callback<usize>>,

    #[prop_or_else(|| String::from("Type to search"))]
    pub placeholder: String,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
}

// This SHOULD be the auto impl, but for some reason that thinks that T needs to
// be Clone
impl<T> Clone for SelectProps<T> {
    fn clone(&self) -> Self {
        Self {
            omit_selected: self.omit_selected,
            display_selected: self.display_selected,

            state: self.state.clone(),
            display: self.display.clone(),

            onselected: self.onselected.clone(),
            onremoved: self.onremoved.clone(),

            placeholder: self.placeholder.clone(),
            readonly: self.readonly,
            disabled: self.disabled,
            loading: self.loading,
        }
    }
}

impl<T> PartialEq for SelectProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.readonly == other.readonly && self.disabled == other.disabled && self.loading == other.loading &&
            self.state == other.state
            // && Arc::ptr_eq(&self.filter, &other.filter) // TODO: don't ignore filter changes?
            && self.omit_selected == other.omit_selected
            && self.placeholder == other.placeholder
            && self.onselected == other.onselected
            && self.onremoved == other.onremoved
    }
}

pub enum Msg {
    Noop,

    Input(String),
    ClearSearch,
    Filtered,

    Selected(usize),
    Removed(usize),
    Hover(usize),

    Focus,
    Blur,
    KeyPress(KeyboardEvent),
}

impl<T: 'static> Component for Select<T> {
    type Properties = SelectProps<T>;
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focused: false,
            selection_index: 0,
            search_text: String::new(),

            _ty: Default::default(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().disabled {
            self.focused = false;
            self.selection_index = 0;
            self.search_text.clear();
        }
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Noop => false,

            Msg::Filtered => true,

            Msg::Input(input) => {
                let props = ctx.props();
                if props.disabled || props.readonly {
                    return false;
                }

                self.focused = true;
                self.search_text = input.clone();

                let state = props.state.clone();
                ctx.link().send_future(async move {
                    if input.is_empty() {
                        state.unfilter().await;
                    } else {
                        state.filter(&input).await;
                    }
                    Msg::Filtered
                });
                true
            }

            Msg::ClearSearch => {
                let options = ctx.props().state.clone();
                ctx.link().send_future(async move {
                    options.unfilter().await;
                    Msg::Filtered
                });
                self.search_text.clear();
                true
            }

            Msg::Selected(idx) => {
                if let Some(ref onselected) = ctx.props().onselected {
                    onselected.emit(idx);
                }
                ctx.link()
                    .send_message_batch(vec![Msg::ClearSearch, Msg::Blur]);
                false
            }

            Msg::Removed(idx) => {
                if let Some(ref onremoved) = ctx.props().onremoved {
                    onremoved.emit(idx);
                }
                false
            }

            Msg::Hover(idx) => {
                self.selection_index = idx;
                true
            }

            Msg::Focus => {
                if ctx.props().disabled || ctx.props().readonly {
                    return false;
                }
                self.focused = true;
                true
            }

            Msg::Blur => {
                self.focused = false;
                self.selection_index = 0;
                self.search_text.clear();
                true
            }

            Msg::KeyPress(event) => {
                let props = ctx.props();
                if props.disabled || props.readonly {
                    return false;
                }
                match event.code().as_ref() {
                    "Enter" => {
                        if let Some((index, _)) = props.state.get_filtered(self.selection_index) {
                            ctx.link().send_message(Msg::Selected(index));
                        }
                        false
                    }

                    "Escape" => {
                        self.focused = false;
                        self.selection_index = 0;
                        self.search_text.clear();
                        true
                    }

                    "ArrowUp" => {
                        self.focused = true;
                        let event: &Event = &event;
                        event.prevent_default();
                        self.selection_index = self.selection_index.saturating_sub(1);
                        true
                    }

                    "ArrowDown" => {
                        self.focused = true;
                        let event: &Event = &event;
                        event.prevent_default();
                        self.selection_index += 1;
                        true
                    }

                    _ => false,
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        let options = if props.omit_selected {
            props
                .state
                .filtered_items()
                .into_iter()
                .filter(|(_, selected, _)| !selected)
                .collect::<Vec<_>>()
        } else {
            props.state.filtered_items()
        };

        let options = if options.is_empty() {
            html! {
                <div class="has-text-centered">
                    <p>
                        <span class="icon">
                            <i class="fas fa-inbox" />
                        </span>
                    </p>
                    <p>{"No Data"}</p>
                </div>
            }
        } else {
            options
                .into_iter()
                .enumerate()
                .map(|(i, (idx, selected, item))| {
                    html! {
                        <a
                            class={classes!(
                                "dropdown-item",
                                if self.selection_index == i {"is-active"}
                                else if selected {"has-background-primary-light"}
                                else {""}
                            )}
                        >
                            <p
                                onmouseenter={link.callback(move |_| Msg::Hover(i))}
                                onmousedown={link.callback(move |event: MouseEvent| {
                                    let event: &Event = &event;
                                    event.prevent_default();
                                    Msg::Selected(idx)
                                })}
                            >
                                { props.display.call(item) }
                            </p>
                        </a>
                    }
                })
                .collect::<Html>()
        };

        html! {
            <div class={classes!("dropdown", if self.focused {"is-active"} else {""})}>
                <div class="dropdown-trigger">
                {
                    if props.state.is_multiple() {
                        self.view_multiple(ctx)
                    } else {
                        self.view_single(ctx)
                    }
                }
                </div>
                <div class="dropdown-menu">
                    <div class="dropdown-content">
                        { options }
                    </div>
                </div>
            </div>
        }
    }
}

impl<T> Select<T> {
    fn view_single(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        if self.focused {
            html! {
                <div class="control has-icons-right">
                    <input
                        class={classes!("input", if props.loading {"is-loading"} else {""})}
                        type="text"
                        value={self.search_text.clone()}
                        placeholder={props.state.selected_items().first().map(|(_, x)| props.display.call(x)).unwrap_or_else(|| props.placeholder.clone())}
                        oninput={link.callback(|evt: InputEvent| Msg::Input(evt.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                        onfocus={link.callback(|_| Msg::Focus)}
                        onblur={link.callback(|_| Msg::Blur)}
                        onkeydown={link.callback(Msg::KeyPress)}
                        disabled={props.disabled}
                        readonly={props.readonly}
                    />
                    <span class="icon is-small is-right">
                    {
                        if self.search_text.is_empty() {
                            html! { <i class="fas fa-search" /> }
                        } else {
                            html! {<button class="delete" onclick={link.callback(|_| Msg::ClearSearch)} /> }
                        }
                    }
                    </span>
                </div>
            }
        } else {
            html! {
                <div class="control has-icons-right">
                    <input
                        class={classes!("input", if props.loading {"is-loading"} else {""})}
                        type="text"
                        value={props.state.selected_items().first().map(|(_, x)| props.display.call(x)).unwrap_or_default()}
                        oninput={link.callback(|evt: InputEvent| {
                            // Don't allow input when not focused
                            evt.prevent_default(); // TODO: this may not work
                            Msg::Focus
                        })}
                        onfocus={link.callback(|_| Msg::Focus)}
                        onblur={link.callback(|_| Msg::Blur)}
                        onclick={link.callback(|_| Msg::Focus)}
                        onkeydown={link.callback(Msg::KeyPress)}
                        disabled={props.disabled}
                        readonly={props.readonly}
                    />
                    <span class="icon is-small is-right">
                        <i class="fas fa-angle-down" />
                    </span>
                </div>
            }
        }
    }

    fn view_multiple(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        html! {
            <div class={classes!("input", "byewlma-search-select-multiple-input-wrapper", if self.focused {"is-active"} else {""})}>
                {
                    if props.display_selected {
                        props.state.selected_items().into_iter().map(|(i, item)| html! {
                            <span class="tag">
                                { props.display.call(item) }
                                <div class="delete is-small" onclick={link.callback(move |_| Msg::Removed(i))} />
                            </span>
                        }).collect::<Html>()
                    } else {
                        html! {}
                    }
                }
                <input
                    class={classes!("input", if props.loading {"is-loading"} else {""})}
                    type="text"
                    placeholder="Type to search"
                    value={self.search_text.clone()}
                    oninput={link.callback(|evt: InputEvent| Msg::Input(evt.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                    onfocus={link.callback(|_| Msg::Focus)}
                    onblur={link.callback(|_| Msg::Blur)}
                    onkeydown={link.callback(Msg::KeyPress)}
                    disabled={props.disabled}
                    readonly={props.readonly}
                />
            </div>
        }
    }
}

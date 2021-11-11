use yew::virtual_dom::{AttrValue, VChild};

use crate::{
    components::icon::{Icon, IconAnimation, IconKind},
    innerlude::*,
    Size,
};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Css Classes to pass along to the root table element
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub is_loading: bool,

    #[prop_or_default]
    pub is_bordered: bool,

    #[prop_or_default]
    pub is_striped: bool,

    #[prop_or_default]
    pub is_narrow: bool,

    #[prop_or_default]
    pub is_fullwidth: bool,

    #[prop_or_default]
    pub is_hoverable: bool,

    /// Wrap the table in a "table-container" to allow x-overflow to scroll
    #[prop_or_default]
    pub is_scrollable: bool,

    #[prop_or_default]
    pub header: Option<VChild<Header>>,

    #[prop_or_default]
    pub children: Children, // ChildrenWithProps<Row>,
}

// pub type TableHeader = Rc<Vec<()>>;
// pub type TableData<const COLS: usize> = Rc<Vec<[(); COLS]>>;

/// Bulma [Table](https://bulma.io/documentation/elements/table/) Component
#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let id = props.id.clone();
    let style = props.style.clone();

    let (table_class, wrapper_class) = if props.is_scrollable {
        let mut wrapper = props.class.clone();
        unsafe {
            wrapper.unchecked_push("table-container");
        }
        (Classes::new(), Some(wrapper))
    } else {
        (props.class.clone(), None)
    };

    let mut table_class = table_class;

    unsafe {
        table_class.unchecked_push("table");
    }
    if props.is_bordered {
        unsafe {
            table_class.unchecked_push("is-bordered");
        }
    }
    if props.is_striped {
        unsafe {
            table_class.unchecked_push("is-striped");
        }
    }
    if props.is_narrow {
        unsafe {
            table_class.unchecked_push("is-narrow");
        }
    }
    if props.is_hoverable {
        unsafe {
            table_class.unchecked_push("is-hoverable");
        }
    }
    if props.is_fullwidth {
        unsafe {
            table_class.unchecked_push("is-fullwidth");
        }
    }

    let mut columns = 0;
    let header = props
        .header
        .clone()
        .map(|header| {
            columns = header.props.children.len();
            Html::from(header)
        })
        .unwrap_or_default();

    let body = if props.is_loading {
        html! {
            <tr>
                <td colspan={columns.to_string()} class="has-text-centered has-background-light">
                    <Icon
                        kind={IconKind::Spinner}
                        animate={IconAnimation::Spin}
                        size={Size::Large}
                        text={String::from("Loading...")}
                    />
                </td>
            </tr>
        }
    } else if props.children.is_empty() {
        html! {
            <tr>
                <td colspan={columns.to_string()} class="has-text-centered has-background-light">
                    <Icon
                        kind={IconKind::Inbox}
                        size={Size::Large}
                        text={String::from("No Data")}
                    />
                </td>
            </tr>
        }
    } else {
        html! { <> { for props.children.iter() } </> }
    };

    let table = html! {
        <table {id} class={table_class} {style}>
            { header }
            <tbody>{body}</tbody>
        </table>
    };

    if let Some(class) = wrapper_class {
        html! {
            <div {class}>
                {table}
            </div>
        }
    } else {
        table
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children, // ChildrenWithProps<HeaderCell>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <thead>
            <tr>
            { for props.children.iter() }
            </tr>
        </thead>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct HeaderCellProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(HeaderCell)]
pub fn header_cell(props: &HeaderCellProps) -> Html {
    html! {
        <th>
        { for props.children.iter() }
        </th>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct RowProps {
    #[prop_or_default]
    pub is_expanded: bool,

    #[prop_or_default]
    pub children: Children, // ChildrenWithProps<Cell>,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
    html! {
        <tr>
            { for props.children.iter() }
        </tr>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct CellProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Css Classes to pass along to the root table element
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub col_span: Option<usize>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let id = props.id.clone();
    let class = props.class.clone();
    let style = props.style.clone();
    let colspan = props
        .col_span
        .as_ref()
        .map(|x| AttrValue::Owned(x.to_string()));

    html! {
        <td {id} {colspan} {class} {style}>
        { for props.children.iter() }
        </td>
    }
}

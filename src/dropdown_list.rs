use crate::egui::*;

/// Returns the iterator element corresponding to the newly-selected item.
pub fn dropdown_list<T, I, F, S>(
    drop_from_widget: Response,
    ui: &mut Ui,
    id_source: impl std::hash::Hash,
    items: I,
    get_text: F,
) -> Option<T>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> S,
    S: Into<WidgetText>,
{
    let popup_id = ui.make_persistent_id(id_source);

    if drop_from_widget.clicked() {
        Popup::open_id(ui.ctx(), popup_id);
    }

    let mut return_val = None;
    egui::Popup::from_response(&drop_from_widget)
        .layout(Layout::top_down_justified(Align::LEFT))
        .open_memory(None)
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        .id(popup_id)
        .width(drop_from_widget.rect.width())
        .show(|ui| {
            ui.set_min_width(ui.available_width());
            let select_first = ui.input(|i| i.key_pressed(Key::Enter));
            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                let mut first = true;
                for item in items {
                    let text = get_text(&item);

                    // TODO: implement arrow key browsing, have the one that'd be
                    // selected by enter highlighted
                    if ui.selectable_label(false, text.into()).clicked() || (select_first && first)
                    {
                        return_val = Some(item);
                        ui.close();
                    }
                    first = false;
                }
            });
        });

    return_val
}

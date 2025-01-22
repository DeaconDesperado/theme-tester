use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ReadApplicationState]);
        subscribe(&[EventType::ModeUpdate]);
    }
    fn update(&mut self, _event: Event) -> bool {
        // this is so that we render ourselves when we get a ModeUpdate (theme change)
        let should_render = true;
        should_render
    }
    fn render(&mut self, _rows: usize, _cols: usize) {
        print_text_with_coordinates(Text::new("# Ribbon"), 0, 0, None, None);
        print_ribbon_with_coordinates(Text::new("no em"), 0, 1, None, None);
        print_ribbon_with_coordinates(Text::new("emp 0").color_range(0, ..), 9, 1, None, None);
        print_ribbon_with_coordinates(Text::new("emp 1").color_range(1, ..), 18, 1, None, None);
        print_ribbon_with_coordinates(Text::new("emp 2").color_range(2, ..), 27, 1, None, None);
        print_ribbon_with_coordinates(Text::new("emp 3").color_range(3, ..), 36, 1, None, None);
        print_ribbon_with_coordinates(Text::new("no em").selected(), 0, 2, None, None);
        print_ribbon_with_coordinates(
            Text::new("emp 0").selected().color_range(0, ..),
            9,
            2,
            None,
            None,
        );
        print_ribbon_with_coordinates(
            Text::new("emp 1").selected().color_range(1, ..),
            18,
            2,
            None,
            None,
        );
        print_ribbon_with_coordinates(
            Text::new("emp 2").selected().color_range(2, ..),
            27,
            2,
            None,
            None,
        );
        print_ribbon_with_coordinates(
            Text::new("emp 3").selected().color_range(3, ..),
            36,
            2,
            None,
            None,
        );

        let table = Table::new()
            .add_styled_row(vec![
                Text::new("title (none)"),
                Text::new("title (emp0)").color_range(0, ..),
                Text::new("title (emp1)").color_range(1, ..),
                Text::new("title (emp2)").color_range(2, ..),
                Text::new("title (emp3)").color_range(3, ..),
                Text::new("title (none, opaque)").opaque(),
                Text::new("title (emp0, opaque)")
                    .color_range(0, ..)
                    .opaque(),
                Text::new("title (emp1, opaque)")
                    .color_range(1, ..)
                    .opaque(),
                Text::new("title (emp2, opaque)")
                    .color_range(2, ..)
                    .opaque(),
                Text::new("title (emp3, opaque)")
                    .color_range(3, ..)
                    .opaque(),
            ])
            .add_styled_row(vec![
                Text::new("unselected, no emphasis"),
                Text::new("unselected, emphasis 0").color_range(0, ..),
                Text::new("unselected, emphasis 1").color_range(1, ..),
                Text::new("unselected, emphasis 2").color_range(2, ..),
                Text::new("unselected, emphasis 3").color_range(3, ..),
                Text::new("unselected, no emphasis, opaque"),
                Text::new("unselected, emphasis 0, opaque")
                    .color_range(0, ..)
                    .opaque(),
                Text::new("unselected, emphasis 1, opaque")
                    .color_range(1, ..)
                    .opaque(),
                Text::new("unselected, emphasis 2, opaque")
                    .color_range(2, ..)
                    .opaque(),
                Text::new("unselected, emphasis 3, opaque")
                    .color_range(3, ..)
                    .opaque(),
            ])
            .add_styled_row(vec![
                Text::new("selected, no emphasis").selected(),
                Text::new("selected, emphasis 0")
                    .color_range(0, ..)
                    .selected(),
                Text::new("selected, emphasis 1")
                    .color_range(1, ..)
                    .selected(),
                Text::new("selected, emphasis 2")
                    .color_range(2, ..)
                    .selected(),
                Text::new("selected, emphasis 3")
                    .color_range(3, ..)
                    .selected(),
                Text::new("selected, no emphasis, opaque")
                    .selected()
                    .opaque(),
                Text::new("selected, emphasis 0, opaque")
                    .color_range(0, ..)
                    .selected()
                    .opaque(),
                Text::new("selected, emphasis 1, opaque")
                    .color_range(1, ..)
                    .selected()
                    .opaque(),
                Text::new("selected, emphasis 2, opaque")
                    .color_range(2, ..)
                    .selected()
                    .opaque(),
                Text::new("selected, emphasis 3, opaque")
                    .color_range(3, ..)
                    .selected()
                    .opaque(),
            ])
            .add_styled_row(vec![
                Text::new("no emphasis"),
                Text::new("emphasis 0").color_range(0, ..),
                Text::new("emphasis 1").color_range(1, ..),
                Text::new("emphasis 2").color_range(2, ..),
                Text::new("emphasis 3").color_range(3, ..),
            ]);
        print_text_with_coordinates(Text::new("# Table"), 0, 4, None, None);
        print_table_with_coordinates(table, 0, 5, None, None);

        print_text_with_coordinates(Text::new("# Nested List"), 0, 10, None, None);
        print_nested_list_with_coordinates(
            vec![
                NestedListItem::new("item with no emphasis"),
                NestedListItem::new("item with emp 0")
                    .indent(1)
                    .color_range(0, ..),
                NestedListItem::new("item with emp 1")
                    .indent(1)
                    .color_range(1, ..),
                NestedListItem::new("item with emp 2")
                    .indent(1)
                    .color_range(2, ..),
                NestedListItem::new("item with emp 3")
                    .indent(1)
                    .color_range(3, ..),
                NestedListItem::new("selected no emp emp 0 emp 1 emp 2 emp 3")
                    .selected()
                    .color_range(0, 16..=20)
                    .color_range(1, 22..=26)
                    .color_range(2, 28..=32)
                    .color_range(3, 33..=38),
                NestedListItem::new("item with no emphasis, opaque").opaque(),
                NestedListItem::new("item with emp 0, opaque")
                    .indent(1)
                    .color_range(0, ..)
                    .opaque(),
                NestedListItem::new("item with emp 1, opaque")
                    .indent(1)
                    .color_range(1, ..)
                    .opaque(),
                NestedListItem::new("item with emp 2, opaque")
                    .indent(1)
                    .color_range(2, ..)
                    .opaque(),
                NestedListItem::new("item with emp 3, opaque")
                    .indent(1)
                    .color_range(3, ..)
                    .opaque(),
                NestedListItem::new("selected no emp emp 0 emp 1 emp 2 emp 3, opaque")
                    .selected()
                    .color_range(0, 16..=20)
                    .color_range(1, 22..=26)
                    .color_range(2, 28..=32)
                    .color_range(3, 33..=38)
                    .opaque(),
            ],
            0,
            11,
            None,
            None,
        );

        print_text_with_coordinates(Text::new("# Text"), 0, 18, None, None);
        print_text_with_coordinates(
            Text::new("not selected no emp emp 0 emp 1 emp 2 emp 3")
                .color_range(0, 19..=23)
                .color_range(1, 25..=29)
                .color_range(2, 31..=35)
                .color_range(3, 36..=42),
            0,
            19,
            None,
            None,
        );
        print_text_with_coordinates(
            Text::new("selected no emp emp 0 emp 1 emp 2 emp 3")
                .selected()
                .color_range(0, 16..=20)
                .color_range(1, 22..=26)
                .color_range(2, 28..=32)
                .color_range(3, 33..=38),
            0,
            20,
            None,
            None,
        );

        print_text_with_coordinates(Text::new("# Text").opaque(), 0, 18, None, None);
        print_text_with_coordinates(
            Text::new("not selected no emp emp 0 emp 1 emp 2 emp 3, opaque")
                .color_range(0, 19..=23)
                .color_range(1, 25..=29)
                .color_range(2, 31..=35)
                .color_range(3, 36..=42)
                .opaque(),
            0,
            19,
            None,
            None,
        );
        print_text_with_coordinates(
            Text::new("selected no emp emp 0 emp 1 emp 2 emp 3, opaque")
                .selected()
                .color_range(0, 16..=20)
                .color_range(1, 22..=26)
                .color_range(2, 28..=32)
                .color_range(3, 33..=38)
                .opaque(),
            0,
            20,
            None,
            None,
        );
    }
}

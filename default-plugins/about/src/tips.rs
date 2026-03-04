use zellij_tile::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::active_component::{ActiveComponent, ClickAction};
use crate::pages::{BulletinList, ComponentLine, Page, TextOrCustomRender};

pub const MAX_TIP_INDEX: usize = 11;

impl Page {
    pub fn new_tip_screen(
        link_executable: Rc<RefCell<String>>,
        base_mode: Rc<RefCell<InputMode>>,
        tip_index: usize,
    ) -> Self {
        if tip_index == 0 {
            Page::tip_1(link_executable)
        } else if tip_index == 1 {
            Page::tip_2(link_executable, base_mode)
        } else if tip_index == 2 {
            Page::tip_3(link_executable)
        } else if tip_index == 3 {
            Page::tip_4(link_executable, base_mode)
        } else if tip_index == 4 {
            Page::tip_5(link_executable)
        } else if tip_index == 5 {
            Page::tip_6(link_executable, base_mode)
        } else if tip_index == 6 {
            Page::tip_7(link_executable)
        } else if tip_index == 7 {
            Page::tip_8(link_executable)
        } else if tip_index == 8 {
            Page::tip_9(link_executable)
        } else if tip_index == 9 {
            Page::tip_10(link_executable, base_mode)
        } else if tip_index == 10 {
            Page::tip_11(link_executable)
        } else if tip_index == 11 {
            Page::tip_12(link_executable, base_mode)
        } else {
            Page::tip_1(link_executable)
        }
    }
    pub fn tip_1(link_executable: Rc<RefCell<String>>) -> Self {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #1").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("查看 Zellij 录屏/教程，学习如何更好地利用"),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("Zellij 的全部功能，了解基础用法、布局、会话等内容！"),
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("访问链接：").color_range(2, ..),
                )),
                ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                    "https://zellij.dev/screencasts",
                )))
                .with_hover(TextOrCustomRender::CustomRender(
                    Box::new(screencasts_link_selected()),
                    Box::new(screencasts_link_selected_len()),
                ))
                .with_left_click_action(ClickAction::new_open_link(
                    format!("https://zellij.dev/screencasts"),
                    link_executable.clone(),
                )),
            ])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_2(link_executable: Rc<RefCell<String>>, base_mode: Rc<RefCell<InputMode>>) -> Self {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #2").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("你可以在 $EDITOR 中打开终端内容，以便搜索").color_range(2, 43..=49),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("其中内容、复制到剪贴板，或保存供稍后使用。"),
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![match *base_mode.borrow() {
                InputMode::Locked => ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("焦点位于终端窗格时：Ctrl g + s + e")
                        .color_range(0, 34..=39)
                        .color_indices(0, vec![43, 47]),
                )),
                _ => ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("焦点位于终端窗格时：Ctrl s + e")
                        .color_range(0, 34..=39)
                        .color_indices(0, vec![43]),
                )),
            }])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_3(link_executable: Rc<RefCell<String>>) -> Self {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #3").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("想把浮动窗格变大吗？"),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("聚焦该窗格后，可按 Alt ] 切换到 ENLARGED 布局。")
                        .color_range(2, 22..=29)
                        .color_range(0, 43..=47),
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    fn tip_4(link_executable: Rc<RefCell<String>>, base_mode: Rc<RefCell<InputMode>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #4").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("你可以将浮动窗格“钉住”，使其始终"),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("可见，即使浮动窗格被隐藏。"),
                ))]),
            ])
            .with_bulletin_list(
                BulletinList::new(Text::new(format!("浮动窗格可被“钉住”：")).color_range(2, ..))
                    .with_items(vec![
                        ActiveComponent::new(TextOrCustomRender::Text(
                            Text::new(format!("鼠标点击其右上角")).color_range(3, 7..=17),
                        )),
                        ActiveComponent::new(TextOrCustomRender::Text(match *base_mode.borrow() {
                            InputMode::Locked => Text::new(format!("使用 Ctrl g + p + i"))
                                .color_range(3, 5..=10)
                                .color_range(3, 14..15)
                                .color_range(3, 18..19),
                            _ => Text::new("使用 Ctrl p + i")
                                .color_range(3, 5..=10)
                                .color_range(3, 14..15),
                        })),
                    ]),
            )
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("一个典型用法是 tail 日志文件，或显示"),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new(format!("实时编译输出，同时在其他窗格工作。")),
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_5(link_executable: Rc<RefCell<String>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #5").color_range(0, ..))
            .with_paragraph(vec![ComponentLine::new(vec![ActiveComponent::new(
                TextOrCustomRender::Text(Text::new("窗格可通过缩放形成堆叠，便于管理。")),
            )])])
            .with_bulletin_list(
                BulletinList::new(Text::new("可这样体验：").color_range(2, ..)).with_items(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("用 Alt f 隐藏该窗格（再次 Alt f 可恢复）")
                            .color_range(3, 20..=24)
                            .color_range(3, 54..=58),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("用 Alt n 打开 4-5 个窗格").color_range(3, 20..=24),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("按 Alt + 直到达到全屏").color_range(3, 6..=10),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("按 Alt - 直到回到初始状态").color_range(3, 6..=10),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("你随时可用 Alt <[]> 回到内置交换布局")
                            .color_range(3, 59..=61)
                            .color_range(3, 64..=65),
                    )),
                ]),
            )
            .with_paragraph(vec![ComponentLine::new(vec![ActiveComponent::new(
                TextOrCustomRender::Text(
                    Text::new("若要禁用此行为，请在 Zellij 配置中加入 stacked_resize false")
                        .color_range(3, 30..=49),
                ),
            )])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("更多详情：").color_range(2, ..),
                )),
                ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                    "https://zellij.dev/tutorials/stacked-resize",
                )))
                .with_hover(TextOrCustomRender::CustomRender(
                    Box::new(stacked_resize_screencast_link_selected),
                    Box::new(stacked_resize_screencast_link_selected_len),
                ))
                .with_left_click_action(ClickAction::new_open_link(
                    "https://zellij.dev/tutorials/stacked-resize".to_owned(),
                    link_executable.clone(),
                )),
            ])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_6(link_executable: Rc<RefCell<String>>, base_mode: Rc<RefCell<InputMode>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #6").color_range(0, ..))
            .with_paragraph(vec![ComponentLine::new(vec![ActiveComponent::new(
                TextOrCustomRender::Text(Text::new("Zellij 快捷键是否与你的其他应用冲突？")),
            )])])
            .with_bulletin_list(
                BulletinList::new(Text::new("试试无冲突快捷键预设：")).with_items(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(match *base_mode.borrow() {
                        InputMode::Locked => Text::new("使用 Ctrl g + o + c 打开 Zellij 配置")
                            .color_range(3, 35..=40)
                            .color_indices(3, vec![44, 48]),
                        _ => Text::new("使用 Ctrl o + c 打开 Zellij 配置")
                            .color_range(3, 35..=40)
                            .color_indices(3, vec![44]),
                    })),
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("按 TAB 进入 Change Mode Behavior").color_range(3, 6..=9),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("用 ENTER 临时启用无冲突方案，或用 Ctrl a 永久启用")
                            .color_range(3, 38..=42)
                            .color_range(3, 64..=69),
                    )),
                ]),
            )
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("更多详情：").color_range(2, ..),
                )),
                ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                    "https://zellij.dev/tutorials/colliding-keybindings",
                )))
                .with_hover(TextOrCustomRender::CustomRender(
                    Box::new(colliding_keybindings_link_selected),
                    Box::new(colliding_keybindings_link_selected_len),
                ))
                .with_left_click_action(ClickAction::new_open_link(
                    "https://zellij.dev/tutorials/colliding-keybindings".to_owned(),
                    link_executable.clone(),
                )),
            ])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_7(link_executable: Rc<RefCell<String>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #7").color_range(0, ..))
            .with_paragraph(vec![ComponentLine::new(vec![ActiveComponent::new(
                TextOrCustomRender::Text(Text::new("想自定义 Zellij 的外观和颜色吗？")),
            )])])
            .with_paragraph(vec![
                ComponentLine::new(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("查看内置主题：").color_range(2, ..),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                        "https://zellij.dev/documentation/theme-list",
                    )))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(theme_list_selected),
                        Box::new(theme_list_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://zellij.dev/documentation/theme-list".to_owned(),
                        link_executable.clone(),
                    )),
                ]),
                ComponentLine::new(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("或创建你自己的主题：").color_range(2, ..),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                        "https://zellij.dev/documentation/themes",
                    )))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(theme_link_selected),
                        Box::new(theme_link_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://zellij.dev/documentation/themes".to_owned(),
                        link_executable.clone(),
                    )),
                ]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_8(link_executable: Rc<RefCell<String>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #8").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("若你用 Alt + <←↓↑→> 或 Alt + <hjkl> 切换窗格焦点并越过")
                        .color_range(0, 34..=36)
                        .color_range(2, 40..=45)
                        .color_range(0, 50..=52)
                        .color_range(2, 56..=61),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("屏幕左右边缘，将会聚焦到下一个或上一个标签页。"),
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_9(link_executable: Rc<RefCell<String>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #9").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("想看社区贡献的插件、集成与教程，可查看"),
                ))]),
                ComponentLine::new(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(
                        Text::new("Awesome-zellij 仓库：").color_range(2, ..=39),
                    )),
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                        "https://github.com/zellij-org/awesome-zellij",
                    )))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(awesome_zellij_link_text_selected),
                        Box::new(awesome_zellij_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/zellij-org/awesome-zellij".to_owned(),
                        link_executable.clone(),
                    )),
                ]),
            ])
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("社区与支持：").color_range(2, ..),
                ))]),
                ComponentLine::new(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new("Discord: "))),
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                        "https://discord.com/invite/CrUAFH3",
                    )))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(discord_link_text_selected),
                        Box::new(discord_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://discord.com/invite/CrUAFH3".to_owned(),
                        link_executable.clone(),
                    )),
                ]),
                ComponentLine::new(vec![
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new("Matrix: "))),
                    ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                        "https://matrix.to/#/#zellij_general:matrix.org",
                    )))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(matrix_link_text_selected),
                        Box::new(matrix_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://matrix.to/#/#zellij_general:matrix.org".to_owned(),
                        link_executable.clone(),
                    )),
                ]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_10(link_executable: Rc<RefCell<String>>, base_mode: Rc<RefCell<InputMode>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #10").color_range(0, ..))
            .with_bulletin_list(
                BulletinList::new(Text::new("Zellij 会话管理器可以：").color_range(2, 11..=25))
                    .with_items(vec![
                        ActiveComponent::new(TextOrCustomRender::Text(Text::new("创建新会话"))),
                        ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                            "在现有会话间切换",
                        ))),
                        ActiveComponent::new(TextOrCustomRender::Text(Text::new("恢复已退出会话"))),
                        ActiveComponent::new(TextOrCustomRender::Text(Text::new("修改会话名称"))),
                        ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                            "将其他用户从当前会话断开",
                        ))),
                    ]),
            )
            .with_paragraph(vec![ComponentLine::new(vec![ActiveComponent::new(
                TextOrCustomRender::Text(match *base_mode.borrow() {
                    InputMode::Locked => Text::new("可通过以下快捷键打开：Ctrl g + o + w")
                        .color_range(3, 24..=29)
                        .color_indices(3, vec![33, 37]),
                    _ => Text::new("可通过以下快捷键打开：Ctrl o + w")
                        .color_range(3, 24..=29)
                        .color_indices(3, vec![33]),
                }),
            )])])
            .with_paragraph(vec![ComponentLine::new(vec![ActiveComponent::new(
                TextOrCustomRender::Text(
                    Text::new("你也可以将它作为欢迎页：zellij -l welcome").color_range(0, 46..=62),
                ),
            )])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_11(link_executable: Rc<RefCell<String>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #11").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("你可以使用 Alt + [] 更改屏幕上的窗格排列")
                        .color_range(0, 55..=57)
                        .color_range(2, 61..=62),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("这对平铺或浮动窗格都有效，取决于当前可见类型。"),
                ))]),
            ])
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("调整大小或分割窗格会打破这种排列，此时可以"),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("再次按 Alt + [] 以恢复。该状态可在")
                        .color_range(0, 25..=27)
                        .color_range(2, 31..=32),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("屏幕右上角看到。"),
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
    pub fn tip_12(link_executable: Rc<RefCell<String>>, base_mode: Rc<RefCell<InputMode>>) -> Page {
        Page::new()
            .main_screen()
            .with_title(Text::new("Zellij 提示 #12").color_range(0, ..))
            .with_paragraph(vec![
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("你可以在 plugin-manager 中加载、重载并跟踪 Zellij 插件。"),
                ))]),
                ComponentLine::new(vec![ActiveComponent::new(TextOrCustomRender::Text(
                    match *base_mode.borrow() {
                        InputMode::Locked => Text::new("可通过以下快捷键打开：Ctrl g + o + p")
                            .color_range(3, 24..=29)
                            .color_indices(3, vec![33, 37]),
                        _ => Text::new("可通过以下快捷键打开：Ctrl o + p")
                            .color_range(3, 24..=29)
                            .color_indices(3, vec![33]),
                    },
                ))]),
            ])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(
                    Text::new("了解更多插件信息：").color_range(2, ..),
                )),
                ActiveComponent::new(TextOrCustomRender::Text(Text::new(
                    "https://zellij.dev/documentation/plugins",
                )))
                .with_hover(TextOrCustomRender::CustomRender(
                    Box::new(plugin_docs_link_text_selected),
                    Box::new(plugin_docs_link_text_selected_len),
                ))
                .with_left_click_action(ClickAction::new_open_link(
                    "https://zellij.dev/documentation/plugins".to_owned(),
                    link_executable.clone(),
                )),
            ])])
            .with_paragraph(vec![ComponentLine::new(vec![
                ActiveComponent::new(TextOrCustomRender::Text(support_the_developer_text())),
                ActiveComponent::new(TextOrCustomRender::Text(sponsors_link_text_unselected()))
                    .with_hover(TextOrCustomRender::CustomRender(
                        Box::new(sponsors_link_text_selected),
                        Box::new(sponsors_link_text_selected_len),
                    ))
                    .with_left_click_action(ClickAction::new_open_link(
                        "https://github.com/sponsors/imsnif".to_owned(),
                        link_executable.clone(),
                    )),
            ])])
            .with_help(Box::new(|hovering_over_link, _menu_item_is_selected| {
                tips_help_text(hovering_over_link)
            }))
    }
}

fn sponsors_link_text_unselected() -> Text {
    Text::new("https://github.com/sponsors/imsnif")
}

fn sponsors_link_text_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://github.com/sponsors/imsnif",
        y + 1,
        x + 1
    );
    34
}

fn sponsors_link_text_selected_len() -> usize {
    34
}

fn plugin_docs_link_text_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://zellij.dev/documentation/plugins",
        y + 1,
        x + 1
    );
    40
}

fn plugin_docs_link_text_selected_len() -> usize {
    40
}

fn awesome_zellij_link_text_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://github.com/zellij-org/awesome-zellij",
        y + 1,
        x + 1
    );
    44
}

fn awesome_zellij_link_text_selected_len() -> usize {
    44
}

fn discord_link_text_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://discord.com/invite/CrUAFH3",
        y + 1,
        x + 1
    );
    34
}

fn discord_link_text_selected_len() -> usize {
    34
}

fn matrix_link_text_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://matrix.to/#/#zellij_general:matrix.org",
        y + 1,
        x + 1
    );
    46
}

fn matrix_link_text_selected_len() -> usize {
    46
}

fn stacked_resize_screencast_link_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://zellij.dev/tutorials/stacked-resize",
        y + 1,
        x + 1
    );
    45
}

fn stacked_resize_screencast_link_selected_len() -> usize {
    45
}

fn colliding_keybindings_link_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://zellij.dev/tutorials/colliding-keybindings",
        y + 1,
        x + 1
    );
    51
}

fn colliding_keybindings_link_selected_len() -> usize {
    51
}

fn theme_link_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://zellij.dev/documentation/themes",
        y + 1,
        x + 1
    );
    39
}
fn theme_link_selected_len() -> usize {
    39
}

fn theme_list_selected(x: usize, y: usize) -> usize {
    print!(
        "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://zellij.dev/documentation/theme-list",
        y + 1,
        x + 1
    );
    43
}
fn theme_list_selected_len() -> usize {
    43
}

fn support_the_developer_text() -> Text {
    let support_text = format!("请支持 Zellij 开发者 <3：");
    Text::new(support_text).color_range(3, ..)
}

fn screencasts_link_selected() -> Box<dyn Fn(usize, usize) -> usize> {
    Box::new(move |x, y| {
        print!(
            "\u{1b}[{};{}H\u{1b}[m\u{1b}[1;4mhttps://zellij.dev/screencasts",
            y + 1,
            x + 1,
        );
        30
    })
}

fn screencasts_link_selected_len() -> Box<dyn Fn() -> usize> {
    Box::new(move || 30)
}

fn tips_help_text(hovering_over_link: bool) -> Text {
    if hovering_over_link {
        let help_text = "帮助：点击或 Shift-点击以在浏览器中打开".to_owned();
        Text::new(help_text)
            .color_substring(3, "点击")
            .color_substring(3, "Shift-点击")
    } else {
        let help_text =
            "帮助：<ESC> - 关闭，<↓↑> - 浏览提示，<Ctrl c> - 启动时不再显示提示".to_owned();
        Text::new(help_text)
            .color_substring(1, "<ESC>")
            .color_substring(1, "<↓↑>")
            .color_substring(1, "<Ctrl c>")
    }
}

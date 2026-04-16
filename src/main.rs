#![windows_subsystem = "windows"]

use chrono::Local;
use eframe::egui;
use egui::{FontData, FontDefinitions};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(windows)]
extern "C" {
    fn GetSystemMetrics(nIndex: i32) -> i32;
}
#[cfg(windows)]
const SM_CXSCREEN: i32 = 0;
#[cfg(windows)]
const SM_CYSCREEN: i32 = 1;

// ── 数据层 ──────────────────────────────────────────

#[derive(Clone, Serialize, Deserialize)]
struct Prompt {
    id: u64,
    title: String,
    tag: String,
    content: String,
    created: String,
}

struct PromptDB {
    path: PathBuf,
    data: Vec<Prompt>,
}

impl PromptDB {
    fn new(path: PathBuf) -> Self {
        if !path.exists() {
            let now = Local::now().format("%Y-%m-%d %H:%M").to_string();
            let defaults = vec![
                Prompt {
                    id: 1,
                    title: "代码审查助手".into(),
                    tag: "编程".into(),
                    content: "请审查以下代码，关注：1) 潜在 bug  2) 性能问题  3) 可读性改进".into(),
                    created: now.clone(),
                },
                Prompt {
                    id: 2,
                    title: "论文润色".into(),
                    tag: "学术".into(),
                    content: "请润色以下学术段落，使其更符合 SCI 论文写作规范，保持原意不变。".into(),
                    created: now,
                },
            ];
            fs::write(&path, serde_json::to_string_pretty(&defaults).unwrap()).unwrap();
        }
        let raw = fs::read_to_string(&path).unwrap();
        let data: Vec<Prompt> = serde_json::from_str(&raw).unwrap();
        Self { path, data }
    }

    fn save(&self) {
        fs::write(&self.path, serde_json::to_string_pretty(&self.data).unwrap()).unwrap();
    }

    fn search(&self, keyword: &str) -> Vec<&Prompt> {
        let kw = keyword.to_lowercase();
        self.data
            .iter()
            .filter(|p| {
                p.title.to_lowercase().contains(&kw)
                    || p.tag.to_lowercase().contains(&kw)
                    || p.content.to_lowercase().contains(&kw)
            })
            .collect()
    }

    fn add(&mut self, title: String, tag: String, content: String) -> u64 {
        let max_id = self.data.iter().map(|p| p.id).max().unwrap_or(0);
        let id = max_id + 1;
        let created = Local::now().format("%Y-%m-%d %H:%M").to_string();
        self.data.push(Prompt { id, title, tag, content, created });
        self.save();
        id
    }

    fn update(&mut self, id: u64, title: String, tag: String, content: String) {
        if let Some(p) = self.data.iter_mut().find(|p| p.id == id) {
            p.title = title;
            p.tag = tag;
            p.content = content;
            self.save();
        }
    }

    fn delete(&mut self, id: u64) {
        self.data.retain(|p| p.id != id);
        self.save();
    }

    fn all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self.data.iter().map(|p| p.tag.clone()).collect();
        tags.sort();
        tags.dedup();
        tags
    }
}

// ── 编辑器 ──────────────────────────────────────────

#[derive(Clone, Copy)]
enum EditorMode {
    New,
    Edit(u64),
}

struct EditorState {
    open: bool,
    mode: EditorMode,
    title: String,
    tag: String,
    content: String,
}

// ── 主应用 ──────────────────────────────────────────

struct App {
    db: PromptDB,
    search: String,
    tag_filter: String,
    selected_id: Option<u64>,
    preview_content: String,
    editor: EditorState,
    confirm_delete: Option<u64>,
    status: String,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 加载系统中文字体
        let mut fonts = FontDefinitions::default();
        for font_path in &["C:/Windows/Fonts/msyh.ttc", "C:/Windows/Fonts/simsun.ttc"] {
            if let Ok(data) = std::fs::read(font_path) {
                let font_name = font_path.split('/').next_back().unwrap_or("chinese");
                fonts.font_data.insert(
                    font_name.to_owned(),
                    FontData::from_owned(data).into(),
                );
                fonts.families.entry(egui::FontFamily::Proportional).or_default()
                    .insert(0, font_name.to_owned());
                fonts.families.entry(egui::FontFamily::Monospace).or_default()
                    .insert(0, font_name.to_owned());
                break;
            }
        }
        cc.egui_ctx.set_fonts(fonts);

        // 强制浅色主题
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        // 应用 3D 风格样式
        apply_3d_style(&cc.egui_ctx);

        let db = PromptDB::new(data_file_path());
        Self {
            db,
            search: String::new(),
            tag_filter: "全部".into(),
            selected_id: None,
            preview_content: String::new(),
            editor: EditorState {
                open: false,
                mode: EditorMode::New,
                title: String::new(),
                tag: String::new(),
                content: String::new(),
            },
            confirm_delete: None,
            status: "Ctrl+N 新增 | Ctrl+F 搜索 | F2 编辑 | Del 删除".into(),
        }
    }

    fn refresh(&mut self) {
        let count = self.filtered().len();
        self.status = format!("共 {} 条提示词 | Ctrl+N 新增 | Ctrl+F 搜索 | F2 编辑 | Del 删除", count);
    }

    fn filtered(&self) -> Vec<&Prompt> {
        let mut items = if self.search.is_empty() {
            self.db.data.iter().collect()
        } else {
            self.db.search(&self.search)
        };
        if self.tag_filter != "全部" {
            items.retain(|p| p.tag == self.tag_filter);
        }
        items
    }

    fn get_prompt(&self, id: u64) -> Option<Prompt> {
        self.db.data.iter().find(|p| p.id == id).cloned()
    }
}

/// 自定义 3D 风格样式
fn apply_3d_style(ctx: &egui::Context) {
    ctx.style_mut(|style| {
        // 按钮：凸起效果
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(240, 240, 240);
        style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(220, 230, 245);
        style.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(210, 210, 220);

        // 按钮边框（3D 高光/阴影）
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(180, 180, 180),
        };
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(140, 170, 220),
        };
        style.visuals.widgets.active.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(120, 150, 200),
        };
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(3);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(3);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(3);

        // 文本框：内凹效果
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(140, 140, 140),
        };
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke {
            width: 1.5,
            color: egui::Color32::from_rgb(100, 140, 200),
        };
        style.visuals.widgets.active.bg_stroke = egui::Stroke {
            width: 1.5,
            color: egui::Color32::from_rgb(80, 120, 190),
        };
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::ZERO;

        // 面板背景
        style.visuals.panel_fill = egui::Color32::from_rgb(240, 240, 240);

        // 窗口圆角
        style.visuals.window_fill = egui::Color32::from_rgb(245, 245, 245);
        style.visuals.window_stroke = egui::Stroke {
            width: 1.0,
            color: egui::Color32::from_rgb(160, 160, 160),
        };
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
            ui.spacing_mut().button_padding = egui::vec2(10.0, 4.0);
            ui.spacing_mut().interact_size.y = 28.0;

            // ── 标题居中 ──
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.heading("AI 提示词管理器");
            });
            ui.separator();

            // ── 搜索栏 ──
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label("搜索:");

                // 3D 搜索框
                let search_resp = ui.add(
                    egui::TextEdit::singleline(&mut self.search)
                        .hint_text("输入关键词...")
                        .desired_width(200.0),
                );
                if search_resp.changed() {
                    self.selected_id = None;
                    self.preview_content.clear();
                    self.refresh();
                }
                ui.add_space(4.0);

                // 标签筛选
                let tags = self.db.all_tags();
                let mut all_tags = vec!["全部".to_string()];
                all_tags.extend(tags);
                let current_tag = self.tag_filter.clone();
                egui::ComboBox::new("tag_filter", "")
                    .selected_text(&current_tag)
                    .width(90.0)
                    .show_ui(ui, |ui| {
                        for tag in &all_tags {
                            if ui.selectable_label(&current_tag == tag, tag).clicked() {
                                self.tag_filter = tag.clone();
                                self.selected_id = None;
                                self.preview_content.clear();
                                self.refresh();
                            }
                        }
                    });

                ui.add_space(8.0);

                // 3D 按钮
                if ui.add(egui::Button::new("➕ 新增")).clicked() {
                    let default_tag = self.db.all_tags().first().cloned().unwrap_or_default();
                    self.editor = EditorState {
                        open: true,
                        mode: EditorMode::New,
                        title: String::new(),
                        tag: default_tag,
                        content: String::new(),
                    };
                }
                if ui.add(egui::Button::new("✏️ 编辑")).clicked() {
                    if let Some(p) = self.get_prompt(self.selected_id.unwrap_or(0)) {
                        self.editor = EditorState {
                            open: true,
                            mode: EditorMode::Edit(p.id),
                            title: p.title.clone(),
                            tag: p.tag.clone(),
                            content: p.content.clone(),
                        };
                    }
                }
                if ui.add(egui::Button::new("📋 复制")).clicked() {
                    if let Some(id) = self.selected_id {
                        if let Some(p) = self.get_prompt(id) {
                            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                let _ = clipboard.set_text(&p.content);
                                self.status = format!("✅ 已复制到剪贴板 [{}]", p.id);
                            }
                        }
                    }
                }
                if ui.add(egui::Button::new("🗑️ 删除")).clicked() {
                    if let Some(id) = self.selected_id {
                        self.confirm_delete = Some(id);
                    }
                }
                if ui.add(egui::Button::new("📤 导出")).clicked() {
                    let export_path = self.db.path.parent().unwrap().join("Prompts_export.json");
                    if let Ok(json) = serde_json::to_string_pretty(&self.db.data) {
                        if fs::write(&export_path, json).is_ok() {
                            self.status = format!("📤 已导出到 {}", export_path.display());
                        }
                    }
                }
            });

            ui.separator();

            // ── 列表 + 预览 ──
            let total_avail = ui.available_height();
            let status_height = 20.0;
            let bottom_padding = 4.0;
            let usable = (total_avail - status_height - bottom_padding).max(180.0);

            let list_height = (usable * 0.45).max(80.0);
            let preview_height = (usable - list_height - 16.0).max(80.0);

            ui.vertical(|ui| {
                ui.add_space(8.0);

                TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder())
                    .column(Column::exact(80.0))
                    .column(Column::exact(140.0))
                    .min_scrolled_height(0.0)
                    .max_scroll_height(list_height)
                    .header(28.0, |mut header| {
                        header.col(|ui: &mut egui::Ui| {
                            ui.label(egui::RichText::new("标题").strong());
                        });
                        header.col(|ui: &mut egui::Ui| {
                            ui.label(egui::RichText::new("标签").strong());
                        });
                        header.col(|ui: &mut egui::Ui| {
                            ui.label(egui::RichText::new("创建时间").strong());
                        });
                    })
                    .body(|mut body| {
                        // 收集 owned 数据，避免借用 self.filtered() 时的冲突
                        let items: Vec<Prompt> = self.filtered().iter().map(|p| (*p).clone()).collect();
                        let sel = self.selected_id;
                        for p in &items {
                            let is_selected = sel == Some(p.id);
                            let id = p.id;
                            body.row(28.0, |mut row| {
                                row.col(|ui: &mut egui::Ui| {
                                    let resp = ui.label(&p.title);
                                    if resp.clicked() {
                                        ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("row_click"), id));
                                    }
                                    if resp.double_clicked() {
                                        ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("row_double"), id));
                                    }
                                    if is_selected { resp.highlight(); }
                                });
                                row.col(|ui: &mut egui::Ui| {
                                    let resp = ui.label(&p.tag);
                                    if resp.clicked() {
                                        ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("row_click"), id));
                                    }
                                    if is_selected { resp.highlight(); }
                                });
                                row.col(|ui: &mut egui::Ui| {
                                    let resp = ui.label(&p.created);
                                    if resp.clicked() {
                                        ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("row_click"), id));
                                    }
                                    if is_selected { resp.highlight(); }
                                });
                            });
                        }
                    });

                // 读取并清除点击事件
                let clicked_id = ctx.data_mut(|d| d.get_temp::<u64>(egui::Id::new("row_click")));
                ctx.data_mut(|d| d.remove::<u64>(egui::Id::new("row_click")));
                if let Some(id) = clicked_id {
                    self.selected_id = Some(id);
                    if let Some(p) = self.get_prompt(id) {
                        self.preview_content = p.content.clone();
                    }
                }
                let double_id = ctx.data_mut(|d| d.get_temp::<u64>(egui::Id::new("row_double")));
                ctx.data_mut(|d| d.remove::<u64>(egui::Id::new("row_double")));
                if let Some(id) = double_id {
                    if let Some(p) = self.get_prompt(id) {
                        self.editor = EditorState {
                            open: true,
                            mode: EditorMode::Edit(p.id),
                            title: p.title.clone(),
                            tag: p.tag.clone(),
                            content: p.content.clone(),
                        };
                    }
                }

                ui.add_space(4.0);

                // 预览区
                ui.label("💡 预览（编辑后按 Ctrl+S 保存）:");
                egui::Frame::new()
                    .fill(ui.visuals().extreme_bg_color)
                    .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
                    .corner_radius(egui::CornerRadius::same(4))
                    .inner_margin(egui::Margin::same(6))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(preview_height)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.preview_content)
                                        .desired_width(f32::INFINITY)
                                        .font(egui::TextStyle::Monospace),
                                );
                            });
                    });
            });

            ui.separator();

            // ── 状态栏 ──
            ui.add_space(1.0);
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.small(&self.status);
            });
            ui.add_space(2.0);
        });

        // ── 编辑器窗口 ──
        if self.editor.open {
            let mut win_open = true;
            let mode = self.editor.mode;
            let title = match mode {
                EditorMode::New => "新增提示词",
                EditorMode::Edit(_) => "编辑提示词",
            };

            egui::Window::new(title)
                .open(&mut win_open)
                .default_size([500.0, 400.0])
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("标题");
                        ui.text_edit_singleline(&mut self.editor.title);
                        ui.label("标签");
                        ui.text_edit_singleline(&mut self.editor.tag);
                        ui.label("提示词内容");
                        ui.add(
                            egui::TextEdit::multiline(&mut self.editor.content)
                                .desired_rows(12)
                                .desired_width(f32::INFINITY)
                                .font(egui::TextStyle::Monospace),
                        );
                        ui.horizontal(|ui| {
                            if ui.button("💾 保存").clicked() {
                                let t = self.editor.title.clone();
                                if t.trim().is_empty() {
                                    self.status = "⚠️ 标题不能为空".into();
                                    return;
                                }
                                let saved_title = t.clone();
                                let id = match mode {
                                    EditorMode::New => self.db.add(
                                        t,
                                        self.editor.tag.clone(),
                                        self.editor.content.clone(),
                                    ),
                                    EditorMode::Edit(id) => {
                                        self.db.update(
                                            id,
                                            t,
                                            self.editor.tag.clone(),
                                            self.editor.content.clone(),
                                        );
                                        id
                                    }
                                };
                                self.editor.open = false;
                                self.selected_id = Some(id);
                                if let Some(p) = self.get_prompt(id) {
                                    self.preview_content = p.content.clone();
                                }
                                self.refresh();
                                self.status = format!("✅ 已保存 [{}] {}", id, saved_title);
                            }
                            if ui.button("取消").clicked() {
                                self.editor.open = false;
                            }
                        });
                    });
                });
            if !win_open {
                self.editor.open = false;
            }
        }

        // ── 删除确认窗口 ──
        if let Some(del_id) = self.confirm_delete {
            let mut win_open = true;
            let p = self.get_prompt(del_id);
            let title_for_display = p.as_ref().map(|p| p.title.clone()).unwrap_or_default();

            egui::Window::new("确认删除")
                .open(&mut win_open)
                .collapsible(false)
                .default_width(320.0)
                .show(ctx, |ui| {
                    ui.label(format!("确定删除「{}」？", title_for_display));
                    ui.horizontal(|ui| {
                        if ui.button("确定").clicked() {
                            // 窗口外执行
                        }
                        if ui.button("取消").clicked() {
                            self.confirm_delete = None;
                        }
                    });
                });
            if !win_open {
                self.db.delete(del_id);
                self.selected_id = None;
                self.preview_content.clear();
                self.confirm_delete = None;
                self.refresh();
                self.status = "✅ 已删除".into();
            }
        }

        // ── 快捷键 ──
        ctx.input(|i| {
            if i.key_pressed(egui::Key::S) && i.modifiers.ctrl {
                if let Some(id) = self.selected_id {
                    if let Some(p) = self.get_prompt(id) {
                        if p.content != self.preview_content {
                            self.db.update(id, p.title.clone(), p.tag.clone(), self.preview_content.clone());
                            self.status = format!("✅ 已保存 [{}] {}", id, p.title);
                        }
                    }
                }
            }
            if i.key_pressed(egui::Key::F2) {
                if self.selected_id.is_some() {
                    if let Some(p) = self.get_prompt(self.selected_id.unwrap_or(0)) {
                        self.editor = EditorState {
                            open: true,
                            mode: EditorMode::Edit(p.id),
                            title: p.title.clone(),
                            tag: p.tag.clone(),
                            content: p.content.clone(),
                        };
                    }
                }
            }
            if i.key_pressed(egui::Key::Delete) {
                if self.selected_id.is_some() && self.confirm_delete.is_none() {
                    self.confirm_delete = self.selected_id;
                }
            }
        });
    }
}

// ── 工具函数 ────────────────────────────────────────

fn data_file_path() -> PathBuf {
    std::env::current_exe()
        .unwrap_or_default()
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .join("Prompts.json")
}

// ── 主入口 ──────────────────────────────────────────

fn main() -> eframe::Result {
    let win_w = 900.0;
    let win_h = 640.0;

    #[cfg(windows)]
    let viewport = {
        let screen_w = unsafe { GetSystemMetrics(SM_CXSCREEN) } as f32;
        let screen_h = unsafe { GetSystemMetrics(SM_CYSCREEN) } as f32;
        let pos_x = ((screen_w - win_w) / 2.0) as i32;
        let pos_y = ((screen_h - win_h) / 2.0) as i32;
        egui::ViewportBuilder::default()
            .with_inner_size([win_w, win_h])
            .with_min_inner_size([720.0, 500.0])
            .with_title("AI 提示词管理器")
            .with_position([pos_x as f32, pos_y as f32])
    };

    #[cfg(not(windows))]
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size([win_w, win_h])
        .with_min_inner_size([720.0, 500.0])
        .with_title("AI 提示词管理器");

    let native = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "AI 提示词管理器",
        native,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

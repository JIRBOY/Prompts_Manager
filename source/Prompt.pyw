#!/usr/bin/env python3
"""AI 提示词管理器 - 极简单文件版 (pyw)"""

import json
import os
import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
from datetime import datetime

DATA_FILE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "Prompts.json")

# ── 数据层 ──────────────────────────────────────────
class PromptDB:
    def __init__(self, path):
        self.path = path
        if not os.path.exists(path):
            self._save([
                {"id": 1, "title": "代码审查助手", "tag": "编程",
                 "content": "请审查以下代码，关注：1) 潜在 bug  2) 性能问题  3) 可读性改进",
                 "created": datetime.now().strftime("%Y-%m-%d %H:%M")},
                {"id": 2, "title": "论文润色", "tag": "学术",
                 "content": "请润色以下学术段落，使其更符合 SCI 论文写作规范，保持原意不变。",
                 "created": datetime.now().strftime("%Y-%m-%d %H:%M")},
            ])
        self.data = self._load()

    def _load(self):
        with open(self.path, "r", encoding="utf-8") as f:
            return json.load(f)

    def _save(self, data=None):
        if data is not None:
            self.data = data
        with open(self.path, "w", encoding="utf-8") as f:
            json.dump(self.data, f, ensure_ascii=False, indent=2)

    def all(self):
        return self.data

    def search(self, keyword):
        kw = keyword.lower()
        return [p for p in self.data if kw in p["title"].lower()
                or kw in p["tag"].lower() or kw in p["content"].lower()]

    def add(self, title, tag, content):
        ids = [p["id"] for p in self.data]
        new_id = max(ids, default=0) + 1
        self.data.append({
            "id": new_id, "title": title, "tag": tag,
            "content": content,
            "created": datetime.now().strftime("%Y-%m-%d %H:%M")
        })
        self._save()
        return new_id

    def update(self, pid, title, tag, content):
        for p in self.data:
            if p["id"] == pid:
                p.update({"title": title, "tag": tag, "content": content})
                self._save()
                return

    def delete(self, pid):
        self.data = [p for p in self.data if p["id"] != pid]
        self._save()

    def tags(self):
        return sorted(set(p["tag"] for p in self.data if p["tag"]))


# ── 居中工具 ────────────────────────────────────────
def center_window(win, w=600, h=480):
    """弹窗居中"""
    sw = win.winfo_screenwidth()
    sh = win.winfo_screenheight()
    x = (sw - w) // 2
    y = (sh - h) // 2
    win.geometry(f"{w}x{h}+{x}+{y}")


# ── UI ──────────────────────────────────────────────
class App:
    def __init__(self, root):
        self.db = PromptDB(DATA_FILE)
        self.root = root
        self.root.title("AI 提示词管理器")
        self.root.geometry("900x640")
        self.root.minsize(720, 500)
        # 主窗口也居中
        center_window(root, 900, 640)
        self._auto_save_after = None  # 自动保存延迟器
        self._style()
        self._build()
        self._refresh()

    # ── 样式 ──
    def _style(self):
        s = ttk.Style()
        s.theme_use("clam")
        s.configure(".", font=("Microsoft YaHei UI", 10), background="#f5f5f5")
        s.configure("Treeview", font=("Microsoft YaHei UI", 9), rowheight=28)
        s.configure("Treeview.Heading", font=("Microsoft YaHei UI", 9, "bold"))
        s.configure("TButton", padding=6)
        s.configure("Header.TLabel", font=("Microsoft YaHei UI", 13, "bold"),
                     foreground="#333")
        s.configure("Toolbar.TButton", padding=(10, 4))

    # ── 布局 ──
    def _build(self):
        root = self.root
        root.columnconfigure(0, weight=1)
        root.rowconfigure(2, weight=1)

        # ── 第 1 行：标题栏 ──
        title_bar = ttk.Frame(root, padding=(12, 6))
        title_bar.grid(row=0, column=0, sticky="ew")
        ttk.Label(title_bar, text="📋 AI 提示词管理器", style="Header.TLabel").pack(side="left")

        # ── 第 2 行：搜索行（单独一行） ──
        search_bar = ttk.Frame(root, padding=(12, 2, 12, 6))
        search_bar.grid(row=1, column=0, sticky="ew")
        search_bar.columnconfigure(1, weight=1)

        ttk.Label(search_bar, text="🔍").grid(row=0, column=0, padx=(0, 4))
        self.search_var = tk.StringVar()
        self.search_var.trace_add("write", lambda *_: self._refresh())
        self.search_entry = ttk.Entry(search_bar, textvariable=self.search_var, font=("", 10))
        self.search_entry.grid(row=0, column=1, sticky="ew", padx=(0, 8))
        self.search_entry.focus_set()  # 启动时聚焦搜索框

        self.tag_var = tk.StringVar(value="全部")
        self.tag_cb = ttk.Combobox(search_bar, textvariable=self.tag_var, width=10, state="readonly")
        self.tag_cb.grid(row=0, column=2, padx=4)
        self.tag_cb.bind("<<ComboboxSelected>>", lambda *_: self._refresh())

        # 按钮组
        btn_frame = ttk.Frame(search_bar)
        btn_frame.grid(row=0, column=3, padx=(4, 0))
        ttk.Button(btn_frame, text="➕ 新增", style="Toolbar.TButton", command=self._open_editor).pack(side="left", padx=2)
        ttk.Button(btn_frame, text="✏️ 编辑", style="Toolbar.TButton", command=self._edit).pack(side="left", padx=2)
        ttk.Button(btn_frame, text="📋 复制", style="Toolbar.TButton", command=self._copy).pack(side="left", padx=2)
        ttk.Button(btn_frame, text="🗑️ 删除", style="Toolbar.TButton", command=self._delete).pack(side="left", padx=2)
        ttk.Button(btn_frame, text="📤 导出", style="Toolbar.TButton", command=self._export).pack(side="left", padx=2)

        # ── 第 3 行：列表 + 预览 ──
        content_frame = ttk.Frame(root, padding=(12, 0, 12, 10))
        content_frame.grid(row=2, column=0, sticky="nsew")
        content_frame.columnconfigure(0, weight=1)
        content_frame.rowconfigure(0, weight=3)  # 列表 3 份
        content_frame.rowconfigure(1, weight=2)  # 预览 2 份

        cols = ("title", "tag", "created")
        self.tree = ttk.Treeview(content_frame, columns=cols, show="headings", selectmode="browse")
        self.tree.heading("title", text="标题")
        self.tree.heading("tag", text="标签")
        self.tree.heading("created", text="创建时间")
        self.tree.column("title", width=360, minwidth=150)
        self.tree.column("tag", width=80, minwidth=60, anchor="center")
        self.tree.column("created", width=140, minwidth=100, anchor="center")
        self.tree.grid(row=0, column=0, sticky="nsew")

        sb = ttk.Scrollbar(content_frame, orient="vertical", command=self.tree.yview)
        sb.grid(row=0, column=1, sticky="ns")
        self.tree.configure(yscrollcommand=sb.set)
        self.tree.bind("<Double-1>", lambda *_: self._edit())
        self.tree.bind("<<TreeviewSelect>>", lambda *_: self._show_detail())

        # 键盘快捷键
        self.tree.bind("<Delete>", lambda *_: self._delete())
        self.tree.bind("<F2>", lambda *_: self._edit())
        self.root.bind("<Control-f>", lambda *_: self.search_entry.focus_set())
        self.root.bind("<Control-n>", lambda *_: self._open_editor())
        self.root.bind("<Control-s>", lambda *_: self._export())

        # 预览区（默认可编辑）
        preview_frame = ttk.Frame(content_frame)
        preview_frame.grid(row=1, column=0, columnspan=2, sticky="nsew", pady=(6, 0))
        preview_frame.columnconfigure(0, weight=1)
        preview_frame.rowconfigure(1, weight=1)

        ttk.Label(preview_frame, text="💡 预览（直接编辑自动保存）").pack(anchor="w")
        self.detail_text = scrolledtext.ScrolledText(preview_frame, height=6, font=("Consolas", 10),
                                                      wrap=tk.WORD, bg="#fafafa")
        self.detail_text.pack(fill="both", expand=True, pady=(2, 0))
        # 可编辑 + 自动保存
        self.detail_text.bind("<KeyRelease>", lambda *_: self._schedule_auto_save())
        self.detail_text.bind("<FocusOut>", lambda *_: self._flush_auto_save())

        # 状态栏
        self.status_var = tk.StringVar(value="就绪 | Ctrl+N 新增 | Ctrl+F 搜索 | F2 编辑 | Del 删除")
        ttk.Label(root, textvariable=self.status_var, padding=(12, 4),
                  foreground="#888").grid(row=3, column=0, sticky="w")

    # ── 操作 ──
    def _refresh(self):
        for i in self.tree.get_children():
            self.tree.delete(i)
        kw = self.search_var.get().strip()
        tag = self.tag_var.get()
        items = self.db.search(kw) if kw else self.db.all()
        if tag != "全部":
            items = [p for p in items if p["tag"] == tag]
        for p in items:
            self.tree.insert("", "end", iid=p["id"],
                             values=(p["title"], p["tag"], p["created"]))
        # 更新标签
        tags = ["全部"] + self.db.tags()
        self.tag_cb["values"] = tags
        self.status_var.set(f"共 {len(items)} 条提示词 | Ctrl+N 新增 | Ctrl+F 搜索 | F2 编辑 | Del 删除")

    def _selected(self):
        sel = self.tree.selection()
        return int(sel[0]) if sel else None

    def _get_prompt(self, pid):
        for p in self.db.all():
            if p["id"] == pid:
                return p
        return None

    def _show_detail(self):
        pid = self._selected()
        if pid is None:
            return
        p = self._get_prompt(pid)
        if p:
            self.detail_text.configure(state="normal")
            self.detail_text.delete("1.0", tk.END)
            self.detail_text.insert("1.0", p["content"])
            # 不 disable，保持可编辑

    # ── 自动保存 ──
    def _schedule_auto_save(self):
        """延迟 800ms 后保存，避免频繁 IO"""
        if self._auto_save_after:
            self.root.after_cancel(self._auto_save_after)
        self._auto_save_after = self.root.after(800, self._flush_auto_save)

    def _flush_auto_save(self):
        """将预览区内容保存到数据库"""
        pid = self._selected()
        if pid is None:
            return
        new_content = self.detail_text.get("1.0", tk.END).strip()
        p = self._get_prompt(pid)
        if p and p["content"] != new_content:
            self.db.update(pid, p["title"], p["tag"], new_content)
            self.status_var.set(f"✅ 已自动保存 [{pid}] {p['title']}")

    # ── 弹窗编辑器 ──
    def _open_editor(self, pid=None):
        w = tk.Toplevel(self.root)
        w.title("编辑提示词" if pid else "新增提示词")
        center_window(w, 640, 500)
        w.transient(self.root)
        w.grab_set()

        # 加载数据
        p = {}
        if pid:
            p = self._get_prompt(pid) or {}

        ttk.Label(w, text="标题").pack(anchor="w", padx=24, pady=(14, 0))
        title_var = tk.StringVar(value=p.get("title", ""))
        title_entry = ttk.Entry(w, textvariable=title_var, font=("", 11))
        title_entry.pack(fill="x", padx=24, pady=4)

        ttk.Label(w, text="标签").pack(anchor="w", padx=24, pady=(4, 0))
        tag_var = tk.StringVar(value=p.get("tag", ""))
        tag_entry = ttk.Combobox(w, textvariable=tag_var, font=("", 11),
                                  values=self.db.tags())
        tag_entry.pack(fill="x", padx=24, pady=4)

        ttk.Label(w, text="提示词内容").pack(anchor="w", padx=24, pady=(4, 0))
        content = scrolledtext.ScrolledText(w, height=16, font=("Consolas", 10), wrap=tk.WORD)
        content.pack(fill="both", expand=True, padx=24, pady=4)
        if pid and "content" in p:
            content.insert("1.0", p["content"])

        # 聚焦
        if pid:
            title_entry.focus_set()
        else:
            title_entry.focus_set()

        btns = ttk.Frame(w, padding=(0, 8))
        btns.pack(fill="x", padx=24)

        def save():
            t = title_var.get().strip()
            tg = tag_var.get().strip()
            c = content.get("1.0", tk.END).strip()
            if not t:
                messagebox.showwarning("提示", "标题不能为空", parent=w)
                title_entry.focus_set()
                return
            saved_pid = pid  # 捕获当前 pid
            if saved_pid:
                self.db.update(saved_pid, t, tg, c)
            else:
                saved_pid = self.db.add(t, tg, c)
            # 先关闭窗口，再刷新主界面（避免焦点冲突）
            w.destroy()
            # 刷新列表
            self._refresh()
            # 选中刚保存的项目
            self._select_by_id(saved_pid)
            # 更新预览区
            self._show_detail()
            self.status_var.set(f"✅ 已保存 [{saved_pid}] {t}")

        ttk.Button(btns, text="💾 保存", command=save).pack(side="right", padx=4)
        ttk.Button(btns, text="取消", command=w.destroy).pack(side="right", padx=4)

        # 快捷键
        w.bind("<Control-Return>", lambda *_: save())
        w.bind("<Escape>", lambda *_: w.destroy())

    def _select_by_id(self, pid):
        """在列表中选中指定 id 的项目"""
        target = str(pid)
        for item in self.tree.get_children():
            if item == target:
                self.tree.selection_set(item)
                self.tree.focus(item)
                self.tree.see(item)
                break

    def _edit(self):
        pid = self._selected()
        if pid:
            self._open_editor(pid)

    def _delete(self):
        pid = self._selected()
        if pid is None:
            return
        if not messagebox.askyesno("确认", "确定删除此提示词？"):
            return
        self.db.delete(pid)
        self._refresh()
        self.detail_text.delete("1.0", tk.END)
        self.status_var.set("✅ 已删除")

    def _copy(self):
        pid = self._selected()
        if pid is None:
            return
        p = self._get_prompt(pid)
        if p:
            self.root.clipboard_clear()
            self.root.clipboard_append(p["content"])
            self.status_var.set("✅ 已复制到剪贴板")

    def _export(self):
        path = os.path.join(os.path.dirname(DATA_FILE), "Prompts_export.json")
        with open(path, "w", encoding="utf-8") as f:
            json.dump(self.db.data, f, ensure_ascii=False, indent=2)
        self.status_var.set(f"📤 已导出到 {path}")
        messagebox.showinfo("导出", f"已导出 {len(self.db.data)} 条提示词\n{path}", parent=self.root)


if __name__ == "__main__":
    root = tk.Tk()
    App(root)
    root.mainloop()

# Command Mindmap Executor 使用說明

[English README](README.md)

Command Mindmap Executor 是一個本地優先的桌面工具，讓你用心智圖方式整理可重用的命令片段，並在需要時輸出成可重用的命令或腳本流程。

## 這個專案可以做什麼

- 用 Tree View 和 Graph View 建立命令流程
- 用模板重用常見命令片段
- 在輸出前先預覽命令或腳本結果
- 將資料保存在本機
- 用 JSON 匯入與匯出 mindmap 與模板資料

## 適合誰使用

這個工具適合需要長期整理與重用命令列流程的人，例如：

- 經常使用 Linux shell、WSL 或 PowerShell 的開發者
- 想把常用指令整理成可視化流程的人
- 需要在多台機器之間搬移命令集的人
- 想把零散筆記整理成可重用模板的人

## 基本流程

1. 建立一張 command mindmap
2. 加入或重用命令模板
3. 連接節點並選擇主輸出路徑
4. 預覽最終命令或腳本
5. 儲存於本機或匯出成 JSON

## 第一版平台範圍

第一版主要針對以下三種目標平台：

- Linux shell
- WSL
- Windows PowerShell

## 本地優先設計

本專案的設計原則是讓資料保留在你的電腦上：

- 本機保存為預設行為
- 不依賴雲端服務才能使用
- 透過匯入與匯出功能做備份與跨主機搬移

## 開發模式啟動

如果你想在本機啟動開發環境：

```bash
npm install
npm run tauri:dev
```

如果你想打包桌面應用程式：

```bash
npm run tauri:build
```

## 文件

- 產品架構總覽：[docs/architecture/overview.md](docs/architecture/overview.md)
- Tauri command 契約：[docs/reference/tauri-command-contracts.md](docs/reference/tauri-command-contracts.md)
- SQLite schema 草案：[docs/reference/sqlite-schema.md](docs/reference/sqlite-schema.md)


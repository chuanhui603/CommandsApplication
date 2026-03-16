# Command Mindmap Executor 架構與介面規格

本文件提供給產品/技術使用者快速理解本專案的產品結構與 Tauri 介面。

## 產品介紹

Command Mindmap Executor 是一個本地優先（local-first）的桌面工具，讓使用者用心智圖整理可重用命令片段，並可匯入/匯出 JSON 在不同環境搬移。

## 產品結構

1. **編輯層**：Tree View + Graph View 編排命令流程。  
2. **資料層**：以 SQLite 儲存 mindmap、範本、產出結果與設定。  
3. **桌面能力層**：由 Tauri 提供本機檔案操作與桌面封裝。  

## 系統邊界

- Vue / TypeScript：UI、狀態、驗證流程、命令生成流程。
- Tauri / Rust：啟動、SQLite、檔案 I/O、型別化回傳。



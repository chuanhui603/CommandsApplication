# Command Mindmap Executor

[English README](README.md)

Command Mindmap Executor 是一個本地優先的桌面應用，讓你用心智圖方式整理可重用命令，並在需要時輸出成 command 或 script。

如果你只是想把專案跑起來，先看下面這段就夠了。

## 直接照做：Windows 開啟步驟

### 1. 安裝必要工具

請先確認你的電腦有這些工具：

- Node.js 20 以上
- Rust toolchain
- Microsoft Visual Studio C++ Build Tools
- WebView2 Runtime

建議用下面指令確認：

```powershell
node -v
npm -v
rustc -V
cargo -V
```

只要其中任何一個指令出現找不到，就要先安裝對應工具。

### 2. 進入專案資料夾

```powershell
cd C:\Users\charkchao\Desktop\API_TEST\CommandsExecuter
```

### 3. 安裝 JavaScript 依賴

```powershell
npm install
```


### 4. 安裝 Tauri CLI

```powershell
npm install -D @tauri-apps/cli
```

### 5. 啟動桌面版開發模式

```powershell
npm run tauri:dev
```

這個指令會同時做兩件事：

- 啟動 Vite 前端開發伺服器
- 啟動 Tauri 桌面視窗

成功後，你會看到一個桌面 App 視窗，而不是只有瀏覽器頁面。

## 如果你只想先看前端畫面

```powershell
npm run dev
```

然後打開終端機顯示的本機網址，通常會是：

```text
http://localhost:5173
```

但這種方式只有前端，不能驗證 Tauri / SQLite 相關功能。

## 最常見的失敗原因

### `tauri` 不是內部或外部命令

通常表示還沒安裝專案內的 CLI，執行：

```powershell
npm install -D @tauri-apps/cli
```

再重跑：

```powershell
npm run tauri:dev
```

### Rust 或 C++ toolchain 缺少

如果出現編譯失敗、linker 失敗、找不到 MSVC 之類的錯誤，通常是這兩個沒裝完整：

- Rust toolchain
- Visual Studio C++ Build Tools

### 只有瀏覽器打開，沒有桌面視窗

你跑的是：

```powershell
npm run dev
```

不是：

```powershell
npm run tauri:dev
```

## 建置桌面安裝包

```powershell
npm run tauri:build
```

建置完成後，可到 Tauri 輸出資料夾查看產物。

## 專案重點

- 用 Tree View 和 Graph View 建立命令流程
- 重用 Linux shell、WSL、Windows PowerShell 範本
- 預覽輸出的 command 或 script
- 資料本機保存
- 支援 JSON 匯入與匯出

## 相關文件

- 架構總覽：[docs/architecture/overview.md](docs/architecture/overview.md)
- Tauri command 契約：[docs/reference/tauri-command-contracts.md](docs/reference/tauri-command-contracts.md)
- SQLite schema 草案：[docs/reference/sqlite-schema.md](docs/reference/sqlite-schema.md)


## 3️⃣ README.md

```md
# tg.nvim

A real-time Telegram client for the terminal, with **Neovim-style keybindings**.

`vimgram` lets you chat on Telegram entirely from your CLI using `hjkl` navigation,
modal editing, and a fast, minimal TUI — built in Rust.

---

## Why vimgram?

Most Telegram clients are:

- GUI-heavy
- Mouse-driven
- Distracting

`vimgram` is built for developers who live in:

- terminals
- SSH sessions
- tmux
- Neovim

If you can use Vim, you already know how to use tg.nvim.

---

## What this is (and isn’t)

### ✅ This is

- A **real Telegram client** (MTProto, not bot API)
- Real-time messages (DMs, groups, channels)
- A long-running terminal app (like `nvim`, `htop`)
- Vim-style navigation and modes

### ❌ This is not

- A Telegram bot
- A one-shot CLI command
- A wrapper around Telegram Desktop

---

## Core design principles

1. **State-driven**
   - UI never talks directly to Telegram
   - Everything flows through `AppState`

2. **Async-first**
   - Telegram updates
   - Keyboard input
   - UI rendering  
     all run concurrently

3. **Vim-native UX**
   - `hjkl` navigation
   - Normal / Insert modes
   - Zero mouse, zero friction

4. **Build one small thing at a time**
   - No big bang
   - Every step must be runnable

---

## Tech stack

- **Rust**
- **tokio** — async runtime
- **grammers** — Telegram MTProto client
- **ratatui** — terminal UI
- **crossterm** — keyboard + terminal control

---

## Project structure (target)
```

src/
├── main.rs # entry point
├── app.rs # AppState & mode logic
├── telegram/
│ ├── client.rs # Telegram connection
│ ├── auth.rs # login & session
│ └── updates.rs # update stream
├── ui/
│ ├── draw.rs # rendering
│ └── input.rs # key handling (hjkl)
└── state.rs # shared state models

```

---

## Vim-style keybindings (initial)

### Modes
- **NORMAL** → navigation
- **INSERT** → typing messages

### NORMAL mode
| Key | Action |
|----|------|
| j / k | move down / up |
| h / l | switch panels |
| gg | jump to top |
| G | jump to bottom |
| i | enter insert mode |
| Ctrl+C | quit |

### INSERT mode
| Key | Action |
|----|------|
| text | type message |
| Enter | send message |
| Esc | normal mode |

---

## Development roadmap (step-by-step)

### Step 1 — Boot & login
**Goal:** Connect to Telegram and authenticate

- Setup `grammers`
- Login via phone + OTP
- Persist session to disk

✅ Output:
`Logged in successfully`

---

### Step 2 — Receive messages
**Goal:** Prove real-time updates work

- Listen to `next_update()`
- Print incoming messages to stdout

✅ Output:
```

Alice: hey
Bob: did you see this?

```

(no UI yet)

---

### Step 3 — Minimal TUI
**Goal:** Own the terminal

- Enable raw mode
- Draw a static box
- Exit cleanly

✅ Output:
```

┌──── tg.nvim ────┐
│ │
└─────────────────┘

````

---

### Step 4 — AppState
**Goal:** Centralize logic

- Chats
- Messages
- Cursor
- Mode (NORMAL / INSERT)

No Telegram, no UI logic here — just state.

---

### Step 5 — Keyboard handling (hjkl)
**Goal:** Vim muscle memory

- Implement NORMAL vs INSERT
- `hjkl`, `i`, `Esc`

At this point, it should **feel like Vim** even without Telegram.

---

### Step 6 — Wire Telegram ↔ UI
**Goal:** Real chat

- Telegram updates → AppState
- User input → send messages
- UI reflects state

This is the “it’s alive” moment.

---

## How to run (eventually)

```bash
cargo build --release
./target/release/tg.nvim
````

---

## Philosophy

> Make it usable first.
> Make it powerful second.
> Make it beautiful last.

If it feels good with:

- one chat
- hjkl
- live messages

you’ve already won.

---

## Status

🚧 Under active development
🧠 Built step-by-step, intentionally
🦀 Written in Rust

```

---

## 4️⃣ How I recommend you proceed (very important)

**Do NOT jump ahead.**

Next steps should be **exactly**:
1. `cargo new tg.nvim`
2. Telegram login working
3. Print messages
4. Then TUI

If you want, next I can:
- Walk you through **Step 1 line by line**
- Create a **commit-by-commit plan**
- Help you write the **first 50 lines** cleanly

Just say:
👉 **“Let’s start Step 1”**
```

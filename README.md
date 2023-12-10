# uuid-rs.nvim
## Setup
### lazy.nvim

Add this your `init.lua`

```lua
require("lazy").setup({{"tokikokoko/uuid-rs.nvim", build = ":UuidBuild"}})
```

## Usage

Generate a UUID and set register.

```bash
:Uuid
# => 625d7129-0371-44c2-8adf-012ec26af02e
```


# dotenv

A library for loading .env files for AOCL.

## Usage

```aocl
import "dotenv.aocl"

# Loading from '.env'
call lib_dotenv_load

# Loading from a custom file
@stack:push "custom.env"
call lib_dotenv_load_from
```

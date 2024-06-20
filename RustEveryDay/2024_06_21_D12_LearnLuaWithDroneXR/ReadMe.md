


I did learn a bit of mlua.

But would would be cool is to make a lua code that update every 0.05 seconods in Rust

This lua received the bytes from the Drone Soccer game a bytes when UDP rust received it.
And lua script can send in return only integer value with the format 2099887766



Code to start:

```

use mlua::{Lua, Result, Value};

fn main() -> Result<()> {
    // Initialize a new Lua state
    let lua = Lua::new();

    // Define a Lua script to be run
    let lua_script = r#"
        -- Lua script content
        counter = counter or 0
        function update()
            counter = counter + 1
            print("Counter:", counter)
        end
    "#;

    // Load and execute the Lua script
    lua.load(lua_script).exec()?;

    // Get the Lua function `update` and keep running it in a loop
    let update_func = lua.globals().get::<_, mlua::Function>("update")?;

    for _ in 0..10 {
        // Call the Lua update function
        update_func.call::<_, ()>(())?;
        
        // Sleep for a bit to simulate time passing
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}

```
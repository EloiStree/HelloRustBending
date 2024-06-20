use std::net::UdpSocket;

use mlua::prelude::*;


//INSAL LUA https://www.youtube.com/watch?v=zXW7YqoMUpA
// https://lua.org/download.html
//https://github.com/rjpcomputing/luaforwindows/releases
//https://www.lua.org



fn execute_variable_given_at_start(){

    // Create a new Lua state
    let lua = Lua::new();

    // Push text to Lua
    let text = "Hello, Luaaaa !";
    
    let _= lua.globals().set("my_text", text);

    // Push bytes to Lua
    let bytes = vec![0x01, 0x02, 0x03, 0x04];
    let _= lua.globals().set("my_bytes", bytes);

    // Run a Lua script that prints the pushed values
    let _=  lua.load(r#"
        print("Text:", my_text)
        for i = 1, #my_bytes do
            io.write(string.format("%02X ", my_bytes:byte(i)))
        end
        print()
    "#).exec();


}


fn call_lua_function_greeting() -> Result<(), LuaError> {
    println!("Hello, world from rust!");
    // Create a new Lua state
    let lua = Lua::new();

    // Define a Lua script
    let lua_script = r#"
        function greet(name)
            return "Hello, " .. name .. " (from Lua)"
        end
    "#;

    // Load and execute the Lua script
    lua.load(lua_script).exec()?;

    // Get the Lua function 'greet'
    let greet: mlua::Function = lua.globals().get("greet")?;

    // Call the Lua function with a parameter
    let result: String = greet.call("World")?;

    // Print the result
    println!("{}", result);

    Ok(())
}

// Define a Rust function to add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}


fn send_message_as_text_udp(message: String) {
    println!("Sending message as text udp 1235: {}", message);
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind socket");
    socket.send_to(message.as_bytes(), "127.0.0.1:1235").expect("Failed to send message");
}
fn send_message_as_bytes_udp(message: &[u8]) {
    println!("Sending message as bytes udp 1236: {}", message.len());
    let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind socket");
    socket.send_to(message, "127.0.0.1:1236").expect("Failed to send message");
}


fn send_message_as_text(message: String) {
    println!("Sending message as text: {}", message);
    send_message_as_text_udp(message.to_string());
    
}
fn send_message_as_byte_integer(message: i32) {
    println!("Sending message as integer: {}", message);
    //Change later and test to be sure it is little endian signed integer.
    let bytes = message.to_le_bytes();
    send_message_as_bytes_udp(&bytes);
}


fn send_message_as_bytes(bytes: &[u8]) {
    // Your logic for sending a message as bytes
    println!("Sending message: {:?}", bytes);
}

fn test_lua_pushing_text_bytes() -> Result<(), LuaError>{


    // Create a new Lua context
    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("add", lua.create_function(|_, (a, b): (i32, i32)| {
        Ok(add(a, b))
    })?)?;

    globals.set("send_message_as_text", lua.create_function(|_, (message):(String)| {
        Ok(send_message_as_text(message))
    })?)?; 
    globals.set("send_message_as_byte_integer", lua.create_function(|_, (message):(i32)| {
        Ok(send_message_as_byte_integer(message))
    })?)?;

    
    // let send_message_as_bytes_fn = lua.create_function(|_, args: mlua::MultiValue| {
    //     let (bytes,): (Vec<u8>,) = FromLuaMulti::from_lua_multi(args)?;
    //     send_message_as_bytes(&bytes)?;
    //     Ok(())
    // })?;
    //globals.set("send_message_as_bytes", send_message_as_bytes_fn)?;
    
    // Register `send_message_as_text` function
    // globals.set("send_message_as_text", lua.create_function(send_message_as_text)?)?;
    // globals.set("send_message_as_bytes", lua.create_function(send_message_as_bytes)?)?;

    // Example Lua script that calls the registered functions
    let script = r#"
        print(add(3, 4)) -- Should print 7
        send_message_as_text("Hello, world!")
        send_message_as_byte_integer(2099887766)
        -- send_message_as_bytes({1, 2, 3, 4})
    "#;

    // Execute the Lua script
    lua.load(script).exec()?;

    Ok(())

}

fn main() -> LuaResult<()> {

call_lua_function_greeting();
execute_variable_given_at_start();

//-------------------------------

test_lua_pushing_text_bytes();




    Ok(())
}

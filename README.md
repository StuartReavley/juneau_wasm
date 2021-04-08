# Jasm -> Wasm

Hi Andika, welcome to the team - really excited to be working with you.

I've laid out a couple of files in the packages/juneau/src/building/jasm_wasm folder to help get you started. Obviously, you will need time to get aquainted with the codebase.

 - ./test/wasmer.rs : we're using Wasmer as a test harness to run WebAssembly without creating a browser instance. This gives you an example of how each test is set up to be run with Wasmer
 - ./test/sum.rs : here's the first example of a test - compiling a simple add function. I suggest you start here as the first major test, as this requires quite a bit of knowledge to complete!
 - ./mod.rs : I've laid out the basic function, along with some initial helper code to give you a head start looking around


Note, if you are using VSCode (like me), then the tasks have a "build", "test", "cargo check" already set up :-)
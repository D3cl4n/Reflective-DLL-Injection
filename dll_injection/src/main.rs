mod process;

fn main() 
{
    println!("Hello, world!");
    unsafe 
    {
        process::open_process(5128);
    }
}


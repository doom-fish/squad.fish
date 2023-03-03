use screencapturekit::SCShareableContent;


fn main() {
    let a = SCShareableContent::current();
    println!("{:?}", a.displays);
    println!("{:?}", a.applications);
    println!("{:?}", a.windows);
    println!("TEST");
}

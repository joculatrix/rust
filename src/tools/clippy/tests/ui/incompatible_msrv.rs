#![warn(clippy::incompatible_msrv)]
#![feature(custom_inner_attributes)]
#![clippy::msrv = "1.3.0"]

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::future::Future;
use std::thread::sleep;
use std::time::Duration;

fn foo() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    assert_eq!(map.entry("poneyland").key(), &"poneyland");
    //~^ incompatible_msrv

    if let Entry::Vacant(v) = map.entry("poneyland") {
        v.into_key();
        //~^ incompatible_msrv
    }
    // Should warn for `sleep` but not for `Duration` (which was added in `1.3.0`).
    sleep(Duration::new(1, 0));
    //~^ incompatible_msrv
}

#[test]
fn test() {
    sleep(Duration::new(1, 0));
}

#[clippy::msrv = "1.63.0"]
async fn issue12273(v: impl Future<Output = ()>) {
    // `.await` desugaring has a call to `IntoFuture::into_future` marked #[stable(since = "1.64.0")],
    // but its stability is ignored
    v.await;
}

fn main() {}
